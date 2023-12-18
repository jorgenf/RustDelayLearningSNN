mod population;
use population::Population;
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
            pop.create_feed_forward(10, 100, 1.0, 1.0, 10.0, 1.0, 10.0);
            
                
                pop.run(1000.0);
         
        
         
    
    
}
