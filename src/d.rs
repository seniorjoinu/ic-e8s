use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use candid::{CandidType, Nat};
use ic_stable_structures::{storable::Bound, Storable};
use num_bigint::BigUint;
use serde::Deserialize;

use crate::{c::ECs, ES_BASES};

/// Fixed-point decimals with primitive math (+-*/) implemented correctly
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EDs {
    pub val: BigUint,
    pub decimals: u8,
}

impl Default for EDs {
    fn default() -> Self {
        Self {
            val: BigUint::default(),
            decimals: 8,
        }
    }
}

impl EDs {
    pub fn new(val: BigUint, decimals: u8) -> Self {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        Self { val, decimals }
    }

    pub fn base(decimals: u8) -> &'static BigUint {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { ES_BASES.get(decimals as usize).unwrap_unchecked() }
    }

    pub fn zero(decimals: u8) -> Self {
        Self::new(BigUint::ZERO, decimals)
    }

    pub fn one(decimals: u8) -> Self {
        Self {
            val: Self::base(decimals).clone(),
            decimals,
        }
    }

    pub fn f0_1(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(10u64), decimals)
    }

    pub fn f0_2(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(5u64), decimals)
    }

    pub fn f0_25(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(4u64), decimals)
    }

    pub fn f0_3(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_33(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(3u64), decimals)
    }

    pub fn f0_4(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(2u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_5(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(2u64), decimals)
    }

    pub fn f0_6(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_67(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(2u64) / BigUint::from(3u64),
            decimals,
        )
    }

    pub fn f0_7(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(7u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_75(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(4u64),
            decimals,
        )
    }

    pub fn f0_8(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(4u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_9(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(9u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn two(decimals: u8) -> Self {
        Self::new(Self::base(decimals) * BigUint::from(2u64), decimals)
    }

    pub fn to_const<const D: usize>(self) -> ECs<D> {
        if self.decimals != D as u8 {
            unreachable!(
                "{} decimals EDs can't be transformed into E{}s!",
                self.decimals, D
            );
        }

        ECs::new(self.val)
    }

    pub fn to_decimals(mut self, new_decimals: u8) -> EDs {
        if new_decimals == self.decimals {
            return self;
        }

        let (dif, mul) = if self.decimals > new_decimals {
            (self.decimals - new_decimals, false)
        } else {
            (new_decimals - self.decimals, true)
        };

        let base = Self::base(dif);
        self.val = if mul {
            self.val * base
        } else {
            self.val / base
        };

        self.decimals = new_decimals;

        self
    }

    pub fn sqrt(&self) -> Self {
        let a = Self::one(self.decimals);
        if self == &a {
            return a;
        }

        let mut low = Self::zero(self.decimals);
        let mut high = self.clone();
        let one = BigUint::from(1u64);

        while (&high - &low).val > one {
            let mid = (&low + &high) / Self::two(self.decimals);
            let mid_squared = &mid * &mid;

            match mid_squared.cmp(self) {
                Ordering::Equal => return mid,
                Ordering::Greater => {
                    high = mid;
                }
                Ordering::Less => {
                    low = mid;
                }
            }
        }

        low
    }
}

impl Display for EDs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = Self::base(self.decimals);

        f.write_str(&format!("{}.{}", &self.val / base, &self.val % base))
    }
}

impl Add for &EDs {
    type Output = EDs;

    fn add(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(&self.val + &rhs.val, self.decimals)
    }
}

impl Add for EDs {
    type Output = EDs;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add<&EDs> for EDs {
    type Output = EDs;

    fn add(self, rhs: &EDs) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<EDs> for &EDs {
    type Output = EDs;

    fn add(self, rhs: EDs) -> Self::Output {
        self.add(&rhs)
    }
}

impl AddAssign<&EDs> for EDs {
    fn add_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.add_assign(&rhs.val)
    }
}

impl AddAssign for EDs {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl Sub for &EDs {
    type Output = EDs;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(&self.val - &rhs.val, self.decimals)
    }
}

impl Sub for EDs {
    type Output = EDs;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Sub<&EDs> for EDs {
    type Output = EDs;

    fn sub(self, rhs: &EDs) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl Sub<EDs> for &EDs {
    type Output = EDs;

    fn sub(self, rhs: EDs) -> Self::Output {
        self.sub(&rhs)
    }
}

impl SubAssign<&EDs> for EDs {
    fn sub_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.sub_assign(&rhs.val)
    }
}

impl SubAssign for EDs {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl Mul for &EDs {
    type Output = EDs;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(
            &self.val * &rhs.val / EDs::base(self.decimals),
            self.decimals,
        )
    }
}

impl Mul for EDs {
    type Output = EDs;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl Mul<&EDs> for EDs {
    type Output = EDs;

    fn mul(self, rhs: &EDs) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<EDs> for &EDs {
    type Output = EDs;

    fn mul(self, rhs: EDs) -> Self::Output {
        self.mul(&rhs)
    }
}

impl Mul<u64> for &EDs {
    type Output = EDs;

    fn mul(self, rhs: u64) -> Self::Output {
        EDs::new(&self.val * BigUint::from(rhs), self.decimals)
    }
}

impl Mul<u64> for EDs {
    type Output = EDs;

    fn mul(self, rhs: u64) -> Self::Output {
        EDs::new(self.val * BigUint::from(rhs), self.decimals)
    }
}

impl MulAssign<&EDs> for EDs {
    fn mul_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val = &self.val * &rhs.val / EDs::base(self.decimals)
    }
}

impl MulAssign for EDs {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl MulAssign<u64> for EDs {
    fn mul_assign(&mut self, rhs: u64) {
        self.val *= BigUint::from(rhs);
    }
}

impl Div for &EDs {
    type Output = EDs;

    fn div(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(
            &self.val * EDs::base(self.decimals) / &rhs.val,
            self.decimals,
        )
    }
}

impl Div for EDs {
    type Output = EDs;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl Div<u64> for EDs {
    type Output = EDs;

    fn div(self, rhs: u64) -> Self::Output {
        EDs::new(self.val / BigUint::from(rhs), self.decimals)
    }
}

impl Div<u64> for &EDs {
    type Output = EDs;

    fn div(self, rhs: u64) -> Self::Output {
        EDs::new(&self.val / BigUint::from(rhs), self.decimals)
    }
}

impl Div<&EDs> for EDs {
    type Output = EDs;

    fn div(self, rhs: &EDs) -> Self::Output {
        (&self).div(rhs)
    }
}

impl Div<EDs> for &EDs {
    type Output = EDs;

    fn div(self, rhs: EDs) -> Self::Output {
        self.div(&rhs)
    }
}

impl DivAssign<&EDs> for EDs {
    fn div_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        *(&mut self.val) = &self.val * EDs::base(self.decimals) / &rhs.val;
    }
}

impl DivAssign for EDs {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}

impl DivAssign<u64> for EDs {
    fn div_assign(&mut self, rhs: u64) {
        self.val /= BigUint::from(rhs);
    }
}

impl From<(u64, u8)> for EDs {
    fn from((value, decimals): (u64, u8)) -> Self {
        Self::new(BigUint::from(value), decimals)
    }
}

impl From<(u128, u8)> for EDs {
    fn from((value, decimals): (u128, u8)) -> Self {
        Self::new(BigUint::from(value), decimals)
    }
}

impl Into<Nat> for EDs {
    fn into(self) -> Nat {
        Nat(self.val)
    }
}

impl Into<Nat> for &EDs {
    fn into(self) -> Nat {
        Nat(self.val.clone())
    }
}

#[derive(CandidType, Deserialize)]
pub struct EDsCandid {
    pub val: Nat,
    pub decimals: u8,
}

impl CandidType for EDs {
    fn _ty() -> candid::types::Type {
        EDsCandid::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        (EDsCandid {
            val: Nat(self.val.clone()),
            decimals: self.decimals,
        })
        .idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EDs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = EDsCandid::deserialize(deserializer)?;

        Ok(Self::new(a.val.0, a.decimals))
    }
}

impl Storable for EDs {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut val_buf = self.val.to_bytes_le();
        let len = val_buf.len();

        assert!(len <= 32, "Unable to encode EDs: value too big");

        val_buf.resize(34, 0);
        val_buf[32] = self.decimals;
        val_buf[33] = len as u8;

        std::borrow::Cow::Owned(val_buf)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        assert_eq!(
            bytes.len(),
            34,
            "Unable to decode EDs: invalid number of bytes provider"
        );

        let len = bytes[33];
        let val = BigUint::from_bytes_le(&bytes[0..len as usize]);
        let decimals = bytes[32];

        Self { val, decimals }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 34,
        is_fixed_size: true,
    };
}

#[cfg(test)]
mod tests {
    use ic_stable_structures::Storable;

    use super::EDs;

    #[test]
    fn encoding_works_fine() {
        let a = EDs::f0_2(7);
        let a1 = EDs::from_bytes(a.to_bytes());
        assert_eq!(a, a1);

        let b = EDs::one(8);
        let b1 = EDs::from_bytes(b.to_bytes());
        assert_eq!(b, b1);

        let c = EDs::from((u128::MAX, 31));
        let c1 = EDs::from_bytes(c.to_bytes());
        assert_eq!(c, c1);

        let d = EDs::from((u64::MAX, 31));
        let d1 = EDs::from_bytes(d.to_bytes());
        assert_eq!(d, d1);
    }
}
