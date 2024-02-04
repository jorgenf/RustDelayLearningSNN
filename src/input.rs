use crate::connections::InputConnection;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Input{ 
    pub id : i32,
    pub spikes : Vec<f32>,
    pub spike_times : Vec<f32>,
    pub p : f32,
    pub input_connections : Vec<Rc<RefCell<InputConnection>>>
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
        
    pub fn add_connection(&mut self, input_connection : Rc<RefCell<InputConnection>>){
        self.input_connections.push(input_connection);
    }

    pub fn update(&mut self, t : f32){
        let mut rng = rand::thread_rng();
        let prob : f32 = rng.gen();
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
                connection.borrow_mut().add_spike();
        }
    }
    }
    
}