use crate::nodes::Neuron;
use crate::input::Input;
use crate::connections::Synapse;
use crate::connections::InputConnection;
use crate::plotting;

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
use std::cell::RefCell;
use std::rc::Rc;

use std::process::Command;


static mut WEIGHT : f32 = 10.0;
static mut DELAY : f32 = 1.0;

pub struct Population{
    t : f32,
    dt : f32,
    dir : String,
    name : String,
    save_delays : bool,
    neurons : Vec<Rc<RefCell<Neuron>>>,
    synapses : Vec<Rc<RefCell<Synapse>>>,
    inputs : Vec<Input>,
    declining_learning_rate : bool,
    neuron_id : i32,
    synapse_id : i32,
    input_id : i32,
    input_connection_id : i32,
}

impl Population{
    pub fn new(dt : f32, dir : String, name: String ,  save_delays : bool)->Self{
        Self{
            t : 0.0,
            dt,
            dir,
            name,
            save_delays,
            neurons : Vec::new(),
            synapses : Vec::new(),
            inputs : Vec::new(),
            declining_learning_rate : false,
            neuron_id : 0,
            synapse_id : 0, 
            input_id : 0,
            input_connection_id : 0,
        }
    }

    fn update(&mut self){
        for input in &mut self.inputs{
            input.update(self.t);
        }
        for neuron in &mut self.neurons{
            neuron.as_ref().borrow_mut().update(self.t, self.dt);
        }
    }

    pub fn run(&mut self, duration : f32){
        println!("--- Running Simulation {} ---", self.name);
        println!("Duration: {}ms", duration);
        println!("Number of neurons: {}", self.neurons.len());
        println!("Number of synapses: {}", self.synapses.len());
        self.dir = "output_data/".to_owned() + &chrono::offset::Local::now().to_string().replace(":", ".")[..19].to_string();
        let start = Instant::now();
        let mut max_mem = 0.0;
        while self.t < duration {
            let progress = (self.t / duration)*100.0;
            let _ = std::io::stdout().flush().unwrap();
            print!("\rProgress: {:.1}%     Simulated time: {:.1}ms    Elapsed real time: {:.1}s", progress, self.t, start.elapsed().as_secs());
            self.update();
            self.t += self.dt;
            self.t = f32::round(self.t*10.0)/10.0;
            if let Some(usage) = memory_stats() {
                max_mem = f32::max(usage.physical_mem as f32, max_mem);
            }
        }
        self.compile_data();
        let stop = start.elapsed();
        println!("\n\n--- Simulation finished ---");
        println!("Elapsed time: {}s", stop.as_secs());
        let mut spikes = 0;
        for neuron in &mut self.neurons{
            spikes += neuron.as_ref().borrow_mut().spikes.len();
        }
        for synapse in self.synapses.iter(){
            let pre = synapse.as_ref().borrow_mut().pre_neuron;
            let post = synapse.as_ref().borrow_mut().post_neuron;
            let delay = synapse.as_ref().borrow_mut().delay;
            print!("Synapse: {}-{} delay: {}\n", pre, post, delay);
        }
        println!("Total spikes: {}", spikes);
        println!("Spikes per neuron: {:.1}", spikes/self.neurons.len());
        println!("Max memory usage: {:.1}MB", max_mem/1000000.0);
        

    }

    fn compile_data(&mut self){
        let _result = create_dir(self.dir.clone());
        
        let mut spike_data : HashMap<String, Vec<f32>> = HashMap::new();
        let mut v_data : HashMap<String, Vec<f32>> = HashMap::new();
        let mut u_data : HashMap<String, Vec<f32>> = HashMap::new();
        for neuron in &mut self.neurons{
            let spikes = neuron.as_ref().borrow_mut().get_spikes().to_vec();

            spike_data.insert(neuron.as_ref().borrow_mut().id.to_string(), spikes);
            let v = neuron.as_ref().borrow_mut().get_v().to_vec();
            v_data.insert(neuron.as_ref().borrow_mut().id.to_string(), v);
            let u = neuron.as_ref().borrow_mut().get_u().to_vec();
            u_data.insert(neuron.as_ref().borrow_mut().id.to_string(), u);
        }
        let mut delay_data : HashMap<String, Vec<(f32,f32)>> = HashMap::new();
        for syn in self.synapses.iter(){
            let post = &syn.as_ref().borrow_mut().post_neuron.to_string();
            let key = syn.as_ref().borrow_mut().pre_neuron.to_string() + "_" + post;
            delay_data.insert(key , syn.as_ref().borrow_mut().get_delays().to_owned());
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

    pub fn create_input(&mut self, neurons : Vec<i32>, p : f32, spike_times: Vec<f32>, w : f32){
        let mut input = Input::new(self.input_id, p, spike_times);
        for neuron in neurons{
            let input_connection = self.create_input_connection(self.input_id, neuron, w);
            input.add_connection(Rc::clone(&input_connection));
            let n = self.neurons.iter().find(|x| x.as_ref().borrow_mut().id == neuron).unwrap().as_ref().borrow_mut().input_connections.push(Rc::clone(&input_connection));
        }
        self.inputs.push(input);
        self.input_id += 1;
    }

    pub fn create_synapse(&mut self, neuron_i : i32, neuron_j : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32,  delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool){
        let syn = Rc::new(RefCell::new(Synapse::new(self.synapse_id, neuron_i, neuron_j, weight, delay, pre_window, post_window, delay_trainable, weight_trainable, partial_delay_learning, self.declining_learning_rate)));
        self.neurons.iter().find(|x| x.as_ref().borrow_mut().id == neuron_i).unwrap().as_ref().borrow_mut().post_synapses. push(syn.clone());
        self.neurons.iter().find(|x| x.as_ref().borrow_mut().id == neuron_j).unwrap().as_ref().borrow_mut().pre_synapses.push(syn.clone());
        self.synapse_id += 1;
        self.synapses.push(syn.clone());
    }

    fn create_input_connection(&mut self, input_neuron : i32, neuron : i32, weight : f32)->Rc<RefCell<InputConnection>>{
        let input_connection = Rc::new(RefCell::new(InputConnection::new(self.input_connection_id, input_neuron, neuron, weight)));
        self.input_connection_id += 1;
        input_connection
    }

    pub fn create_feed_forward(&mut self, layers : i32, nodes_per_layer : i32, p : f32, d_min : f32, d_max : f32, w_min : f32, w_max : f32){
        let mut rng = rand::thread_rng();
        for _i in 0..layers*nodes_per_layer{
            self.create_neuron();
        }
        for layer in 0..(layers-1){
            for neuron_i in nodes_per_layer * layer..nodes_per_layer * (layer + 1){
                    for neuron_j in nodes_per_layer*(layer + 1)..nodes_per_layer*(layer+2){
                        let prob : f32 = rng.gen();
                        if prob < p{
                            let d : f32;
                            let w : f32;
                            if d_min == d_max{
                                d = d_min;
                            }else{
                                d = f32::round(rng.gen_range(d_min..d_max) * 10.0)/10.0;
                            }
                            if w_min == w_max{
                                w = w_min;
                            }else{
                                w = f32::round(rng.gen_range(w_min..w_max) * 10.0)/10.0;
                            }
                            self.create_synapse(neuron_i, neuron_j, w, d, 7.0, 7.0, true, false, false, self.declining_learning_rate)
                        }
                }
            }
        }
    }

    pub fn create_neuron(&mut self){
        let neuron = Rc::new(RefCell::new(Neuron::new(self.neuron_id)));
        self.neuron_id += 1;
        self.neurons.push(neuron);
    }

    fn create_reservoir(&mut self){

    }
    
    fn plot_topology(&mut self){

    }

    fn plot_delays(&mut self){

    }

    fn plot_raster(&mut self){

    }
}









