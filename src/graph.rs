use super::Tdim;
use super::tree;

#[derive(Debug,Clone,Default)]
pub struct Graph<T>{
    nodes: Tdim<T>,
    seen: Vec<bool>,
}

pub trait graph_base<T,G>{
    fn new() -> Self;
    fn see(&mut self,now: G);
    fn connected(&mut self,s: G,g: G) -> bool;
    fn refresh(&mut self);
    fn init(nodes: Tdim<T>,size: G) -> Self;
}

impl graph_base<usize,usize> for Graph<usize>{
    fn new() -> Self{
        Graph::default()
    }

    fn see(&mut self,now: usize){
        self.seen[now] = true;

        for next in self.nodes[now].clone(){
            if self.seen[next]{
                continue;
            }
            self.see(next);
        }
    }

    fn connected(&mut self,s: usize,g: usize) -> bool{
        self.refresh();
        self.see(s);
        self.seen[g]
    }

    fn refresh(&mut self){
        self.seen.resize(self.seen.len(),false);
    }

    fn init(nodes: Tdim<usize>,size: usize) -> Self{
        let mut tmp = Graph::new();
        tmp.nodes = nodes;
        tmp.seen.resize(size + 1,false);
        tmp
    }
}

impl Graph<usize>{

}

