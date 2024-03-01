use std::{borrow::BorrowMut, collections::HashMap};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::{population::Population, connections::{Synapse, InputConnection}, input::Input};


pub struct Neuron{
    pub id : i32,
    a : f64,
    b : f64,
    c : f64,
    d : f64,
    u : f64,
    th : f64,
    v : f64,
    pub pre_synapses : Vec<Arc<Mutex<Synapse>>>,
    pub post_synapses : Vec<Arc<Mutex<Synapse>>>,
    pub input_connections : Vec<Arc<Mutex<InputConnection>>>,
    v_hist : Vec<f64>,
    u_hist : Vec<f64>,
    pub spikes : Vec<f64>,
    inputs : Vec<(f64, f64)>
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

    pub fn get_v(&mut self)-> &mut Vec<f64>{
        &mut self.v_hist
    }

    pub fn get_u(&mut self)-> &mut Vec<f64>{
        &mut self.u_hist
    }

    pub fn get_spikes(&mut self)-> &mut Vec<f64>{
        &mut self.spikes
    }

    fn train_synapses(&mut self, t : f64){

        let mut pre_arrival_times = Vec::new();
        let mut pre = Vec::new();
        for syn in self.pre_synapses.iter(){
            let avg_arrival_t = syn.lock().unwrap().get_avg_arrival_time(t);
            if avg_arrival_t < 0.0{
                pre_arrival_times.push(avg_arrival_t);
                pre.push(syn);
            }else if avg_arrival_t > 0.0{
                syn.lock().unwrap().train(t, avg_arrival_t);
            }
            
        }
        if !pre_arrival_times.is_empty(){
            let sum : f64 = pre_arrival_times.iter().sum();
            let avg_pre_arrival_time = sum / pre_arrival_times.len() as f64;
            for syn in pre{
                syn.lock().unwrap().train(t, avg_pre_arrival_time);
            }   
        }
    }

    pub fn update(&mut self, t : f64){
        let dt = 0.1;
        let mut input = 0.0;
        for syn in self.input_connections.iter(){
            input += syn.lock().unwrap().get_spike();
        }
        for syn in self.pre_synapses.iter(){
            input += syn.lock().unwrap().get_spikes(t);
        }
        let mut i = 0.0;
        if input != 0.0{
            self.inputs.push((input, 1.0));
        }
        for (inp, counter) in self.inputs.iter_mut(){
            *counter -= dt;
            if counter > &mut 0.0{
                i += *inp;
            }
        }
        self.inputs.retain(|x| x.0 != 0.0);
        
        self.v += 0.5 * (0.04 * f64::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.v += 0.5 * (0.04 * f64::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.u += self.a * (self.b * self.v - self.u) * dt;
        self.v_hist.push(self.v);
        self.u_hist.push(self.u);
        if self.v >= self.th{
            self.v = self.th;
            self.v = self.c;
            self.u += self.d;
            self.spikes.push(t);
            self.inputs.clear();
            for post in self.post_synapses.iter(){
                post.lock().unwrap().add_spike(t);
            }
            self.train_synapses(t);
        }
        
    }
}
