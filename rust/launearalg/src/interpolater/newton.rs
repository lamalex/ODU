use crate::interpolater::traits::Interpolate;
pub struct NewtonInterpolationSolution();

pub struct NewtonPolynomialInterpolater;
impl Interpolate for NewtonPolynomialInterpolater {
    type Output = NewtonInterpolationSolution;

    fn interpolate(points: Vec<(T, T)>) -> Option<Self::Output> {
        None
    }
}
