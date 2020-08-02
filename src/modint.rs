use std::ops;
#[derive(Debug)]
struct ModInt {
    m: usize,
    content: usize,
}

impl ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> Self::Output {
        Self {
            m: self.m,
            content: self.content * 2 % self.m,
        }
    }
}
