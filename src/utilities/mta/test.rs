use crate::utilities::mta::range_proofs::tests::generate_init;
use crate::utilities::mta::{MessageA, MessageB};
use curv::elliptic::curves::{Curve, Scalar};


#[cfg(test)]
#[macro_export]
macro_rules! test_for_all {
    ([$($attrs:tt)*] $fn: ident =>) => {};
    ([$($attrs:tt)*] $fn: ident => $inst_name: ident = $inst:path, $($rest: tt)*) => {
        paste::paste!{
            #[test]
            $($attrs)*
            fn [<$fn _$inst_name>]() {
                $fn::<$inst>()
            }
        }
        crate::test_for_all!([$($attrs)*] $fn => $($rest)*);
    };
}


#[cfg(test)]
#[macro_export]
macro_rules! test_for_all_curves {
    (#[should_panic] $fn: ident) => {
        crate::test_for_all_curves!([#[should_panic]] $fn);
    };
    ($fn: ident) => {
        crate::test_for_all_curves!([] $fn);
    };
    ([$($attrs:tt)*] $fn: ident) => {
        crate::test_for_all!{[$($attrs)*] $fn =>
            secp256k1 = curv::elliptic::curves::Secp256k1,
            p256 = curv::elliptic::curves::Secp256r1,
            ed25519 = curv::elliptic::curves::Ed25519,
            ristretto = curv::elliptic::curves::Ristretto,
            bls12_381_1 = curv::elliptic::curves::Bls12_381_1,
            bls12_381_2 = curv::elliptic::curves::Bls12_381_2,
        }
    };
}


#[cfg(test)]
test_for_all_curves!(test_mta);
fn test_mta<E: Curve>() {
    let alice_input = Scalar::<E>::random();
    let (dlog_statement, ek_alice, dk_alice) = generate_init();
    let bob_input = Scalar::<E>::random();
    let (m_a, _) = MessageA::a(&alice_input, &ek_alice, &[dlog_statement.clone()]);
    let (m_b, beta, _, _) = MessageB::b(&bob_input, &ek_alice, m_a, &[dlog_statement]).unwrap();
    let alpha = m_b
        .verify_proofs_get_alpha(&dk_alice, &alice_input)
        .expect("wrong dlog or m_b");

    let left = alpha.0 + beta;
    let right = alice_input * bob_input;
    assert_eq!(left, right);
}
