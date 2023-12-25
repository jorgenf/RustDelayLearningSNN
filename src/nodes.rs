use std::collections::HashMap;
use rand::Rng;

use crate::population::Population;

pub trait Node{
    fn update(&mut self, t : f32, dt : f32, i : f32)->bool;
}

pub struct Neuron{
    pub id : i32,
    a : f32,
    b : f32,
    c : f32,
    d : f32,
    u : f32,
    th : f32,
    v : f32,
    pub pre_synapses : Vec<i32>,
    pub post_synapses : Vec<i32>,
    pub input_connections : Vec<i32>,
    v_hist : Vec<f32>,
    u_hist : Vec<f32>,
    pub spikes : Vec<f32>,
    inputs : Vec<(f32, f32)>
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
                v_hist : Vec::new(),
                u_hist : Vec::new(),
                spikes : Vec::new(),
                inputs : Vec::new()
                }
}
    pub fn get_synapse_ids(&mut self)->(&mut Vec<i32>, &mut Vec<i32>){
        (&mut self.pre_synapses, &mut self.post_synapses)
    }
    pub fn get_v(&mut self)-> &mut Vec<f32>{
        &mut self.v_hist
    }
}

impl Node for Neuron{
    fn update(&mut self, t : f32, dt : f32, input : f32) ->bool{
        let mut i : f32 = input;
        let mut remove_indexes : Vec<i32> = Vec::new();
        for (index, (inp, counter)) in self.inputs.iter_mut().enumerate(){
            *counter -= dt;
            if counter > &mut 0.0{
                i += *inp;
            }else{
                remove_indexes.push(index as i32);
            }
        }
        for ri in remove_indexes{
            self.inputs.remove(ri as usize);
        }
        if input != 0.0{
            self.inputs.push((input, 1.0));
        }
        if self.id == 0{
            println!("Input: {:.?}", self.inputs);
        }
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        if self.id == 0{
            println!("V: {}", self.v)
        }
        self.u += self.a * (self.b * self.v - self.u) * dt;
        if self.v > self.th{
            self.v = self.th;
        }
        self.v_hist.push(self.v);
        self.u_hist.push(self.u);
        if self.v >= self.th{
            self.v = self.c;
            self.u += self.d;
            self.spikes.push(t);
            self.inputs.clear();
            return true;
        }else{
            return false;
        }
    }
}



pub fn node_update<T: Node>(node: &mut T, t : f32, dt : f32, i : f32)-> bool {
    node.update(t, dt, i)
}

pub struct Input{ 
    pub id : i32,
    pub spikes : Vec<f32>,
    pub spike_times : Vec<f32>,
    pub p : f32,
    pub input_connections : Vec<i32>
}


impl Input{
    pub fn new(id : i32, p : f32, spike_times : Vec<f32>) -> Self{
       
        Self {
                id,
                spikes : Vec::new(),
                spike_times,
                p, 
                input_connections : Vec::new()
            }
  
        }

        pub fn add_connection(&mut self, connection : i32){
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