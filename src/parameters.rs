use std::collections::HashMap;

#[derive(Clone)]
pub struct Parameters{
    pub name : String,
    pub topology : String,
    pub n : i32,
    pub p : Option<f64>,
    pub l : Option<i32>,
    pub k : Option<f64>,
    pub m : Option<i32>,
    pub declining_learning_rate : f64,
    pub w_learning_p : f64,
    pub w_span : (f64, f64),
    pub partial_d : bool,
    pub d_learning_p : f64,
    pub d_span : (f64, f64),
    pub seed : Option<u64>,
}

impl Parameters{
    pub fn new_FeedForward(name : String, n : i32, p : f64, l : i32, declining_learning_rate : f64, w_learning_p : f64, w_span : (f64, f64), partial_d : bool, d_learning_p : f64, d_span : (f64, f64), seed : u64)-> Self{
        Self{name : name, topology : String::from("ff"), n : n, p : Some(p), l : Some(l), k : None, m : None, declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed : Some(seed)}
    }
    pub fn new_ScaleFree(name : String, n : i32, k : f64, declining_learning_rate : f64, w_learning_p : f64, w_span : (f64, f64), partial_d : bool, d_learning_p : f64, d_span : (f64, f64), seed : u64)-> Self{
        Self{name : name, topology : String::from("sf"), n : n, p : None, l : None, k : Some(k), m : None, declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed : Some(seed)}
    }
    pub fn new_SmallWorld(name : String, n : i32, p : f64, m : i32, declining_learning_rate : f64, w_learning_p : f64, w_span : (f64, f64), partial_d : bool, d_learning_p : f64, d_span : (f64, f64), seed : u64)-> Self{
        Self{name : name, topology : String::from("sw"), n : n, p : Some(p), l : None, k : None, m : Some(m), declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed : Some(seed)}
    }
    pub fn new_Reservoir(name : String, n : i32, p : f64, declining_learning_rate : f64, w_learning_p : f64, w_span : (f64, f64), partial_d : bool, d_learning_p : f64, d_span : (f64, f64), seed : u64)-> Self{
        Self{name : name, topology : String::from("res"), n : n, p : Some(p), l : None, k : None, m : None, declining_learning_rate, w_learning_p, w_span, partial_d, d_learning_p, d_span, seed : Some(seed)}
    }
}