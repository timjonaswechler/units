use crate::features::DefaultFloat;

pub trait UnitScale {
    const SCALE: DefaultFloat = 1.0;

    fn scale() -> DefaultFloat {
        Self::SCALE
    }
}
