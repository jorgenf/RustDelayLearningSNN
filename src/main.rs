mod population;
use population::Population;
mod nodes;
mod connections;
mod input;
mod plotting;
mod evolution;
mod parameters;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use fdg_sim;
use fdg_img;
use slab_tree;



fn main() {
    //let mut pop = Population::new(0.1, String::from("."), String::from("test"), true);
    //let ev = Evolution{generations: 2,individuals: vec![pop]};  
    
        let mut pops : Vec<Population> = evolution::get_individuals();
        evolution::run_evolution(1000.0, 20, pops, 4, 8, 0.0);
}

        

    /*
    println!("Starting");
        let mut pop = Population::new(0.1, String::from("."), String::from("test"), true);
        pop.create_feed_forward(4, 10, 1.0, 1.0, 1.0, 20.0, 20.0, true);
        
        
        for i in 0..10{
            pop.create_input(vec![i], 0.0, vec![i as f64 + 0.1], 20.0);
        }
        
        pop.run(200.0);
     */
    
