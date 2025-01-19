/// trait for ```min```, ```max``` fns.
/// Is auto implemented for any type that implements ```PartialOrd```.
///
/// Can be implemented by non ```PartialOrd``` number-containers types,
/// by returning the min/max of each element.
pub trait MinMax {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

impl<T: PartialOrd> MinMax for T {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
}
