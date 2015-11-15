use signals::Unit;

#[derive(Debug, Clone)]
pub struct Series {
    name: String,
    data: Vec<f32>,
    unit: Unit
}
