use num::{BigInt, ToPrimitive};
use std::{
    cmp::Ordering,
    convert::TryInto,
    ops::{Add, AddAssign, Mul, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Cents {
    value: u64,
}

impl Cents {
    pub fn new(dollars: i64, cents: u8) -> Self {
        let dollars: u64 = num::abs(dollars).try_into().unwrap();

        Self {
            value: (dollars * 100) + cents as u64,
        }
    }
}

impl From<Cents> for u64 {
    fn from(cents: Cents) -> u64 {
        cents.value
    }
}

impl From<u64> for Cents {
    fn from(int: u64) -> Self {
        Self { value: int }
    }
}

impl From<BigInt> for Cents {
    fn from(int: BigInt) -> Self {
        Self {
            value: num::abs(int).to_u64().unwrap(),
        }
    }
}

impl From<&BigInt> for Cents {
    fn from(int: &BigInt) -> Self {
        Self {
            value: num::abs(int.to_owned()).to_u64().unwrap(),
        }
    }
}

impl Add for Cents {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Add<u64> for Cents {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        Self {
            value: self.value + other,
        }
    }
}

impl AddAssign for Cents {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            value: self.value + other.value,
        }
    }
}

impl AddAssign<u64> for Cents {
    fn add_assign(&mut self, other: u64) {
        *self = Self {
            value: self.value + other,
        }
    }
}

impl Mul for Cents {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Mul<u64> for Cents {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            value: self.value * rhs,
        }
    }
}

impl PartialEq for Cents {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Cents {}

impl PartialOrd for Cents {
    fn partial_cmp(&self, other: &Cents) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cents {
    fn cmp(&self, other: &Cents) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Sub for Cents {
    type Output = Cents;

    fn sub(self, other: Cents) -> Cents {
        Self {
            value: self.value - other.value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cents;

    #[test]
    fn test_add_to_u64() {
        let actual = Cents::new(21, 21) + 2121;
        let expected = Cents::new(42, 42);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compare_to_u64() {
        assert_eq!(Cents::new(42, 42), 4242.into());
    }

    #[test]
    fn test_add_two_cents() {
        assert_eq!(Cents::from(2121) + Cents::from(2121), Cents::new(42, 42));
    }

    #[test]
    fn test_one_cent_should_equal_one() {
        assert_eq!(Cents::new(0, 1), 1.into());
    }
}
