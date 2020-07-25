use super::Pair;
use super::Tdim;
use super::graph::*;

#[derive(Debug,Clone,Default)]
struct Grid<T>{
    grid: Tdim<T>,
    seen: Tdim<bool>,
    x_num: usize,
    y_num: usize,
    start: Pair<usize>,
    goal: Pair<usize>,
}

impl graph_base<char,Pair<usize>> for Grid<char>{
    fn new() -> Self {
        Grid::default()
    }

    fn see(&mut self,now: Pair<usize>){
        self.seen[now.0][now.1] = true;
        let dx: Vec<isize> = vec![-1,0,1,0];
        let dy: Vec<isize> = vec![0,-1,0,1];
        for i in 0..4{
            let (nx,ny) = (now.0 as isize + dx[i],now.1 as isize + dy[i]);
            if ny < 0 || ny >= self.y_num as isize || nx < 0 || nx >= self.x_num as isize{
                continue;
            }
            
            let (nx,ny) = (nx as usize,ny as usize);
            
            if self.grid[nx][ny] == 'w'{
                continue;
            }

            if self.seen[nx][ny]{
                continue;
            }

            self.see((nx,ny));
        }
    }

    fn connected(&mut self,s: Pair<usize>,g: Pair<usize>) -> bool {
        self.refresh();
        self.see(s);
        self.seen[g.0][g.1]
    }

    fn refresh(&mut self) {
        let mut tmp = Vec::new();
        tmp.resize(self.seen[0].len(), false);
        self.seen.resize(self.seen.len(), tmp);
    }

    fn init(grid: Tdim<char>,size: Pair<usize>) -> Self{
        let mut tmp = Grid::new();
        let mut tmp_b: Vec<bool> = Vec::new();
        tmp_b.resize(size.1 + 1, false);
        tmp.seen.resize(size.0 + 1,tmp_b);
        tmp.x_num = size.0;
        tmp.y_num = size.1;
        tmp.grid = grid;
        tmp
    }
}

impl Graph<char>{

}