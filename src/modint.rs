use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
#[derive(Debug, Copy, Clone)]
struct ModInt {
    m: usize,
    content: usize,
}

macro_rules! impl_add {
    ($($t:ty),*) => {
        $(
            impl Add<$t> for ModInt {
                type Output = ModInt;
                fn add(self, rhs: $t) -> Self::Output {
                    Self {
                        m: self.m,
                        content: (self.content + (rhs % self.m as $t) as usize) % self.m,
                    }

                }
            }
        )*
    };
}
macro_rules! impl_add_as {
    ($($t:ty),*) => {
        $(
            impl AddAssign<$t> for ModInt {
                fn add_assign(&mut self, rhs: $t){
                    self.content = (self.content + (rhs % self.m as $t) as usize) % self.m;

                }
            }
        )*
    };
}
macro_rules! impl_sub {
    ($($t:ty),*) => {
        $(
            impl Sub<$t> for ModInt {
                type Output = ModInt;
                fn sub(self, rhs: $t) -> Self::Output {
                    Self {
                        m: self.m,
                        content: (self.content - (rhs % self.m as $t) as usize) % self.m,
                    }

                }
            }
        )*
    };
}
macro_rules! impl_sub_as {
    ($($t:ty),*) => {
        $(
            impl SubAssign<$t> for ModInt {
                fn sub_assign(&mut self, rhs: $t){
                    self.content = (self.content - (rhs % self.m as $t) as usize) % self.m;
                }
            }
        )*
    };
}
macro_rules! impl_mul {
    ($($t:ty),*) => {
        $(
            impl Mul<$t> for ModInt{
                type Output = ModInt;
                fn mul(self,rhs: $t) -> Self::Output{
                    Self{
                        m: self.m,
                        content: self.content * (rhs % self.m as $t) as usize % self.m,
                    }
                }
            }
        )*
    };
}
macro_rules! impl_mul_as {
    ($($t:ty),*) => {
        $(
            impl MulAssign<$t> for ModInt {
                fn mul_assign(&mut self, rhs: $t){
                    self.content = (self.content * (rhs % self.m as $t) as usize) % self.m;
                }
            }
        )*
    };
}
macro_rules! impl_div {
        ($($t:ty),*) => {
        $(
            impl Div<$t> for ModInt{
                type Output = ModInt;
                fn div(self,rhs: $t) -> Self::Output{
                    let rhs = rhs as u128 % self.m as u128;
                    let rhs = ModInt::new(self.m, rhs as usize);
                    Self{
                        m: self.m,
                        content: self.content * rhs.inverse() % self.m,
                    }
                }
            }
        )*
    };
}
macro_rules! impl_div_as {
    ($($t:ty),*) => {
        $(
            impl DivAssign<$t> for ModInt{
                fn div_assign(&mut self,rhs: $t){
                    let rhs = rhs as u128 % self.m as u128;
                    let rhs = ModInt::new(self.m, rhs as usize);
                    self.content = self.content * rhs.inverse() % self.m;
                }
            }
        )*
    };
}

impl_add!(u8, u16, u32, u64, u128, usize);
impl_add_as!(u8, u16, u32, u64, u128, usize);
impl_sub!(u8, u16, u32, u64, u128, usize);
impl_sub_as!(u8, u16, u32, u64, u128, usize);
impl_mul!(u8, u16, u32, u64, u128, usize);
impl_mul_as!(u8, u16, u32, u64, u128, usize);
impl_div!(u8, u16, u32, u64, u128);
impl_div_as!(u8, u16, u32, u64, u128);

impl ModInt {
    #[allow(dead_code)]
    fn new(m: usize, content: usize) -> Self {
        Self {
            m,
            content: content % m,
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, content: usize) {
        self.content = content % self.m;
    }

    #[allow(dead_code)]
    fn ext_gcd(a: usize, b: usize, x: &mut isize, y: &mut isize) -> usize {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }
        let d = Self::ext_gcd(b, a % b, y, x);
        *y = *y - (a / b) as isize * (*x);
        d
    }

    #[allow(dead_code)]
    fn inverse(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        Self::ext_gcd(self.content, self.m, &mut x, &mut y);
        if x < 0 {
            let x = x.abs() as usize * 2;
            let x = x % self.m;
            x
        } else {
            let x = x as usize % self.m;
            x
        }
    }
}
