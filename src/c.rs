use std::{
    borrow::Cow,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use candid::{CandidType, Nat};
use ic_stable_structures::{storable::Bound, Storable};
use num_bigint::BigUint;
use serde::Deserialize;

use crate::{d::EDs, ES_BASES};

pub type E8s = ECs<8>;

/// Fixed-point decimals with primitive math (+-*/) implemented correctly
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ECs<const DECIMALS: usize> {
    pub val: BigUint,
}

impl<const D: usize> ECs<D> {
    pub fn new(val: BigUint) -> Self {
        Self { val }
    }

    pub fn base() -> &'static BigUint {
        if D > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { ES_BASES.get(D).unwrap_unchecked() }
    }

    pub fn base_d(decimals: u8) -> &'static BigUint {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { ES_BASES.get(decimals as usize).unwrap_unchecked() }
    }

    pub fn zero() -> Self {
        Self::new(BigUint::ZERO)
    }

    pub fn one() -> Self {
        Self {
            val: Self::base().clone(),
        }
    }

    pub fn f0_1() -> Self {
        Self::new(Self::base() / BigUint::from(10u64))
    }

    pub fn f0_2() -> Self {
        Self::new(Self::base() / BigUint::from(5u64))
    }

    pub fn f0_25() -> Self {
        Self::new(Self::base() / BigUint::from(4u64))
    }

    pub fn f0_3() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(10u64))
    }

    pub fn f0_33() -> Self {
        Self::new(Self::base() / BigUint::from(3u64))
    }

    pub fn f0_4() -> Self {
        Self::new(Self::base() * BigUint::from(2u64) / BigUint::from(5u64))
    }

    pub fn f0_5() -> Self {
        Self::new(Self::base() / BigUint::from(2u64))
    }

    pub fn f0_6() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(5u64))
    }

    pub fn f0_67() -> Self {
        Self::new(Self::base() * BigUint::from(2u64) / BigUint::from(3u64))
    }

    pub fn f0_7() -> Self {
        Self::new(Self::base() * BigUint::from(7u64) / BigUint::from(10u64))
    }

    pub fn f0_75() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(4u64))
    }

    pub fn f0_8() -> Self {
        Self::new(Self::base() * BigUint::from(4u64) / BigUint::from(5u64))
    }

    pub fn f0_9() -> Self {
        Self::new(Self::base() * BigUint::from(9u64) / BigUint::from(10u64))
    }

    pub fn two() -> Self {
        Self::new(Self::base() * BigUint::from(2u64))
    }

    pub fn sqrt(&self) -> Self {
        let base = Self::base();
        let whole = &self.val / base;
        let sqrt_whole = whole.sqrt();

        Self::new(sqrt_whole * base)
    }

    pub fn to_dynamic(self) -> EDs {
        EDs::new(self.val, D as u8)
    }

    pub fn to_decimals<const D1: usize>(self) -> ECs<D1> {
        if D1 == D {
            return ECs::<D1>::new(self.val);
        }

        let (dif, mul) = if D > D1 {
            (D - D1, false)
        } else {
            (D1 - D, true)
        };

        let base = Self::base_d(dif as u8);

        if mul {
            ECs::<D1>::new(self.val * base)
        } else {
            ECs::<D1>::new(self.val / base)
        }
    }
}

impl<const D: usize> Display for ECs<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = ECs::<D>::base();

        f.write_str(&format!("{}.{}", &self.val / base, &self.val % base))
    }
}

impl<const D: usize> Add for &ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val + &rhs.val)
    }
}

impl<const D: usize> Add for ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<const D: usize> Add<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: &ECs<D>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<const D: usize> Add<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: ECs<D>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<const D: usize> AddAssign<&ECs<D>> for ECs<D> {
    fn add_assign(&mut self, rhs: &ECs<D>) {
        self.val.add_assign(&rhs.val)
    }
}

impl<const D: usize> AddAssign for ECs<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl<const D: usize> Sub for &ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val - &rhs.val)
    }
}

impl<const D: usize> Sub for ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<const D: usize> Sub<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: &ECs<D>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<const D: usize> Sub<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: ECs<D>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<const D: usize> SubAssign<&ECs<D>> for ECs<D> {
    fn sub_assign(&mut self, rhs: &ECs<D>) {
        self.val.sub_assign(&rhs.val)
    }
}

impl<const D: usize> SubAssign for ECs<D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl<const D: usize> Mul for &ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val * &rhs.val / ECs::<D>::base())
    }
}

impl<const D: usize> Mul for ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl<const D: usize> Mul<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: &ECs<D>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<const D: usize> Mul<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: ECs<D>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<const D: usize> MulAssign<&ECs<D>> for ECs<D> {
    fn mul_assign(&mut self, rhs: &ECs<D>) {
        self.val = &self.val * &rhs.val / ECs::<D>::base()
    }
}

impl<const D: usize> MulAssign for ECs<D> {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl<const D: usize> Div for &ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val * ECs::<D>::base() / &rhs.val)
    }
}

impl<const D: usize> Div for ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl<const D: usize> Div<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: &ECs<D>) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<const D: usize> Div<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: ECs<D>) -> Self::Output {
        self.div(&rhs)
    }
}

impl<const D: usize> DivAssign<&ECs<D>> for ECs<D> {
    fn div_assign(&mut self, rhs: &ECs<D>) {
        self.val = &self.val * ECs::<D>::base() / &rhs.val;
    }
}

impl<const D: usize> DivAssign for ECs<D> {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}

impl<const D: usize> CandidType for ECs<D> {
    fn _ty() -> candid::types::Type {
        Nat::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        Nat::idl_serialize(&Nat(self.val.clone()), serializer)
    }
}

impl<'de, const C: usize> Deserialize<'de> for ECs<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ECs::new(Nat::deserialize(deserializer)?.0))
    }
}

impl<const D: usize> From<u64> for ECs<D> {
    fn from(value: u64) -> Self {
        Self::new(BigUint::from(value))
    }
}

impl<const D: usize> From<u128> for ECs<D> {
    fn from(value: u128) -> Self {
        Self::new(BigUint::from(value))
    }
}

impl<const D: usize> Storable for ECs<D> {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.val.to_bytes_le())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self::new(BigUint::from_bytes_le(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: D as u32,
        is_fixed_size: true,
    };
}
