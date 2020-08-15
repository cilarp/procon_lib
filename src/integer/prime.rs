pub trait Prime {
    fn is_prime(&self) -> bool;
    fn eratosthenes(&self) -> Vec<usize>;
    fn prime_factorization(&self) -> Vec<(usize, usize)>;
}

macro_rules! impl_prime {
    ($($t:ty),*) => {
        $(
            impl Prime for $t{
                fn is_prime(&self) -> bool{
                    let mut n = *self;
                    if n < 2{
                        return false
                    }
                    let maxi = (n as f64).powf(0.5) as Self;
                    for i in 2..=maxi{
                        if n % i == 0{
                            return false;
                        }
                    }
                    return true;
                }

                fn eratosthenes(&self) -> Vec<usize>{
                    let n = *self as usize;
                    let mut spf = vec![None; n+1];
                    let mut is_prime = vec![true; n+1];
                    let mut primes = Vec::new();

                    is_prime[0] = false;
                    is_prime[1] = false;

                    for i in 2..n+1 {
                        if is_prime[i] {
                            primes.push(i);
                            spf[i] = Some(i);
                        }
                        for prime in &primes {
                            if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                                break;
                            }
                            is_prime[i * prime] = false;
                            spf[i * prime] = Some(*prime);
                        }
                    }
                    primes
                }

                fn prime_factorization(&self) -> Vec<(usize,usize)>{
                    let mut res = Vec::new();
                    let mut n = *self as usize;
                    let ps = n.eratosthenes();
                    for i in ps{
                        let mut exp = 0;
                        while n % i == 0{
                            n /= i;
                            exp += 1;
                        }
                        if exp != 0{
                            res.push((i,exp));
                        }
                    }
                    if n > 1{
                        res.push((n,1));
                    }
                    res
                }
            }
        )*
    };
}

impl_prime!(u8, u16, u32, u64, u128, usize);
