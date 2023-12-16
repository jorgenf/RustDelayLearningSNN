use std::collections::HashMap;
use rand::Rng;

static mut ID : i32 = -1;

pub struct Population{
    t : f32,
    synapses : Vec<Synapse>
}


pub trait Node{
    fn update(&mut self);
}

pub struct Neuron{

}

impl Neuron{

}

impl Node for Neuron{
    fn update(&mut self) {
        println!("update for neuron")
    }

  
}

pub struct Input{
    pub population : Population,
    pub id : i32,
    pub spikes : Vec<f32>,
    pub spike_times : Vec<f32>,
    pub p : f32,
    pub post_synapse : Vec<Synapse>,
    pub poly_pattern : PolychronousPattern
}


impl Input{
    pub fn new(&mut self, population : Population, p : f32, post_synapse : Vec<Synapse>) -> Self{
       
        unsafe{
            let ret = Self {
                population,
                id : ID,
                spikes : Vec::new(),
                spike_times : Vec::new(),
                p, 
                post_synapse : Vec::new(),
                poly_pattern : PolychronousPattern{t: 1.0, id: 1}
            };
                ID += 1;
                return ret;
        }
        }
        
}



impl Node for Input{
    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let prob : f32 = rng.gen();
        if !self.spike_times.is_empty() && self.spike_times.contains(&self.population.t){
            for syn in &self.post_synapse{
                syn.add_spike();
                self.spikes.push(self.population.t)
            }
        }else if  prob < self.p{
            for syn in &self.post_synapse{
                syn.add_spike();
                self.spikes.push(self.population.t)
            }
        }
    }
}

pub fn node_update<T: Node>(t: &mut T) {
    t.update();
}



pub struct Synapse{
    pub pop : u32, //populasjon
    pub i_node : Neuron, // in-node
    pub j_node : Neuron, // ut-node
    pub weight : f32,
    pub delay : f32,
    pub pre_window : f32,
    pub post_window : f32,
    pub delay_trainable : bool,
    pub weight_trainable: bool,
    pub partial_delay_training : bool,
    pub declining_learning_rate :bool,
    pub delay_history : HashMap<String, u32>,
    pub spikes : Vec<f32>,   


}


impl Synapse{
    fn add_spike(&self){

    }

    fn get_spike(&self){

    }

    fn f_func(&self){

    }

    fn g_func(&self){
        
    }

}

impl Default for Synapse{
    fn default() -> Self {
        Synapse { 
            pop: 1, 
            i_node: Neuron{}, 
            j_node: Neuron{}, 
            weight: 10.0, 
            delay: 10.0, 
            pre_window: 3.0,
            post_window: 3.0,
            delay_trainable: true,
            weight_trainable: false, 
            partial_delay_training: false,
            declining_learning_rate: false, 
            delay_history: HashMap::new(), 
            spikes: Vec::new() 
        }
    }
}

struct PolychronousPattern{
    t : f32,
    id : i32,
    //down_patterns: Vec<PolychronousPattern>
}






