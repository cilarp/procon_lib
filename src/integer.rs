pub trait Integer {
    fn gcd(a: Self, b: Self) -> Self;
    fn lcm(a: Self, b: Self) -> Self;
    fn ext_gcd(a: Self, b: Self, x: &mut isize, y: &mut isize) -> Self;
    fn gcd_with(&self, other: Self) -> Self;
    fn lcm_with(&self, other: Self) -> Self;
    fn is_prime(&self) -> bool;
    fn get_div(&self) -> Vec<usize>;
    fn digit(&self) -> usize;
    fn prime_fact(&self) -> Vec<Tuple<usize>>;
    fn div_num(&self) -> usize;
}

pub type Tuple<T> = (T, T);

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t{
                fn gcd(a: Self,b: Self) -> Self{
                    let mut a = a;
                    let mut b = b;
                    if a < b{
                       let c = b;
                        b = a;
                        a = c;
                    }
                    if b == 0{
                        a
                    }else{
                        Self::gcd(b,a % b)
                    }
                }

                fn lcm(a: Self,b: Self) -> Self{
                    a / Self::gcd(a,b) * b
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

                fn gcd_with(&self,other: Self) -> Self{
                    Self::gcd(*self,other)
                }

                fn lcm_with(&self, other: Self) -> Self{
                    Self::lcm(*self,other)
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

                fn prime_fact(&self) -> Vec<Tuple<usize>>{
                    let mut n = *self;
                    let mut v: Vec<Tuple<usize>> = Vec::new();

                    let mut i = 2;
                    while i*i <= n{
                        if n % i == 0{
                            let mut ex: usize = 0;
                            while n % i == 0{
                                ex += 1;
                                n /= i;
                            }
                            v.push((i as usize,ex));
                        }
                        i += 1;
                    }
                    if n != 1{
                        v.push((n as usize,1));
                    }
                    v
                }

                fn div_num(&self) -> usize{
                    let v = self.prime_fact();
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
