use ark_curve25519::Fq;

use crate::G2Type;

// TODO: Bellperson stuff

// TODO: G2Type::Fq is not, in fact, the pairing field
prime_field!("curve25519", Fq, G2Type::Fq);