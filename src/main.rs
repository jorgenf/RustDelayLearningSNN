mod population;
use population::Population;
mod nodes;
mod connections;
mod plotting;
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
        let mut pop = Population::new(0.1, String::from("."), String::from("test"), true);
        pop.create_feed_forward(2, 3, 1.0, 1.0, 1.0, 20.0, 20.0);
        
        for i in 0..3{
            pop.create_input(vec![i], 0.0, vec![i as f32 + 0.1], 20.0);
        }
        
        pop.run(200.0);

        
         
    
    
}
