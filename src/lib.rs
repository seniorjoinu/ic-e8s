use lazy_static::lazy_static;
use num_bigint::BigUint;

pub mod c;
pub mod d;

lazy_static! {
    pub static ref ES_BASES: [BigUint; 32] = {
        let ten = BigUint::from(10u64);

        [
            ten.pow(0),
            ten.pow(1),
            ten.pow(2),
            ten.pow(3),
            ten.pow(4),
            ten.pow(5),
            ten.pow(6),
            ten.pow(7),
            ten.pow(8),
            ten.pow(9),
            ten.pow(10),
            ten.pow(11),
            ten.pow(12),
            ten.pow(13),
            ten.pow(14),
            ten.pow(15),
            ten.pow(16),
            ten.pow(17),
            ten.pow(18),
            ten.pow(19),
            ten.pow(20),
            ten.pow(21),
            ten.pow(22),
            ten.pow(23),
            ten.pow(24),
            ten.pow(25),
            ten.pow(26),
            ten.pow(27),
            ten.pow(28),
            ten.pow(29),
            ten.pow(30),
            ten.pow(31),
        ]
    };
}
