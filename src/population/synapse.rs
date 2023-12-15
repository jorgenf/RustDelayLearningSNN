use std::collections::HashMap;

struct Synapse{
    pop : u32, //populasjon
    i_node : f32, // in-node
    j_node : f32, // ut-node
    weight : f32,
    delay : f32,
    pre_window : f32,
    post_window : f32,
    delay_trainable : bool,
    weight_trainable: bool,
    partial_delay_training : bool,
    declining_learning_rate :bool,
    delay_history : HashMap<String, u32>,
    spikes : Vec<f32>,   


}

impl Synapse{
    fn new(pop: u32, i_node : f32, j_node : f32, weight : f32, delay : f32, pre_window : f32, post_window : f32, delay_trainable : bool, weight_trainable : bool, partial_delay_training : bool, declining_learning_rate : bool) -> Synapse{
        
        Synapse{
            pop, 
            i_node,
            j_node,
            weight, 
            delay, 
            pre_window,
            post_window, 
            delay_trainable,
            weight_trainable,
            partial_delay_training,
            declining_learning_rate,
            delay_history : HashMap::new(),
            spikes : Vec::new()   
        }
        
    }
}
/*

impl Default for Synapse{
    fn default() -> Self {
        Synapse { 
            pop: 1.0, 
            i_node: (), 
            j_node: (), 
            weight: 10.0, 
            delay: 10.0, 
            pre_window: 3.0,
            post_window: 3.0,
            delay_trainable: true,
            weight_trainable: false, 
            partial_delay_training: false,
            declining_learning_rate: false, 
            delay_history: (), 
            spikes: () 
        }
    }
}
 */