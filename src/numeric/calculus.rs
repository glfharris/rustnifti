
/// Trait for implementing common calculus on function(mathematical) types.
pub trait Calculus {
    /// Converts self into the integral of itself with c as the constant of integration
    fn integral(&mut self, c: f32) {}
    /// Converst self into the differential of itself
    fn differential(&mut self) {}
}
