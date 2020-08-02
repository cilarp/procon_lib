pub trait Integer {
    fn gcd(&self, other: Self) -> Self;
    fn lcm(&self, other: Self) -> Self;
    fn is_prime(&self) -> bool;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t{
                fn gcd(&self,other: Self) -> Self{
                    let mut x = *self;
                    let mut y = {
                        if x < other{
                            x = other;
                            *self
                        }else{
                            other
                        }
                    };
                    let mut r = 0;
                    while y > 0{
                        r = x % y;
                        x = y;
                        y = r;
                    }
                    x
                }

                fn lcm(&self, other: Self) -> Self{
                    *self / &self.gcd(other) * other
                }

                fn is_prime(&self) -> bool{
                    let n = *self;
                    if n == 0 || n == 1{
                        return false
                    }

                    if n == 2{
                        return true;
                    }

                    let mut i: Self = 2;
                    while i*i < n{
                        if n % i == 0{
                            return false
                        }
                        i += 1;
                    }
                    true
                }
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize);
