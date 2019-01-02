//! Revolut currency amount
//!
//! This module holds the `Amount` type and the `AmountParseError`.
//!
//! The maximum and minimum amount values can in any case be known by using `max_value()` and
//! `min_value()` functions in the `Amount` type, or the `MAX` and `MIN` constants:
//!
//! ```
//! use std::u64;
//! use revolut_customer::Amount;
//!
//! let max_value = Amount::max_value();
//! let min_value = Amount::min_value();
//!
//! assert_eq!(max_value, Amount::from_repr(u64::max_value()));
//! assert_eq!(min_value, Amount::from_repr(u64::min_value()));
//! ```

use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
    u64,
};

use failure::{Error, Fail, ResultExt};
use serde::{Deserialize, Serialize};

/// Largest possible currency amount.
pub const MAX: Amount = Amount::max_value();
/// Smallest possible currency amount
pub const MIN: Amount = Amount::min_value();

/// Revolut currency amount
///
/// This data structure can be used the same way as any other number. An `Amount` can be added or
/// subtracted to another `Amount`, and it can be divided and multiplied by an integer. All
/// operations that are defined in the `Amount` scope and that are exact can be used directly as
/// usual integer / float point operations.
///
/// No negative amounts can exist, since an `Amount` is unsigned, so the negation operator '-',
/// then, has no use with an `Amount`.
///
/// Its internal representation is a 64 bit unsigned integer, that is displayed as a fixed point,
/// number of factor 1/100. This means that an internal representation of `100` would be an
/// external amount of `1`. The internal representation shouldn't be used except when serializing
/// and deserializing the data, since this type is sent in *JSON* as its internal `u64`.
///
/// The use is the following:
///
/// ```
/// use revolut_customer::Amount;
///
/// let amount = Amount::from_repr(1_65); // 1.65
/// let ten = Amount::from_repr(10_00); // 10
/// let add_ten = amount + ten;
/// assert_eq!(add_ten, Amount::from_repr(11_65)); // 11.65
/// ```
///
/// They can be divided and multiplied by any other unsigned integer:
///
/// ```
/// # use revolut_customer::Amount;
/// let mut amount = Amount::from_repr(7_00); // 7
/// amount *= 10u32;
/// assert_eq!(amount, Amount::from_repr(70_00)); // 70
///
/// amount = amount / 30u16;
/// assert_eq!(amount, Amount::from_repr(2_33)); // 2.33
///
/// amount %= 1u8;
/// assert_eq!(amount, Amount::from_repr(0_33)); // 0.33
/// ```
///
/// Amounts can easily be displayed using the `Display` trait as any other number:
///
/// ```
/// # use revolut_customer::Amount;
///
/// let amount = Amount::from_repr(56_00);
/// assert_eq!(format!("{}", amount), "56");
/// assert_eq!(format!("{:.2}", amount), "56.00");
/// assert_eq!(format!("{:.5}", amount), "56.00000");
/// assert_eq!(format!("{:05.1}", amount), "056.0");
///
/// // And with rounding:
/// let amount = Amount::from_repr(0_56); // 0.56
/// assert_eq!(format!("{:.1}", amount), "0.6");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Amount {
    value: u64,
}

impl Amount {
    /// Creates a new amount from its internal representation.
    pub fn from_repr(value: u64) -> Self {
        Self { value }
    }

    /// Gets the internal representation of the amount.
    pub fn get_repr(self) -> u64 {
        self.value
    }

    /// Returns the smallest value that can be represented as a currency amount.
    pub const fn min_value() -> Self {
        Self {
            value: u64::min_value(),
        }
    }

    /// Returns the largest value that can be represented as a currency amount.
    pub const fn max_value() -> Self {
        Self {
            value: u64::max_value(),
        }
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let units = self.value / 1_00;
        let decimal_repr = self.value % 1_00;
        let result = match f.precision() {
            None => {
                if decimal_repr == 0 {
                    format!("{}", units)
                } else if decimal_repr % 10 == 0 {
                    format!("{}.{:01}", units, decimal_repr / 10)
                } else {
                    format!("{}.{:02}", units, decimal_repr)
                }
            }
            // No decimal digits.
            Some(0) => format!("{}", if decimal_repr >= 50 { units + 1 } else { units }),
            // One decimal digit.
            Some(1) => format!(
                "{}.{:01}",
                units,
                if decimal_repr % 10 >= 5 {
                    decimal_repr / 10 + 1
                } else {
                    decimal_repr / 10
                }
            ),
            // 2 or more decimal digits precision.
            Some(p) => {
                let mut string = format!("{}.{:02}", units, decimal_repr);
                for _ in 2..p {
                    string.push('0');
                }
                string
            }
        };

        match f.width() {
            None => write!(f, "{}", result),
            Some(w) => {
                if w < result.len() {
                    write!(f, "{}", result)
                } else {
                    let mut pad = String::new();
                    for _ in result.len()..w {
                        pad.push('0');
                    }
                    write!(f, "{}{}", pad, result)
                }
            }
        }
    }
}

impl FromStr for Amount {
    type Err = Error;
    #[allow(clippy::cast_possible_truncation)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            let parts = s.split('.').count();
            let mut split = s.split('.');
            match parts {
                2 => {
                    let units_str = split.next().unwrap();
                    let units: u64 = if units_str.is_empty() {
                        0
                    } else {
                        let u = units_str.parse::<u64>().context(AmountParseError {
                            amount_str: s.to_owned(),
                        })?;
                        if u <= u64::max_value() / 1_00 {
                            u * 1_00
                        } else {
                            return Err(AmountParseError {
                                amount_str: s.to_owned(),
                            }
                            .into());
                        }
                    };
                    let mut decimals_str =
                        String::from(split.next().expect("decimals disappeared"));
                    if decimals_str.is_empty() {
                        return Err(AmountParseError {
                            amount_str: s.to_owned(),
                        }
                        .into());
                    }
                    if decimals_str.len() == 1 {
                        decimals_str.push('0');
                    }
                    let decimals: u64 = {
                        let d = decimals_str.parse::<u64>().context(AmountParseError {
                            amount_str: s.to_owned(),
                        })?;
                        if decimals_str.len() == 2 {
                            d
                        } else {
                            let divisor = 10_u64.pow(decimals_str.len() as u32 - 2);
                            let rem = d % divisor;
                            if rem >= divisor / 2 {
                                d / divisor + 1
                            } else {
                                d / divisor
                            }
                        }
                    };

                    if u64::max_value() - decimals >= units {
                        Ok(Self::from_repr(units + decimals))
                    } else {
                        Err(AmountParseError {
                            amount_str: s.to_owned(),
                        }
                        .into())
                    }
                }
                _ => Err(AmountParseError {
                    amount_str: s.to_owned(),
                }
                .into()),
            }
        } else {
            let units = s.parse::<u64>().context(AmountParseError {
                amount_str: s.to_owned(),
            })?;

            if units <= u64::max_value() / 1_00 {
                Ok(Self::from_repr(units * 1_00))
            } else {
                Err(AmountParseError {
                    amount_str: s.to_owned(),
                }
                .into())
            }
        }
    }
}

macro_rules! impl_ops_int {
    ($($t:ty)*) => ($(
        impl Div<$t> for Amount {
            type Output = Self;

            fn div(self, rhs: $t) -> Self {
                Self { value: self.value / u64::from(rhs) }
            }
        }

        impl DivAssign<$t> for Amount {
            fn div_assign(&mut self, rhs: $t) {
                self.value /= u64::from(rhs)
            }
        }

        impl Rem<$t> for Amount {
            type Output = Self;

            fn rem(self, rhs: $t) -> Self {
                Self { value: self.value % (u64::from(rhs) * 1_00)}
            }
        }

        #[allow(clippy::suspicious_op_assign_impl)]
        impl RemAssign<$t> for Amount {
            fn rem_assign(&mut self, rhs: $t) {
                self.value %= u64::from(rhs) * 1_00
            }
        }

        impl Mul<$t> for Amount {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self {
                Self { value: self.value * u64::from(rhs) }
            }
        }

        impl Mul<Amount> for $t {
            type Output = Amount;

            fn mul(self, rhs: Amount) -> Self::Output {
                Self::Output { value: u64::from(self) * rhs.value }
            }
        }

        impl MulAssign<$t> for Amount {
            fn mul_assign(&mut self, rhs: $t) {
                self.value *= u64::from(rhs)
            }
        }
    )*)
}

impl_ops_int! { u8 u16 u32 u64 }

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

/// Revolut amount parse error.
#[derive(Debug, Clone, Fail, PartialEq)]
#[fail(display = "the amount {} is not a valid Revolut amount", amount_str)]
pub struct AmountParseError {
    pub(crate) amount_str: String,
}
