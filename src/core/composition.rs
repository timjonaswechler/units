//! Unit composition system supporting all three approaches

/// Trait for unit composition - works with single units, tuples, aliases, and prefixed units
pub trait UnitComposition {
    /// Convert from this unit to SI base units
    fn to_si_factor() -> f64;
    
    /// Convert from SI base units to this unit  
    fn from_si_factor() -> f64;
    
    /// Human-readable symbol for this unit
    fn symbol() -> String;
}

// Implementation for empty tuple (SI base units)
impl UnitComposition for () {
    #[inline]
    fn to_si_factor() -> f64 { 1.0 }
    
    #[inline]
    fn from_si_factor() -> f64 { 1.0 }
    
    fn symbol() -> String { "SI".to_string() }
}

// Implementation for tuple units (2 units: U1/U2)
impl<U1, U2> UnitComposition for (U1, U2) 
where 
    U1: UnitComposition, 
    U2: UnitComposition 
{
    #[inline]
    fn to_si_factor() -> f64 { 
        U1::to_si_factor() / U2::to_si_factor() 
    }
    
    #[inline]
    fn from_si_factor() -> f64 { 
        1.0 / Self::to_si_factor() 
    }
    
    fn symbol() -> String { 
        format!("{}/{}", U1::symbol(), U2::symbol()) 
    }
}

// Implementation for tuple units (3 units: U1*U2/U3)
impl<U1, U2, U3> UnitComposition for (U1, U2, U3) 
where 
    U1: UnitComposition, 
    U2: UnitComposition, 
    U3: UnitComposition 
{
    #[inline]
    fn to_si_factor() -> f64 { 
        U1::to_si_factor() * U2::to_si_factor() / U3::to_si_factor() 
    }
    
    #[inline]
    fn from_si_factor() -> f64 { 
        1.0 / Self::to_si_factor() 
    }
    
    fn symbol() -> String { 
        format!("{}⋅{}/{}", U1::symbol(), U2::symbol(), U3::symbol()) 
    }
}

// Implementation for tuple units (4 units: U1*U2/(U3*U4))
impl<U1, U2, U3, U4> UnitComposition for (U1, U2, U3, U4) 
where 
    U1: UnitComposition, 
    U2: UnitComposition, 
    U3: UnitComposition,
    U4: UnitComposition 
{
    #[inline]
    fn to_si_factor() -> f64 { 
        (U1::to_si_factor() * U2::to_si_factor()) / (U3::to_si_factor() * U4::to_si_factor())
    }
    
    #[inline]
    fn from_si_factor() -> f64 { 
        1.0 / Self::to_si_factor() 
    }
    
    fn symbol() -> String { 
        format!("{}⋅{}/{}⋅{}", U1::symbol(), U2::symbol(), U3::symbol(), U4::symbol()) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestUnit1;
    impl UnitComposition for TestUnit1 {
        fn to_si_factor() -> f64 { 1.0 }
        fn from_si_factor() -> f64 { 1.0 }
        fn symbol() -> String { "T1".to_string() }
    }

    struct TestUnit2;
    impl UnitComposition for TestUnit2 {
        fn to_si_factor() -> f64 { 2.0 }
        fn from_si_factor() -> f64 { 0.5 }
        fn symbol() -> String { "T2".to_string() }
    }

    #[test]
    fn test_tuple_composition_2() {
        type TupleUnit = (TestUnit1, TestUnit2);
        
        assert_eq!(TupleUnit::to_si_factor(), 0.5); // 1.0 / 2.0
        assert_eq!(TupleUnit::from_si_factor(), 2.0); // 1.0 / 0.5
        assert_eq!(TupleUnit::symbol(), "T1/T2");
    }
}