use std::collections::HashMap;

static mut MAX_DELAY : f32 = 20.0;


pub struct Synapse{
    pub id : i32,
    pub pre_neuron : i32,
    post_neuron : i32,
    weight : f32,
    pub delay : f32,
    pre_window : f32,
    post_window : f32,
    delay_trainable : bool,
    weight_trainable: bool,
    partial_delay_learning : bool,
    declining_learning_rate :bool,
    delay_history : HashMap<String,f32>,
    spikes : Vec<f32>, 
    spike_history : Vec<f32>,
    average_arrival_time : f32,  
}


impl Synapse{
    pub fn new(id : i32, pre_neuron: i32, post_neuron : i32, weight : f32, delay : f32, pre_window : f32, post_window : f32, delay_trainable : bool, weight_trainable : bool, partial_delay_learning : bool, declining_learning_rate : bool)->Self{
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
            average_arrival_time : 0.0
         }
    }

    pub fn add_spike(&mut self, t : f32){
        self.spikes.push(t);
        self.spike_history.insert(0, t);
    }

    pub fn get_spikes(&mut self, t : f32)->f32{
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

    pub fn get_spike_history(&mut self)-> &mut HashMap<String, f32>{
        &mut self.delay_history
    }

    pub fn get_avg_arrival_t(&mut self, spike_time : f32)-> Vec<f32>{
        let mut pre : Vec<f32> = Vec::new();
        if self.delay_trainable{
            
            let mut post : Vec<f32> = Vec::new();
            for syn_spike in &mut self.spike_history{
                let t_dist = (*syn_spike + self.delay) - spike_time; 
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
            if pre.is_empty(){
                self.g_func(sum_post/ post_len as f32);
            }
        }
        let mut sum = 0.0;
        for spike in &pre{
            sum += spike;
        }
        self.average_arrival_time = sum / pre.len() as f32;
        pre  
        }

    pub fn f_func(&mut self, pop_average_arrival_time : f32){
        let delta_t_dist =  self.average_arrival_time - pop_average_arrival_time;
        let delta_d = -3.0 * libm::tanh((delta_t_dist as f64) / 3.0);
        unsafe{
            self.delay += f32::min(delta_d as f32, MAX_DELAY);
            self.delay = f32::max(self.delay, 0.1);
            self.delay = f32::round(self.delay * 10.0)/10.0;
        }
        //self.delay_history.insert(t, self.delay);
        
    }

    pub fn g_func(&mut self, avg_post : f32){
        let dd = (3.0 / 2.0) * libm::tanh((2.5625 - 0.625 * avg_post) as f64) + 1.5;
        unsafe{
            self.delay += dd as f32;
            self.delay = f32::min(f32::round(self.delay * 10.0)/10.0, MAX_DELAY);
            
        }
    }

}


pub struct InputConnection{
    pub id : i32,
    input_i : i32,
    neuron_j : i32,
    spike : bool,
    weight : f32,
}

impl InputConnection{
    pub fn new(id : i32, input_i : i32, neuron_j : i32, weight : f32)-> Self{
        Self {id, input_i, neuron_j, spike : false, weight }
    }

    pub fn get_spike(&mut self, t : f32)-> f32{
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