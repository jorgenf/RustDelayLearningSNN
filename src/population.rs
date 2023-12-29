use crate::nodes::Neuron;
use crate::nodes::Input;
use crate::nodes::node_update;
use crate::nodes::Node;
use crate::connections::Synapse;
use crate::connections::InputConnection;
use crate::plotting;

use chrono;
use std::collections::HashMap;
use rand::Error;
use rand::Rng;
use std::time::Instant;
use memory_stats::memory_stats;
use std::fs::{File, create_dir};
use std::io::{BufWriter, Write};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::process::Command;


static mut WEIGHT : f32 = 10.0;
static mut DELAY : f32 = 1.0;

pub struct Population{
    t : f32,
    dt : f32,
    dir : String,
    name : String,
    save_delays : bool,
    neurons : HashMap<i32, Neuron>,
    inputs : HashMap<i32, Input>,
    synapses : HashMap<i32, Synapse>,
    input_connections : HashMap<i32, InputConnection>,
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
            neurons : HashMap::new(),
            inputs : HashMap::new(),
            synapses : HashMap::new(),
            input_connections : HashMap::new(),
            declining_learning_rate : false,
            neuron_id : 0,
            synapse_id : 0, 
            input_id : 0,
            input_connection_id : 0,
        }
    }

    fn update(&mut self){
        for (_id, input) in &mut self.inputs{
            let spike = input.update(self.t, self.dt, 0.0);
            if spike{
                for id in &mut input.input_connections{
                    let connection = self.input_connections.get_mut(id).unwrap();
                    connection.add_spike();
                }
            }
            
        }
        for (_id, neuron )in &mut self.neurons{
            let mut i = 0.0;
            for input_connection in &mut neuron.input_connections{
                i += self.input_connections.get_mut(&input_connection).unwrap().get_spike(self.t);
            }     
            for pre_syn in &mut neuron.pre_synapses{
                let syn = self.synapses.get_mut(pre_syn).unwrap();
                let spikes = syn.get_spikes(self.t);
                i += spikes;
            }           
            let spike : bool = node_update(neuron, self.t, self.dt, i);
            if spike{
                let mut arrival_times: Vec<f32> = Vec::new();
                for pre_syn in &mut neuron.pre_synapses{
                    let mut syn_arrival_times = self.synapses.get_mut(&pre_syn).unwrap().get_avg_arrival_t(self.t);
                    arrival_times.append(&mut syn_arrival_times);
                }
                let length = arrival_times.len();
                let mut sum = 0.0;
                for time in arrival_times{
                    sum += time;
                }
                let avg_spike_arrival_time = sum / length as f32;

                for pre_syn in &mut neuron.pre_synapses{
                    let syn = self.synapses.get_mut(pre_syn).unwrap();
                    syn.f_func(avg_spike_arrival_time);
                }
                for post_syn in &mut neuron.post_synapses{
                    let syn = self.synapses.get_mut(&post_syn).unwrap();
                    syn.add_spike(self.t);
                }
        
        
            }
        }
        for (i,syn) in &mut self.synapses{
            syn.update(self.t);
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
        for (_id, neuron) in &mut self.neurons{
            spikes += neuron.spikes.len();
        }
        println!("Total spikes: {}", spikes);
        println!("Spikes per neuron: {:.1}", spikes/self.neurons.len());
        println!("Max memory usage: {:.1}MB", max_mem/1000000.0);
        

    }

    fn get_spikes(&mut self, id : i32, t : f32)->f32{
        let syn = self.synapses.get_mut(&id).unwrap();
        syn.get_spikes(t)
    }

    fn compile_data(&mut self){
        let _result = create_dir(self.dir.clone());
        
        let mut spike_data : HashMap<String, Vec<f32>> = HashMap::new();
        let mut v_data : HashMap<String, Vec<f32>> = HashMap::new();
        let mut u_data : HashMap<String, Vec<f32>> = HashMap::new();
        for (id, neuron) in &mut self.neurons{
            spike_data.insert(id.to_string(), neuron.get_spikes().to_vec());
            v_data.insert(id.to_string(), neuron.get_v().to_vec());
            u_data.insert(id.to_string(), neuron.get_u().to_vec());
        }
        let mut delay_data : HashMap<String, Vec<(f32,f32)>> = HashMap::new();
        for (id, syn) in &mut self.synapses{
            let key = syn.pre_neuron.to_string() + "_" + &syn.post_neuron.to_string();
            delay_data.insert(key , syn.get_delays().to_owned());
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
            input.add_connection(self.input_connection_id);
            self.neurons.get_mut(&neuron).unwrap().input_connections.push(self.input_connection_id);
            self.create_input_connection(self.input_id, neuron, w);
        }
        self.inputs.insert(self.input_id, input);
        self.input_id += 1;
    }

    pub fn create_synapse(&mut self, neuron_i : i32, neuron_j : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32,  delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool){
        let syn = Synapse::new(self.synapse_id, neuron_i, neuron_j, weight, delay, pre_window, post_window, delay_trainable, weight_trainable, partial_delay_learning, self.declining_learning_rate);
        self.neurons.get_mut(&neuron_i).unwrap().post_synapses.push(syn.id);
        self.neurons.get_mut(&neuron_j).unwrap().pre_synapses.push(syn.id);
        self.synapse_id += 1;
        self.synapses.insert(syn.id, syn);
    }

    fn create_input_connection(&mut self, input_neuron : i32, neuron : i32, weight : f32){
        let input_connection = InputConnection::new(self.input_connection_id, input_neuron, neuron, weight);
        self.input_connection_id += 1;
        self.input_connections.insert(input_connection.id, input_connection);

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
        let neuron = Neuron::new(self.neuron_id);
        self.neuron_id += 1;
        self.neurons.insert(neuron.id, neuron);
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









