use std::{borrow::BorrowMut, collections::HashMap};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{population::Population, connections::{Synapse, InputConnection}, input::Input};


pub struct Neuron{
    pub id : i32,
    a : f32,
    b : f32,
    c : f32,
    d : f32,
    u : f32,
    th : f32,
    v : f32,
    pub pre_synapses : Vec<Rc<RefCell<Synapse>>>,
    pub post_synapses : Vec<Rc<RefCell<Synapse>>>,
    pub input_connections : Vec<Rc<RefCell<InputConnection>>>,
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

    pub fn get_v(&mut self)-> &mut Vec<f32>{
        &mut self.v_hist
    }

    pub fn get_u(&mut self)-> &mut Vec<f32>{
        &mut self.u_hist
    }

    pub fn get_spikes(&mut self)-> &mut Vec<f32>{
        &mut self.spikes
    }

    fn train_synapses(&mut self, t : f32){

        let mut pre_arrival_times = Vec::new();
        let mut pre = Vec::new();
        for syn in self.pre_synapses.iter(){
            let avg_arrival_t = syn.as_ref().borrow_mut().get_avg_arrival_time(t);
            if avg_arrival_t < 0.0{
                pre_arrival_times.push(avg_arrival_t);
                pre.push(syn);
            }else if avg_arrival_t > 0.0{
                syn.as_ref().borrow_mut().train(t, avg_arrival_t);
            }
            
        }
        if !pre_arrival_times.is_empty(){
            let sum : f32 = pre_arrival_times.iter().sum();
            let avg_pre_arrival_time = sum / pre_arrival_times.len() as f32;
            for syn in pre{
                syn.as_ref().borrow_mut().train(t, avg_pre_arrival_time);
            }   
        }
    }

    pub fn update(&mut self, t : f32, dt : f32){
        let mut input = 0.0;
        for syn in self.input_connections.iter(){
            syn.as_ref().borrow_mut().get_spike();
        }
        for syn in self.pre_synapses.iter(){
            input += syn.as_ref().borrow_mut().get_spikes(t);
        }
        let mut i = 0.0;
        for (inp, counter) in self.inputs.iter_mut(){
            *counter -= dt;
            if counter > &mut 0.0{
                i += *inp;
            }
        }
        self.inputs.retain(|x| x.0 != 0.0);
        if input != 0.0{
            self.inputs.push((input, 1.0));
        }
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.v += 0.5 * (0.04 * f32::powi(self.v, 2) + 5.0 * self.v + 140.0 - self.u + i) * dt;
        self.u += self.a * (self.b * self.v - self.u) * dt;
        if self.v >= self.th{
            self.v = self.th;
            self.v = self.c;
            self.u += self.d;
            self.spikes.push(t);
            self.inputs.clear();
            for post in self.post_synapses.iter(){
                post.as_ref().borrow_mut().add_spike(t);
            }
            self.train_synapses(t);

        }
        self.v_hist.push(self.v);
        self.u_hist.push(self.u);
    }
}
