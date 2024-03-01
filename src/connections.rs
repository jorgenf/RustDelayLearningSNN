use std::collections::HashMap;

use crate::population::Population;

static mut MAX_DELAY :f64 = 20.0;


pub struct Synapse{
    pub id : i32,
    pub pre_neuron : i32,
    pub post_neuron : i32,
    weight :f64,
    pub delay :f64,
    pre_window :f64,
    post_window :f64,
    delay_trainable : bool,
    weight_trainable: bool,
    partial_delay_learning : bool,
    declining_learning_rate :f64,
    delay_history : Vec<(f64,f64)>,
    spikes : Vec<f64>, 
    spike_history : Vec<f64>,
    average_arrival_time :f64,  
}


impl Synapse{
    pub fn new(id : i32, pre_neuron: i32, post_neuron : i32, weight :f64, delay :f64, pre_window :f64, post_window :f64, delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : f64)->Self{
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
            delay_history: vec![(0.0, delay)], 
            spikes: Vec::new(),
            spike_history : Vec::new(),
            average_arrival_time : 0.0,
         }
    }

    pub fn add_spike(&mut self, t :f64){
        self.spikes.push(t);
        self.spike_history.insert(0, t);
    }

    pub fn get_spikes(&mut self, t :f64)->f64{
        let mut i = 0.0;
        let mut index = 0;
        for spike in &mut self.spikes{
            if *spike + self.delay == t{
                i += self.weight;
                self.spikes.remove(index);
                break;
            }
            index += 1;
        }
        
        return i;
    }

    pub fn train(&mut self, t :f64, avg_arrival_t :f64){
        if self.delay_trainable{
            if avg_arrival_t < 0.0{
                self.f_func(t, avg_arrival_t);
            }else if avg_arrival_t > 0.0 && !self.partial_delay_learning{
                self.g_func(t, avg_arrival_t);
            }
        }
    }

    pub fn get_avg_arrival_time(&mut self, spike_time :f64)->f64{
        let mut spike_hist = self.spike_history.clone();
        spike_hist.retain(|x| (x + self.delay) - spike_time > self.pre_window && (x + self.delay) - spike_time < 0.0);
        if spike_hist.is_empty(){
            let mut spike_hist = self.spike_history.clone();
            spike_hist.retain(|x| (x + self.delay) - spike_time < self.post_window && (x + self.delay) - spike_time > 0.0);
            let mut sum :f64 = spike_hist.iter().sum();
            sum += self.delay * spike_hist.len() as f64;
            self.average_arrival_time = sum / spike_hist.len() as f64;
            return self.average_arrival_time;
        }else{
            let mut sum :f64 = spike_hist.iter().sum();
            sum += self.delay * spike_hist.len() as f64;
            self.average_arrival_time = -sum / spike_hist.len() as f64;
            return self.average_arrival_time;
        }
    }

    pub fn get_spike_history(&mut self)-> &mut Vec<(f64,f64)>{
        &mut self.delay_history
    }

    pub fn store_delay(&mut self, t :f64){
        self.delay_history.push((t, self.delay));
    }

    fn f_func(&mut self, t :f64, pop_average_arrival_time :f64){
        let delta_t_dist =  pop_average_arrival_time - self.average_arrival_time;
        let delta_d = -3.0 * libm::tanh((delta_t_dist as f64) / 3.0);
        unsafe{
            self.delay +=f64::min(delta_d, MAX_DELAY);
            self.delay =f64::max(self.delay, 0.1);
            self.delay =f64::round(self.delay * 10.0)/10.0;
        }
        self.store_delay(t);
        
    }

    fn g_func(&mut self, t :f64, avg_post :f64){
        let dd = (3.0 / 2.0) * libm::tanh((2.5625 - 0.625 * avg_post) as f64) + 1.5;
        unsafe{
            self.delay += dd as f64;
            self.delay =f64::min(f64::round(self.delay * 10.0)/10.0, MAX_DELAY);
            
        }
        self.store_delay(t);
    }

    pub fn get_delays(&mut self)->&mut Vec<(f64,f64)>{
        &mut self.delay_history
    }

}


pub struct InputConnection{
    pub id : i32,
    input_i : i32,
    neuron_j : i32,
    spike : bool,
    weight :f64,
}

impl InputConnection{
    pub fn new(id : i32, input_i : i32, neuron_j : i32, weight :f64)-> Self{
        Self {id, input_i, neuron_j, spike : false, weight }
    }

    pub fn get_spike(&mut self)->f64{
        if self.spike{
            self.spike = false;
            return self.weight;
        }else{
            return 0.0;
        }
    }

    pub fn add_spike(&mut self){
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