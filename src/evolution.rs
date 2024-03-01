use num_cpus;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::{borrow::Borrow, thread};
use std::time::Duration;
use threadpool::ThreadPool;
use crate::population::Population;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use csv::{Reader, StringRecord};
use tokio;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::parameters::Parameters;
use itertools::Itertools;
use rand::seq::SliceRandom;

pub fn get_individuals()->Vec<Population>{
    let seed: u64 = 1;
    let params : Vec<Parameters> = vec![
        Parameters::new_FeedForward(String::from("ind1"), 256, 0.1, 1, 0.1, 1.0, (1.0, 10.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind2"), 256, 0.2, 2, 0.2, 1.0, (1.0, 15.0), false, 0.8, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind3"), 256, 0.3, 3, 0.3, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind4"), 256, 0.4, 4, 1.0, 1.0, (5.0, 10.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind5"), 256, 0.5, 5, 1.0, 1.0, (10.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind6"), 256, 0.6, 1, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind7"), 256, 0.7, 2, 1.0, 1.0, (10.0, 15.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind8"), 256, 0.8, 3, 0.1, 1.0, (1.0, 5.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind9"), 256, 0.9, 4, 0.2, 1.0, (10.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind10"), 256, 1.0, 5, 0.3, 1.0, (15.0, 30.0), false, 0.1, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind11"), 256, 0.1, 1, 1.0, 1.0, (1.0, 30.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind12"), 256, 0.2, 2, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind13"), 256, 0.3, 3, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind14"), 256, 0.4, 4, 1.0, 1.0, (1.0, 20.0), false, 0.5, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind15"), 256, 0.5, 5, 1.0, 1.0, (1.0, 30.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind16"), 256, 0.6, 1, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind17"), 256, 0.7, 2, 1.0, 1.0, (1.0, 30.0), false, 0.3, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind18"), 256, 0.8, 3, 0.1, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind19"), 256, 0.9, 4, 0.2, 1.0, (1.0, 30.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind20"), 256, 1.0, 5, 0.3, 1.0, (10.0, 10.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind21"), 256, 0.1, 1, 1.0, 1.0, (10.0, 10.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind22"), 256, 0.2, 2, 1.0, 1.0, (20.0, 20.0), false, 0.8, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind23"), 256, 0.3, 3, 1.0, 1.0, (10.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind24"), 256, 0.4, 4, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind25"), 256, 0.5, 5, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind26"), 256, 0.6, 1, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind27"), 256, 0.7, 2, 0.1, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind28"), 256, 0.8, 3, 0.2, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind29"), 256, 0.9, 4, 0.3, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind30"), 256, 1.0, 5, 1.0, 1.0, (1.0, 20.0), false, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind31"), 256, 0.1, 1, 1.0, 1.0, (1.0, 20.0), true, 1.0, (1.0, 20.0), seed),
        Parameters::new_FeedForward(String::from("ind32"), 256, 0.2, 2, 1.0, 1.0, (1.0, 20.0), true, 1.0, (1.0, 20.0), seed),
        
        
    ];
    let pops : Vec<Population> = params.into_par_iter().map(| param| create_network(param)).collect();
    pops
}

fn create_network(parameters : Parameters)->Population{
    let name = &parameters.name;
    let mut pop = Population::new(String::from(name));
    pop.create_feed_forward(parameters.l.unwrap(), parameters.n/parameters.l.unwrap(), parameters.p.unwrap(), parameters.declining_learning_rate, parameters.w_learning_p, parameters.w_span,parameters.partial_d, parameters.d_learning_p, parameters.d_span, parameters.seed.unwrap());
    for inp in 0..256{
        if pop.name == "ind6"{
            pop.create_input(vec![inp], 0.9, vec![], 30.0);
        }else{
            pop.create_input(vec![inp], 0.2, vec![], 30.0);
        }
        }
    pop
}

fn run_thread(mut pop : Population, duration : f64, generations : i32, store_data : bool)->(f64, std::option::Option<Parameters>){
    pop.name += &String::from("_gen_".to_owned() + &generations.to_string());
    println!("Running individual: {}", pop.name);
    pop.run(duration,  store_data, true)
}

    
pub fn run_evolution(duration : f64, generations : i32, individuals : Vec<Population>, top_n : i32, best_n : i32, mutation_p : f64){
    let mut pops = individuals;
    let mut id = 21;
    for gen in 0..generations{
        println!("--- Running gen: {} ---", gen.to_string());
        let store_data = if gen == generations-1{
            true
        }else{
            false
        };
        let result : Vec<(f64, std::option::Option<Parameters>)> = pops.into_par_iter().map(| pop| run_thread(pop, duration, gen, store_data)).collect();
        let mut best_ind : Vec<(f64, std::option::Option<Parameters>)> = vec![];
        for (score, params) in result{
            println!("Score {}", score);
            if best_ind.len() < best_n as usize || (best_ind.to_owned().into_iter().any(|x| x.0 < score)){
                best_ind.push((score, params));
                if best_ind.len() > best_n as usize{
                    best_ind.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    best_ind.remove(0);
                }
                
            } 
        }
        pops = vec![];
        println!("best ind len {}", best_ind.len());
        best_ind.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let top_ind = &best_ind[best_ind.len()-top_n as usize..];
        for ind in top_ind{
            println!("Top ind score: {}", ind.0);
            let pop = create_network(ind.1.clone().unwrap());
            pops.push(pop);
        }
        println!("top ind len {}", pops.len());
        let combinations = best_ind.iter().combinations(2);
        for ind in combinations{
            if ind[0].1.clone().unwrap().topology == "ff"{
                let n = vec![ind[0].1.clone().unwrap().n, ind[1].1.clone().unwrap().n].choose(&mut rand::thread_rng()).unwrap().clone();
                let p = vec![ind[0].1.clone().unwrap().p, ind[1].1.clone().unwrap().p].choose(&mut rand::thread_rng()).unwrap().unwrap();
                let l = vec![ind[0].1.clone().unwrap().l, ind[1].1.clone().unwrap().l].choose(&mut rand::thread_rng()).unwrap().unwrap();
                let declining_learning_rate = *vec![ind[0].1.clone().unwrap().declining_learning_rate, ind[1].1.clone().unwrap().declining_learning_rate].choose(&mut rand::thread_rng()).unwrap();
                let w_learning_p = *vec![ind[0].1.clone().unwrap().w_learning_p, ind[1].1.clone().unwrap().w_learning_p].choose(&mut rand::thread_rng()).unwrap();
                let w_span = *vec![ind[0].1.clone().unwrap().w_span, ind[1].1.clone().unwrap().w_span].choose(&mut rand::thread_rng()).unwrap();
                let partial_d = *vec![ind[0].1.clone().unwrap().partial_d, ind[1].1.clone().unwrap().partial_d].choose(&mut rand::thread_rng()).unwrap();
                let d_learning_p = *vec![ind[0].1.clone().unwrap().d_learning_p, ind[1].1.clone().unwrap().d_learning_p].choose(&mut rand::thread_rng()).unwrap();
                let d_span = *vec![ind[0].1.clone().unwrap().d_span, ind[1].1.clone().unwrap().d_span].choose(&mut rand::thread_rng()).unwrap();
                let seed = vec![ind[0].1.clone().unwrap().seed, ind[1].1.clone().unwrap().seed].choose(&mut rand::thread_rng()).unwrap().unwrap();
                let param = Parameters::new_FeedForward(String::from(String::from("ind") + &id.to_string()), n, p, l, declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed);
                let pop = create_network(param);
                id += 1;
                pops.push(pop);
            }else{
                println!("NOOOOO!");
            }
               
        }
    }
}

