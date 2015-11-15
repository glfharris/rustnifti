use std::ops::Index;
use signals::Series;

pub struct Frame {
    data: Vec<Series>,
    index: Series,
}
