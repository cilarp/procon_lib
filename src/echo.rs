use std::io::{stdout, BufWriter, StdoutLock, Write};

pub trait EchoBase {
    fn echo(&self);
    fn echo_with_stdout(&self, out: &mut BufWriter<StdoutLock>);
}

pub trait Echo {
    fn echo(&self);
}

macro_rules! impl_echo_base {
    ($($t:ty),*) => {
        $(
            impl EchoBase for $t{
                fn echo(&self){
                    println!("{}",*self);
                }

                fn echo_with_stdout(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{}",self).unwrap();
                }
            }
        )*
    };
}

macro_rules! impl_echo_base_f {
    ($($t:ty),*) => {
        $(
            impl EchoBase for $t{
                fn echo(&self){
                    println!("{}",*self);
                }

                fn echo_with_stdout(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{:.12}",self).unwrap();
                }
            }
        )*
    };
}

impl_echo_base!(u8, u16, u32, u64, u128, usize);
impl_echo_base!(i8, i16, i32, i64, i128, isize);
impl_echo_base!(char, &str, String);
impl_echo_base_f!(f32, f64);

impl<T: EchoBase> Echo for Vec<T> {
    fn echo(&self) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        for i in self.clone().iter() {
            i.echo_with_stdout(&mut out);
        }
    }
}
