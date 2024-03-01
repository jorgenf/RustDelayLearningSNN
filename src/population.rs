use crate::nodes::Neuron;
use crate::input::Input;
use crate::connections::Synapse;
use crate::connections::InputConnection;
use crate::plotting;
use crate::parameters;
use crate::parameters::Parameters;


use chrono;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use rand::Error;
use rand::Rng;
use std::time::Instant;
use memory_stats::memory_stats;
use std::fs::{File, create_dir};
use std::io::{BufWriter, Write};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use slab_tree;
use std::sync::{Arc, Mutex};
use std::process::Command;


static mut WEIGHT :f64 = 10.0;
static mut DELAY :f64 = 1.0;

pub struct Population{
    t :f64,
    dir : String,
    pub name : String,
    neurons : Vec<Arc<Mutex<Neuron>>>,
    synapses : Vec<Arc<Mutex<Synapse>>>,
    inputs : Vec<Input>,
    neuron_id : i32,
    synapse_id : i32,
    input_id : i32,
    input_connection_id : i32,
    parameters : Option<parameters::Parameters>,
    pub score : f64,
}

impl Population{
    pub fn new(name: String)->Self{
        Self{
            t : 0.0,
            dir : String::from("output_data/"),
            name,
            neurons : Vec::new(),
            synapses : Vec::new(),
            inputs : Vec::new(),
            neuron_id : 0,
            synapse_id : 0, 
            input_id : 0,
            input_connection_id : 0,
            parameters : None,
            score : 0.0
        }
    }

    fn update(&mut self){
        for input in &mut self.inputs{
            input.update(self.t);
        }
        for neuron in &mut self.neurons{
            neuron.lock().unwrap().update(self.t);
        }
    }

    pub fn run(&mut self, duration :f64, store_data : bool, verbose : bool)->(f64, std::option::Option<Parameters>){
        if verbose{
            println!("--- Running Simulation {} ---", self.name);
            println!("Duration: {}ms", duration);
            println!("Number of neurons: {}", self.neurons.len());
            println!("Number of synapses: {}", self.synapses.len());
        }
        
        if self.name == ""{
            self.dir += &chrono::offset::Local::now().to_string().replace(":", ".")[..19].to_string();
        }else {
            self.dir += &self.name;
        }
        
        let start = Instant::now();
        let mut max_mem = 0.0;
        while self.t < duration {
            let progress = (self.t / duration)*100.0;
            if verbose{
                let _ = std::io::stdout().flush().unwrap();
                print!("\rProgress: {:.1}%     Simulated time: {:.1}ms    Elapsed real time: {:.1}s", progress, self.t, start.elapsed().as_secs());
            }
            self.update();
            self.t += 0.1;
            self.t =f64::round(self.t*10.0)/10.0;
            if let Some(usage) = memory_stats() {
                max_mem =f64::max(usage.physical_mem as f64, max_mem);
            }
        }
        if store_data{
            self.compile_data();
        }
        let stop = start.elapsed();
        let mut spikes = 0;
        for neuron in &mut self.neurons{
            spikes += neuron.lock().unwrap().spikes.len();
        }
        self.score = spikes as f64;
        if verbose{
            println!("\n\n--- Simulation {} finished ---", self.name);
            println!("Elapsed time: {}s", stop.as_secs());
            println!("Total spikes: {}", spikes);
            println!("Spikes per neuron: {:.1}", spikes as f64 / self.neurons.len() as f64);
            println!("Max memory usage: {:.1}MB", max_mem/1000000.0);
        }
        (self.score, self.parameters.clone())
    }

    fn compile_data(&mut self){
        let _result = create_dir(self.dir.clone());
        
        let mut spike_data : HashMap<String, Vec<f64>> = HashMap::new();
        let mut v_data : HashMap<String, Vec<f64>> = HashMap::new();
        let mut u_data : HashMap<String, Vec<f64>> = HashMap::new();
        for neuron in &mut self.neurons{
            let spikes = neuron.lock().unwrap().get_spikes().to_vec();

            spike_data.insert(neuron.lock().unwrap().id.to_string(), spikes);
            let v = neuron.lock().unwrap().get_v().to_vec();
            v_data.insert(neuron.lock().unwrap().id.to_string(), v);
            let u = neuron.lock().unwrap().get_u().to_vec();
            u_data.insert(neuron.lock().unwrap().id.to_string(), u);
        }
        let mut delay_data : HashMap<String, Vec<(f64,f64)>> = HashMap::new();
        for syn in self.synapses.iter(){
            let post = &syn.lock().unwrap().post_neuron.to_string();
            let key = syn.lock().unwrap().pre_neuron.to_string() + "_" + post;
            delay_data.insert(key , syn.lock().unwrap().get_delays().to_owned());
        }
        
        let spike_json = serde_json::to_string(&spike_data).unwrap();
        let _result = self.write_json( self.dir.clone() + "/spike_data.json", spike_json);
        
        let v_json = serde_json::to_string(&v_data).unwrap();
        let _result = self.write_json(self.dir.clone() + "/v_data.json", v_json);

        let u_json = serde_json::to_string(&u_data).unwrap();
        let _result = self.write_json(self.dir.clone() + "/u_data.json", u_json);

        let delay_json = serde_json::to_string(&delay_data).unwrap();
        let _result = self.write_json(self.dir.clone() + "/delay_data.json", delay_json);
        //self.plot_data(self.dir.clone());
        
        plotting::plot_delays(&self.dir, self.t,  20.0, delay_data);
        plotting::plot_network(&self.dir);

    }

    fn write_json(&mut self, path : String, data : String)->std::io::Result<()>{
        let mut spike_file = File::create(path)?;
        spike_file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn plot_data(&mut self, path : String){
        Command::new("python").args(["plot_data.py", &path]).output().expect("");
    }

    pub fn get_size(&mut self)->usize{
        self.neurons.len()
    }

    pub fn create_input(&mut self, neurons : Vec<i32>, p :f64, spike_times: Vec<f64>, w :f64){
        let mut input = Input::new(self.input_id, p, spike_times);
        for neuron in neurons{
            let input_connection = self.create_input_connection(self.input_id, neuron, w);
            input.add_connection(Arc::clone(&input_connection));
            //let n = self.neurons.iter().find(|x| x.lock().unwrap().id == neuron).unwrap().lock().unwrap().input_connections.push(Arc::clone(&input_connection));
        }
        self.inputs.push(input);
        self.input_id += 1;
    }

    pub fn create_synapse(&mut self, neuron_i : i32, neuron_j : i32, weight :f64, delay :f64, pre_window :f64, post_window :f64,  delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : f64){
        let syn = Arc::new(Mutex::from(Synapse::new(self.synapse_id, neuron_i, neuron_j, weight, delay, pre_window, post_window, delay_trainable, weight_trainable, partial_delay_learning, declining_learning_rate)));
        self.neurons.iter().find(|x| x.lock().unwrap().id == neuron_i).unwrap().lock().unwrap().post_synapses. push(syn.clone());
        self.neurons.iter().find(|x| x.lock().unwrap().id == neuron_j).unwrap().lock().unwrap().pre_synapses.push(syn.clone());
        self.synapse_id += 1;
        self.synapses.push(syn.clone());
    }

    fn create_input_connection(&mut self, input_neuron : i32, neuron : i32, weight :f64)->Arc<Mutex<InputConnection>>{
        let input_connection = Arc::new(Mutex::from(InputConnection::new(self.input_connection_id, input_neuron, neuron, weight)));
        self.input_connection_id += 1;
        input_connection
    }

    pub fn create_feed_forward(&mut self, layers : i32, nodes_per_layer : i32, p :f64, declining_learning_rate : f64, w_learning_p : f64, w_span : (f64, f64), partial_d : bool, d_learning_p : f64, d_span : (f64, f64), seed : u64){
        self.parameters = Some(parameters::Parameters::new_FeedForward(self.name.clone(), layers * nodes_per_layer, p, layers, declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed));
        let mut rng = rand::thread_rng();
        for _i in 0..layers*nodes_per_layer{
            self.create_neuron();
        }
        for layer in 0..(layers-1){
            for neuron_i in nodes_per_layer * layer..nodes_per_layer * (layer + 1){
                    for neuron_j in nodes_per_layer*(layer + 1)..nodes_per_layer*(layer+2){
                        let prob :f64 = rng.gen();
                        if prob < p{
                            let d :f64;
                            let w :f64;
                            if d_span.0 == d_span.1{
                                d = d_span.0;
                            }else{
                                d =f64::round(rng.gen_range(d_span.0..d_span.1) * 10.0)/10.0;
                            }
                            if w_span.0 == w_span.1{
                                w = w_span.0;
                            }else{
                                w =f64::round(rng.gen_range(w_span.0..w_span.1) * 10.0)/10.0;
                            }
                            self.create_synapse(neuron_i, neuron_j, w, d, -10.0, 7.0, true, false, partial_d, self.parameters.as_ref().unwrap().declining_learning_rate)
                        }
                }
            }
        }
    }

    pub fn create_scale_free(){

    }

    pub fn create_small_world(){

    }

    pub fn create_reservoir(){

    }

    pub fn create_neuron(&mut self){
        let neuron = Arc::new(Mutex::from(Neuron::new(self.neuron_id)));
        self.neuron_id += 1;
        self.neurons.push(neuron);
    }
    
    fn plot_topology(&mut self){

    }

    fn plot_delays(&mut self){

    }

    fn plot_raster(&mut self){

    }
}









