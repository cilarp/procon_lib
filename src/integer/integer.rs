use super::prime;

pub trait Integer {
    fn gcd(a: Self, b: Self) -> Self;
    fn lcm(a: Self, b: Self) -> Self;
    fn ext_gcd(a: Self, b: Self, x: &mut isize, y: &mut isize) -> Self;
    fn gcd_with(&self, other: Self) -> Self;
    fn lcm_with(&self, other: Self) -> Self;
    fn get_div(&self) -> Vec<usize>;
    fn digit(&self) -> usize;
    fn div_num(&self) -> usize;
}

pub type Tuple<T> = (T, T);

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t{
                fn get_div(&self) -> Vec<usize>{
                    let n = *self;
                    let mut v: Vec<usize> = Vec::new();
                    let mut i = 1;
                    while i * i <= n{
                        if n % i == 0{
                            v.push(i as usize);
                            if i != n / i{
                                v.push((n / i) as usize);
                            }
                        }
                        i += 1;
                    }
                    v.sort();
                    v
                }

                fn digit(&self) -> usize{
                    let mut r = 0;
                    let mut n = *self;
                    while n != 0{
                        r += 1;
                        n /= 10;
                    }
                    r
                }


                fn div_num(&self) -> usize{
                    let v = self.prime_factorize();
                    let mut res = 1;
                    for i in v{
                        res = res * (i.1 + 1);
                    }
                    res
                }
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize);
