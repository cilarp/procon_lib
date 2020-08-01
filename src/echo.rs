use std::io::Write;

#[macro_export]
macro_rules! echo {
    ($e:expr) => {
        $e.echo(&mut std::io::stdout()).unwrap()
    };
}

pub trait Echo {
    fn echo(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error>;
}

macro_rules! impl_echo {
    ($($t:ty),*) => {
        $(
            impl Echo for $t {
                fn echo(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error>{
                    writeln!(out,"{}", &self)?;
                    Ok(())
                }
            }
        )*
    };
}

macro_rules! impl_echo_f {
    ($($t:ty),*) => {
        $(
            impl Echo for $t {
                fn echo(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error>{
                    writeln!(out,"{:.12}", &self)?;
                    Ok(())
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
    fn echo(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error> {
        for i in self {
            i.echo(out).unwrap();
        }
        Ok(())
    }
}
