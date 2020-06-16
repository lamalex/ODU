pub mod linear_piecewise {
    use crate::{traits::Interpolate, vector::Vector};

    impl<T> Interpolate for Vector<T> {
        fn interpolate(&self, step: f64) -> Vec<String> {
            vec![String::from("eat it")]
        }
    }
}
