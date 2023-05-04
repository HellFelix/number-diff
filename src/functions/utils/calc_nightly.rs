use crate::Function;
use crate::Elementary;

impl FnOnce<(f64,)> for Function {
    type Output = f64; 
    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        self.func.clone().call()(args.0)
    }
}

impl FnMut<(f64,)> for Function {
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        self.func.clone().call()(args.0)
    }
}

impl Fn<(f64,)> for Function {
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        self.func.clone().call()(args.0)
    }
}
