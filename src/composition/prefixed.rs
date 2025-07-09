use crate::define_prefix;
use crate::{core::UnitScale, features::DefaultFloat};
use std::marker::PhantomData;

pub struct Prefixed<P, U>(PhantomData<(P, U)>);

pub(crate) trait Prefix {
    const FACTOR: DefaultFloat;
}

impl<P: Prefix, U: UnitScale> UnitScale for Prefixed<P, U> {
    const SCALE: DefaultFloat = P::FACTOR * U::SCALE;
}
