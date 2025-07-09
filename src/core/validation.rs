// Trait for compile-time assertions
pub trait ConstAssert<const ASSERTION: bool> {}
impl ConstAssert<true> for () {}
