pub trait Echo {
    fn echo(&self);
}

macro_rules! impl_echo {
    ($($t:ty),*) => {
        $(
            impl Echo for $t {
                fn echo(&self) {
                    println!("{}", &self);
                }

            }
        )*
    };
}

macro_rules! impl_echo_f {
    ($($t:ty),*) => {
        $(
            impl Echo for $t {
                fn echo(&self) {
                    println!("{:.12}", &self);
                }
            }
        )*
    };
}

impl_echo!(u8, u16, u32, u64, u128, usize);
impl_echo!(i8, i16, i32, i64, i128, isize);
impl_echo!(&str, String, char);
impl_echo_f!(f32, f64);

impl<T: Echo> Echo for Vec<T> {
    fn echo(&self) {
        for i in self {
            i.echo();
        }
    }
}
