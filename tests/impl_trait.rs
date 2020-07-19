use macro_galois_field::Field;
use num_traits::{One, Zero};

#[derive(Field, Debug, Default, Copy, Clone)]
#[prime = 2]
struct Fp2(u64);

impl One for Fp2 {
    fn one() -> Self {
        Fp2(1)
    }
}

impl Zero for Fp2 {
    fn zero() -> Self {
        Fp2(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[test]
fn test_ops_fp2() {
    let a = Fp2(3);
    let b = Fp2(3);
    assert_eq!(a + b, Fp2(0), "{} + {}", a.0, b.0);
    assert_eq!(a - b, Fp2(0), "{} - {}", a.0, b.0);
    assert_eq!(a - b, Fp2(2), "{} - {}", a.0, b.0);
    assert_eq!(a * b, Fp2(1), "{} * {}", a.0, b.0);
    assert_eq!(a * b, Fp2(3), "{} * {}", a.0, b.0);
    assert_eq!(a / b, Fp2(1), "{} / {}", a.0, b.0);

    let a = Fp2(3);
    let b = Fp2(100);
    assert_eq!(a + b, Fp2(1), "{} + {}", a.0, b.0);
    assert_eq!(a - b, Fp2(1), "{} - {}", a.0, b.0);
}

#[derive(Field, Debug, Default, Copy, Clone)]
#[prime = 99991]
struct Fp99991(u64);

impl One for Fp99991 {
    fn one() -> Self {
        Fp99991(1)
    }
}

impl Zero for Fp99991 {
    fn zero() -> Self {
        Fp99991(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[test]
fn test_ops_fp99991() {
    let a = Fp99991(3);
    let b = Fp99991(1000000);
    assert_eq!(a + b, Fp99991(93), "{} + {}", a.0, b.0);
    assert_eq!(a - b, Fp99991(99904), "{} - {}", a.0, b.0);
    assert_eq!(a * b, Fp99991(270), "{} * {}", a.0, b.0);
    assert_eq!(a / b, Fp99991(96658), "{} / {}", a.0, b.0);
    let a = Fp99991(1);
    assert_eq!(a / b, Fp99991(98880), "{} / {}", a.0, b.0);
}