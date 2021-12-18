pub trait FitnessEvaluator {
    fn evaluate(&self, func: Box<dyn Fn(&Vec<f32>) -> f32>) -> f32;
}