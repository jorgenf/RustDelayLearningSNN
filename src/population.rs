use std::{collections::HashMap, ffi::NulError, ops::Deref};
use rand::Rng;
use std::io::Write;
use std::time::{Duration, Instant};
use std::cmp;


static mut WEIGHT : f32 = 10.0;
static mut DELAY : f32 = 1.0;

pub struct Population{
    t : f32,
    dt : f32,
    dir : String,
    name : String,
    save_delays : bool,
    neurons : HashMap<i32, Neuron>,
    inputs : HashMap<i32, Neuron>,
    synapses : HashMap<i32, Synapse>,
    declining_learning_rate : bool,
    neuron_id : i32,
    synapse_id : i32
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
            declining_learning_rate : false,
            neuron_id : 0,
            synapse_id : 0
        }
    }

    fn update(&mut self){
        for (id, neuron )in &mut self.neurons{
            let mut i = 0.0;
            let mut contributing_synapses : Vec<i32> = Vec::new();
            for pre_syn in &mut neuron.pre_synapses{
                let syn = self.synapses.get_mut(pre_syn).unwrap();
                let spikes = syn.get_spikes(self.t);
                if spikes > 0.0{
                    contributing_synapses.push(syn.id);
                }
                i += spikes;
            }
            let spike : bool = node_update(neuron, self.t, self.dt, i);
            if spike{
                for pre_syn in &mut neuron.pre_synapses{
                    let syn = self.synapses.get_mut(&pre_syn).unwrap();
                    if syn.weight_trainable{
                        for cont_syn in &contributing_synapses{  

                        }
                    }
                }
                for post_syn in &mut neuron.post_synapses{
                    let syn = self.synapses.get_mut(&post_syn).unwrap();
                    syn.add_spike(self.t);
                }
            }
        }
    }

    pub fn run(&mut self, duration : f32){
        println!("--- Running Simulation {} ---", self.name);
        println!("Duration: {}", duration);
        println!("Number of neurons: {}", self.neurons.len());
        println!("Number of synapses: {}", self.synapses.len());
        let start = Instant::now();
        while self.t < duration {
            self.update();
            let progress = (self.t / duration)*100.0;
            let _ = std::io::stdout().flush().unwrap();
            print!("\rProgress: {:.1}%     Time: {:.1}ms", progress, self.t);
            self.t += self.dt;
        }
        let stop = start.elapsed();
        println!("\n\n--- Simulation finished ---");
        println!("Simulation time: {}sec", stop.as_millis()/1000);
        let mut spikes = 0;
        for (id, neuron) in &mut self.neurons{
            spikes += neuron.spikes.len();
        }
        println!("Total spikes: {}", spikes);
        println!("Spikes per neuron: {:.1}", spikes/self.neurons.len());
        
    }

    fn get_spikes(&mut self, id : i32, t : f32)->f32{
        let syn = self.synapses.get_mut(&id).unwrap();
        syn.get_spikes(t)
    }

    fn add_spikes(&mut self, id : i32){

    }

    pub fn get_size(&mut self)->usize{
        self.neurons.len()
    }

    fn create_input(&mut self){

    }

    fn create_synapse(&mut self, neuron_i : i32, neuron_j : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32,  delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool){
        let syn = Synapse::new(self.synapse_id, weight, delay, pre_window, post_window, delay_trainable, weight_trainable, partial_delay_learning, self.declining_learning_rate);
        self.neurons.get_mut(&neuron_i).unwrap().post_synapses.push(syn.id);
        self.neurons.get_mut(&neuron_j).unwrap().pre_synapses.push(syn.id);
        self.synapse_id += 1;
        self.synapses.insert(syn.id, syn);
    }

    pub fn create_feed_forward(&mut self, layers : i32, nodes_per_layer : i32, p : f32, d_min : f32, d_max : f32, w_min : f32, w_max : f32){
        let mut rng = rand::thread_rng();
        let prob : f32 = rng.gen();
        for _i in 0..layers*nodes_per_layer{
            self.add_neuron();
        }
        for layer in 0..(layers-1){
            for neuron_i in nodes_per_layer * layer..nodes_per_layer * (layer + 1){
                    for neuron_j in nodes_per_layer*(layer + 1)..nodes_per_layer*(layer+2){
                        if prob < p{
                            let w = rng.gen_range(w_min..w_max);
                            let d = rng.gen_range(d_min..d_max);
                            self.create_synapse(neuron_i, neuron_j, w, d, 3.0, 3.0, true, false, false, self.declining_learning_rate)
                        }
                }
            }
        }
    }

    fn add_neuron(&mut self){
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


pub trait Node{
    fn update(&mut self, t : f32, dt : f32, i : f32)->bool;
}

pub struct Neuron{
    id : i32,
    a : f32,
    b : f32,
    c : f32,
    d : f32,
    u : f32,
    th : f32,
    v : f32,
    pre_synapses : Vec<i32>,
    post_synapses : Vec<i32>,
    v_hist : HashMap<String, f32>,
    u_hist : HashMap<String, f32>,
    spikes : Vec<f32>
}

impl Neuron{
    pub fn new(id : i32)->Self{
            Self {
                id,
                a : 0.02,
                b : 0.2,
                c : -65.0,
                d : 8.0,
                u : -14.0,
                th : 30.0,
                v : -70.0,
                pre_synapses : Vec::new(),
                post_synapses : Vec::new(),
                v_hist : HashMap::new(),
                u_hist : HashMap::new(),
                spikes : Vec::new(),
                }
}
    pub fn get_synapse_ids(&mut self)->(&mut Vec<i32>, &mut Vec<i32>){
        (&mut self.pre_synapses, &mut self.post_synapses)
    }
}

impl Node for Neuron{
    fn update(&mut self, t : f32, dt : f32, i : f32) ->bool{
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.u += self.a * (self.b * self.v - self.u) * dt;
        if self.v > self.th{
            self.v = self.th;
        }
        self.v_hist.insert(t.to_string(), self.v);
        self.u_hist.insert(t.to_string(), self.u);
        if self.v >= self.th{
            self.v = self.c;
            self.u += self.d;
            self.spikes.push(t);
            return true;
        }else{
            return false;
        }
    }
}

pub struct Input{
    pub id : i32,
    pub spikes : Vec<f32>,
    pub spike_times : Vec<f32>,
    pub p : f32,
    pub post_synapse : Vec<Synapse>,
    pub poly_pattern : PolychronousPattern
}


impl Input{
    pub fn new(&mut self, id : i32, p : f32, post_synapse : Vec<Synapse>) -> Self{
       
        Self {
                id,
                spikes : Vec::new(),
                spike_times : Vec::new(),
                p, 
                post_synapse : Vec::new(),
                poly_pattern : PolychronousPattern{t: 1.0, id: 1}
            }
  
        }
        
}



impl Node for Input{
    fn update(&mut self, t : f32, dt : f32, i : f32)->bool {
        let mut rng = rand::thread_rng();
        let prob : f32 = rng.gen();
        if !self.spike_times.is_empty() && self.spike_times.contains(&t){
            for syn in &mut self.post_synapse{
                syn.add_spike(t);
                self.spikes.push(t);
                return true;
            }
        }else if  prob < self.p{
            for syn in &mut self.post_synapse{
                syn.add_spike(t);
                self.spikes.push(t);
                return true;
            }
        }
        false
    }
}

pub fn node_update<T: Node>(node: &mut T, t : f32, dt : f32, i : f32)-> bool {
    node.update(t, dt, i)
}



struct Synapse{
    id : i32,
    weight : f32,
    delay : f32,
    pre_window : f32,
    post_window : f32,
    delay_trainable : bool,
    weight_trainable: bool,
    partial_delay_learning : bool,
    declining_learning_rate :bool,
    delay_history : HashMap<String, u32>,
    spikes : Vec<f32>,   
}


impl Synapse{
    fn new(id : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32, delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool)->Self{
        Self {
            id,
            weight, 
            delay, 
            pre_window, 
            post_window, 
            delay_trainable, 
            weight_trainable,
            partial_delay_learning, 
            declining_learning_rate, 
            delay_history: HashMap::new(), 
            spikes: Vec::new()
         }
    }

    fn add_spike(&mut self, t : f32){
        self.spikes.push(t);
    }

    fn get_spikes(&mut self, t : f32)->f32{
        let mut i = 0.0;
        let mut index = 0;
        for spike in &mut self.spikes{
            if *spike + self.delay == t{
                i += self.weight;
                break;
                
            }
            index += 1;
        }
        //self.spikes.remove(index);
        return i;
    }

    fn f_func(&self){
        if self.delay_trainable{

        }

    }

    fn g_func(&self){
       if self.delay_trainable && !self.partial_delay_learning{
            
        } 
    }

}

/*

impl Default for Synapse{
    fn default() -> Self {
        Synapse { 
            i_node: Neuron{}, 
            j_node: Neuron{}, 
            weight: 10.0, 
            delay: 10.0, 
            pre_window: 3.0,
            post_window: 3.0,
            delay_trainable: true,
            weight_trainable: false, 
            partial_delay_learning: false,
            declining_learning_rate: false, 
            delay_history: HashMap::new(), 
            spikes: Vec::new() 
        }
    }
}
 */
struct PolychronousPattern{
    t : f32,
    id : i32,
    //down_patterns: Vec<PolychronousPattern>
}






