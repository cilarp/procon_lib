use super::graph;
use super::Tdim;

#[derive(Debug,Clone,Default)]
struct Tree<T>{
    nodes: Tdim<T>,
    seen: Vec<bool>,
    root: T,
    depth: Vec<usize>,
    subtree_size: Vec<usize>
}

impl graph::graph_base<usize,usize> for Tree<usize>{
    fn new() -> Self{
        Tree::default()
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

    fn refresh(&mut self) {
        self.seen = Vec::default();
        self.depth = Vec::default();
    }

    fn init(nodes: Tdim<usize>,size: usize) -> Self{
        let mut tmp = Tree::new();
        tmp.nodes = nodes;
        tmp.seen.resize(size + 1,false);
        tmp.depth.resize(size + 1, 0);
        tmp.subtree_size.resize(size + 1, 0);
        tmp
    }
}

impl Tree<usize>{
    fn new() -> Self{
        Tree::default()
    }

    fn calc_depth_and_subtreesize(&mut self,now: usize,parent: usize,depth: usize){
        self.depth[now] = depth;
        for next in self.nodes[now].clone(){
            if next == parent{
                continue;
            }
            self.calc_depth_and_subtreesize(next, now, depth + 1);
        }

        self.subtree_size[now] = 1;
        for child in self.nodes[now].clone(){
            if child == parent{
                continue;
            }
            self.subtree_size[now] += self.subtree_size[child];
        }
    }


}
