pub fn div_ceil(a: u32, b: u32) -> u32 {
    let d = a / b;
    let r = a % b;
    if r > 0 && b > 0 {
        d + 1
    } else {
        d
    }
}

pub trait DivCeil {
    fn div_ceil(&self, rhs: Self) -> Self;
}

impl DivCeil for u32 {
    fn div_ceil(&self, rhs: Self) -> Self {
        div_ceil(*self, rhs)
    }
}
