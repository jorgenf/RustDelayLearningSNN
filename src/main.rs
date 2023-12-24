mod population;
use population::Population;
mod nodes;
mod connections;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;



fn main() {

    println!("Starting");
    
    /*

        
        let pool = ThreadPool::new(6);
        for i in 4..8 {
            
            let mut pop = Population::new(0.1, String::from("."), String::from("test"), false);
            pop.create_feed_forward(i, 100, 1.0, 1.0, 10.0, 1.0, 10.0);
            pool.execute(move|| {
                
                pop.run(1000.0);
            });
        }
        pool.join();
        */
        let mut pop = Population::new(0.1, String::from("."), String::from("test"), false);
            //pop.create_feed_forward(2, 2000, 1.0, 0.1, 20.0, 200.0, 200.1);
            
        pop.create_neuron();
        pop.create_neuron();
        pop.create_neuron();
        pop.create_synapse(0, 2, 12.0, 10.0, 7.0, 7.0, true, false, false, false);
        pop.create_synapse(1, 2, 12.0, 10.0, 7.0, 7.0, true, false, false, false);
        pop.create_input(vec![0], 0.0, vec![1.0], 200.0);
        pop.create_input(vec![1], 0.0, vec![4.0], 200.0);
        
        pop.run(20.0);
         
        
         
    
    
}
