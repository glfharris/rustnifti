
/// Trait returning a value from a function
pub trait Eval {
    fn value(&self, x: f32) -> f32;
}

/// Trait to determine domain of a function
pub trait Domain {
    /// returns true if x in the domain of the funtions
    fn in_domain(&self, x: f32) -> bool;
}
