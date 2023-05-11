use crate::Function;

impl Function {
    pub fn call(&self, x: f64) -> f64 {
        self.func.clone().call()(x)
    }
}
