use std::io::{stdout, BufWriter, StdoutLock, Write};

pub trait EchoBase {
    fn echo(&self);
    fn echo_with_space(&self);
    fn echo_with_nothing(&self);
    fn stdout_echo(&self, out: &mut BufWriter<StdoutLock>);
    fn stdout_echo_with_space(&self, out: &mut BufWriter<StdoutLock>);
    fn stdout_echo_with_nothing(&self, out: &mut BufWriter<StdoutLock>);
}

pub trait Echo {
    fn echo(&self);
    fn echo_with_space(&self);
    fn echo_with_nothing(&self);
}

macro_rules! impl_echo_base {
    ($($t:ty),*) => {
        $(
            impl EchoBase for $t{
                fn echo(&self){
                    println!("{}",*self);
                }

                fn echo_with_space(&self){
                    print!("{} ",*self);
                }

                fn echo_with_nothing(&self){
                    print!("{}",*self);
                }

                fn stdout_echo(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{}",self).unwrap();
                }


                fn stdout_echo_with_space(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{} ",self).unwrap();
                }


                fn stdout_echo_with_nothing(&self,out: &mut BufWriter<StdoutLock>){
                    write!(out,"{}",self).unwrap();
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
                    println!("{:.12}",*self);
                }

                fn echo_with_space(&self){
                    print!("{:.12} ",*self);
                }

                fn echo_with_nothing(&self){
                    print!("{:.12}",*self);
                }

                fn stdout_echo(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{:.12}",self).unwrap();
                }


                fn stdout_echo_with_space(&self,out: &mut BufWriter<StdoutLock>){
                    writeln!(out,"{:.12} ",self).unwrap();
                }


                fn stdout_echo_with_nothing(&self,out: &mut BufWriter<StdoutLock>){
                    write!(out,"{:.12}",self).unwrap();
                }
            }
        )*
    };
}

impl_echo_base!(u8, u16, u32, u64, u128, usize);
impl_echo_base!(i8, i16, i32, i64, i128, isize);
impl_echo_base!(char, &str, String);
impl_echo_base_f!(f32, f64);

impl<T: EchoBase> Echo for [T] {
    fn echo(&self) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        for i in self.clone().iter() {
            i.stdout_echo(&mut out);
        }
    }

    fn echo_with_space(&self) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        for i in self.clone().iter() {
            i.stdout_echo_with_space(&mut out);
        }
        write!(out, "\n").unwrap();
    }

    fn echo_with_nothing(&self) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        for i in self.clone().iter() {
            i.stdout_echo_with_nothing(&mut out);
        }
        write!(out, "\n").unwrap();
    }
}
