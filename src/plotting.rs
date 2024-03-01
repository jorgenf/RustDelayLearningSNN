use std::collections::HashMap;

use plotters::prelude::*;
use std::fs;
use fdg_sim::{ForceGraph, ForceGraphHelper, force};
use fdg_img;

pub fn plot_delays(dir : &String, duration : f64, max_delay : f64, delay_data : HashMap<String, Vec<(f64, f64)>>){
    let path = dir.to_owned() + "/delays.png";
    let root_area = BitMapBackend::new(&path, (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Delay plot", ("sans-serif", 40))
    .build_cartesian_2d(0.0f64..duration, 0.0f64..max_delay)
    .unwrap();
  ctx.configure_mesh().draw().unwrap();
  for (_id, data) in delay_data{
    let mut data_points : Vec<(f64, f64)> = vec![];
    let mut d : f64 = 0.1;
    for  t in 0..(duration * 10.0) as i32{
      let data_copy = data.clone();
      let val = data_copy.into_iter().find(|x| x.0 == t as f64 / 10.0);
      if val != None{
        d = val.unwrap().1;
      }
      data_points.push((t as f64 / 10.0,d));
      
    }
    ctx.draw_series(
        LineSeries::new(data_points, &BLACK)
      ).unwrap();
     
  }
  
}

pub fn plot_network(dir : &String){
  let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    // create a circle
    let nodes = 10;

    graph.add_force_node("0", ());
    for x in 1..nodes {
        graph.add_force_node(x.to_string(), ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    // generate svg text for your graph
    let svg = fdg_img::gen_image(graph, None).unwrap();

    // save the svg on disk (or send it to an svg renderer)
    let path = dir.clone() + "/network.svg";
    fs::write(path, svg.as_bytes()).unwrap();
}
