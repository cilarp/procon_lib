pub type Pair<T> = (T,T);
pub type Stack<T> = Vec<T>;
pub type Queue<T> = std::collections::VecDeque<T>;
pub type Tdim<T> = Vec<Vec<T>>;

pub mod graph;
pub mod tree;
pub mod grid;

