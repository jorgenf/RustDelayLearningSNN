use std::collections::HashMap;

use plotters::prelude::*;


pub fn plot_delays(dir : &String, duration : f32, max_delay : f32, delay_data : HashMap<String, Vec<(f32, f32)>>){
    let path = dir.to_owned() + "/delays.png";
    let root_area = BitMapBackend::new(&path, (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Delay plot", ("sans-serif", 40))
    .build_cartesian_2d(0.0f32..duration, 0.0f32..max_delay)
    .unwrap();
  ctx.configure_mesh().draw().unwrap();
  for (_id, data) in delay_data{
    ctx.draw_series(
        LineSeries::new(data, &BLACK)
      ).unwrap();
     
  }
  
}
