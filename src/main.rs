mod population;

fn main() {
    let syn = population::synapse::Synapse::new{pop: 3.0, i_node : 0.0, j_node : 0.0, weight : 0.0, delay : 0.0, pre_window : 0.0, post_window : 0.0, delay_trainable : true, weight_trainable : false, partial_delay_training : false, declining_learning_rate : false};
    println!("Hello, world!");
    let x : u64;
    x = 64;
    println!("x is : {}", x);
}
