use slab_tree::*;

impl PolychronousPattern{
    pub fn add_spike(t: f64){
        let tree = slab_tree::Trebuilder::new().with_root(t as String).build();
    }
}
