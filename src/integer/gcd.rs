pub trait GCD {
    fn gcd(&self, other: Self) -> Self;
    fn ext_gcd(a: Self, b: Self, x: &mut isize, y: &mut isize) -> Self;
    fn lcm(&self, other: Self) -> Self;
}

macro_rules! impl_gcds {
    ($($t:ty),*) => {
        $(
            impl GCD for $t{
                fn gcd(&self,other: Self) -> Self{
                    let mut n = *self;
                    let mut m = other;
                    if n < m{
                       let c = m;
                        m = n;
                        n = c;
                    }
                    if m == 0{
                        n
                    }else{
                        m.gcd(n % m)
                    }
                }

                fn lcm(&self,other: Self) -> Self{
                    *self / self.gcd(other) * other
                }

                fn ext_gcd(a: Self,b: Self,x: &mut isize,y: &mut isize) -> Self{
                    if b == 0{
                        *x = 1;
                        *y = 0;
                        return a;
                    }
                    let d = Self::ext_gcd(b,a % b,y,x);
                    *y = *y - (a / b) as isize* (*x);
                    d
                }
            }
        )*
    };
}

impl_gcds!(u8, u16, u32, u64, u128);
