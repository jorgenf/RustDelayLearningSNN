use std::mem::ManuallyDrop;
use std::{collections::HashMap, ffi::NulError, ops::Deref};
use rand::Rng;
use std::io::Write;
use std::time::{Duration, Instant};
use std::cmp;
use libm::tanh;
use round::round;

static mut MAX_DELAY : f32 = 20.0;
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
        println!("\n");
        for (_id, input) in &mut self.inputs{
            let spike = input.update(self.t, self.dt, 0.0);
            if spike{
                for id in &mut input.input_connections{
                    let connection = self.input_connections.get_mut(id).unwrap();
                    connection.add_spike()
                }
            }
            
        }
        for (_id, neuron )in &mut self.neurons{
            println!("Neuron ID: {}", _id);
            let mut i = 0.0;
            for input_connection in &mut neuron.input_connections{
                i += self.input_connections.get_mut(&input_connection).unwrap().get_spike(self.t);
            }
            for pre_syn in &mut neuron.pre_synapses{
                let syn = self.synapses.get_mut(pre_syn).unwrap();
                let spikes = syn.get_spikes(self.t);
                if spikes != 0.0{
                    println!("Incoming spikes");
                }
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
                    println!("Adding spike from neuron {} to synapse {}, post neuron {}", neuron.id, syn.id, syn.post_neuron);
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
            println!("T: {}", self.t);
            self.update();
            let progress = (self.t / duration)*100.0;
            let _ = std::io::stdout().flush().unwrap();
            print!("\rProgress: {:.1}%     Time: {:.1}ms", progress, self.t);
            self.t += self.dt;
            self.t = f32::round(self.t*10.0)/10.0;
        }
        let stop = start.elapsed();
        println!("\n\n--- Simulation finished ---");
        println!("Simulation time: {}sec", stop.as_millis()/1000);
        let mut spikes = 0;
        for (_id, neuron) in &mut self.neurons{
            spikes += neuron.spikes.len();
        }
        println!("Total spikes: {}", spikes);
        println!("Spikes per neuron: {:.1}", spikes/self.neurons.len());
        for (id, neuron) in &mut self.neurons{
            println!("ID: {} Spikes: {}", id, neuron.spikes.len());
        }
        
    }

    fn get_spikes(&mut self, id : i32, t : f32)->f32{
        let syn = self.synapses.get_mut(&id).unwrap();
        syn.get_spikes(t)
    }


    pub fn get_size(&mut self)->usize{
        self.neurons.len()
    }

    pub fn create_input(&mut self, neurons : Vec<i32>, p : f32, w : f32){
        let mut input = Input::new(self.input_id, p);
        for neuron in neurons{
            input.add_connection(self.input_connection_id);
            self.neurons.get_mut(&neuron).unwrap().input_connections.push(self.input_connection_id);
            self.create_input_connection(self.input_id, neuron, w);
        }
        self.inputs.insert(self.input_id, input);
        self.input_id += 1;
    }

    fn create_synapse(&mut self, neuron_i : i32, neuron_j : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32,  delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool){
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
        let prob : f32 = rng.gen();
        for _i in 0..layers*nodes_per_layer{
            self.add_neuron();
        }
        for layer in 0..(layers-1){
            println!("Layer {}", layer);
            for neuron_i in nodes_per_layer * layer..nodes_per_layer * (layer + 1){
                    for neuron_j in nodes_per_layer*(layer + 1)..nodes_per_layer*(layer+2){
                        if prob < p{
                            let w = f32::round(rng.gen_range(w_min..w_max) * 10.0)/10.0;
                            let d = f32::round(rng.gen_range(d_min..d_max) * 10.0)/10.0;
                            println!("DELAY: {}", d);
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
    input_connections : Vec<i32>,
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
                input_connections : Vec::new(),
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



pub fn node_update<T: Node>(node: &mut T, t : f32, dt : f32, i : f32)-> bool {
    node.update(t, dt, i)
}



struct Synapse{
    id : i32,
    pre_neuron : i32,
    post_neuron : i32,
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
    spike_history : Vec<f32>  
}


impl Synapse{
    fn new(id : i32, pre_neuron: i32, post_neuron : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32, delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool)->Self{
        Self {
            id,
            pre_neuron,
            post_neuron,
            weight, 
            delay, 
            pre_window, 
            post_window, 
            delay_trainable, 
            weight_trainable,
            partial_delay_learning, 
            declining_learning_rate, 
            delay_history: HashMap::new(), 
            spikes: Vec::new(),
            spike_history : Vec::new(),
         }
    }

    fn add_spike(&mut self, t : f32){
        self.spikes.push(t);
        self.spike_history.insert(0, t);
    }

    fn get_spikes(&mut self, t : f32)->f32{
        println!("Spikes in synapse: {}", self.spikes.len());
        let mut i = 0.0;
        let mut index = 0;
        for spike in &mut self.spikes{
            println!("Spike: {} delay {}  t {} w {}", spike, self.delay, t, self.weight);
            if *spike + self.delay == t{
                println!("SPIKE!");
                i += self.weight;
                self.spikes.remove(index);
                break;
            }
            index += 1;
        }
        
        return i;
    }

    fn get_avg_arrival_t(&mut self, spike_time : f32)-> Vec<f32>{
        let mut pre : Vec<f32> = Vec::new();
        if self.delay_trainable{
            
            let mut post : Vec<f32> = Vec::new();
            for syn_spike in &mut self.spike_history{
                let t_dist = spike_time - (*syn_spike + self.delay); 
                if t_dist > self.pre_window{
                    break;
                }else if  t_dist < 0.0 && t_dist > self.post_window{
                    post.push(t_dist);
                }else if t_dist != 0.0{
                    pre.push(t_dist);
                }
            }
            let post_len = post.len();
            let mut sum_post = 0.0;
            for i in post{
                sum_post += i;
            }
            self.g_func(sum_post/ post_len as f32);
        }
        pre  
        }

    fn f_func(&mut self, delta_t_dist : f32){
        let delta_d = -3.0 * libm::tanh((delta_t_dist as f64) / 3.0);
        unsafe{
            self.delay += f32::min(delta_d as f32, MAX_DELAY);
            self.delay = f32::round(self.delay * 10.0)/10.0;
        }
        
    }

    fn g_func(&mut self, avg_post : f32){
        let dd = (3.0 / 2.0) * libm::tanh((2.5625 - 0.625 * avg_post) as f64) + 1.5;
        unsafe{
            self.delay += f32::min(dd as f32, MAX_DELAY);
            self.delay = f32::round(self.delay * 10.0)/10.0;
        }   
    }

}

pub struct Input{
    pub id : i32,
    pub spikes : Vec<f32>,
    pub spike_times : Vec<f32>,
    pub p : f32,
    pub input_connections : Vec<i32>,
    pub poly_pattern : PolychronousPattern
}


impl Input{
    pub fn new(id : i32, p : f32) -> Self{
       
        Self {
                id,
                spikes : Vec::new(),
                spike_times : Vec::new(),
                p, 
                input_connections : Vec::new(),
                poly_pattern : PolychronousPattern{t: 1.0, id: 1}
            }
  
        }

        fn add_connection(&mut self, connection : i32){
            self.input_connections.push(connection);
        }
        
}


impl Node for Input{
    fn update(&mut self, t : f32, dt : f32, i : f32)->bool {
        let mut rng = rand::thread_rng();
        let prob : f32 = rng.gen();
        if !self.spike_times.is_empty() && self.spike_times.contains(&t){
            self.spikes.push(t);
            return true;
            
        }else if  prob < self.p{
            self.spikes.push(t);
            return true;
            }
        false
    }
}

struct InputConnection{
    id : i32,
    input_i : i32,
    neuron_j : i32,
    spike : bool,
    weight : f32,
}

impl InputConnection{
    fn new(id : i32, input_i : i32, neuron_j : i32, weight : f32)-> Self{
        Self {id, input_i, neuron_j, spike : false, weight }
    }

    fn get_spike(&mut self, t : f32)-> f32{
        if self.spike{
            return self.weight;
        }else{
            return 0.0;
        }
    }

    fn add_spike(&mut self){
        self.spike = true;
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






