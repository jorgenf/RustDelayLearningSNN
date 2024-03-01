use crate::connections::InputConnection;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct Input{ 
    pub id : i32,
    pub spikes : Vec<f64>,
    pub spike_times : Vec<f64>,
    pub p : f64,
    pub input_connections : Vec<Arc<Mutex<InputConnection>>>
}


impl Input{
    pub fn new(id : i32, p : f64, spike_times : Vec<f64>) -> Self{
       
        Self {
                id,
                spikes : Vec::new(),
                spike_times,
                p, 
                input_connections : Vec::new()
            }
        }
        
    pub fn add_connection(&mut self, input_connection : Arc<Mutex<InputConnection>>){
        self.input_connections.push(input_connection);
    }

    pub fn update(&mut self, t : f64){
        let mut rng = rand::thread_rng();
        let prob : f64 = rng.gen();
        let mut spike = false;
        if !self.spike_times.is_empty() && self.spike_times.contains(&t){
            self.spikes.push(t);
            spike = true;
        }else if  prob < self.p{
            self.spikes.push(t);
            spike = true;
            }
        if spike{
            for connection in &mut self.input_connections{
                connection.lock().unwrap().add_spike();
        }
    }
    }
    
}