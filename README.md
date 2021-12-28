### Temporary Repository

This is just a test repository based on [this ECDSA branch](https://github.com/tmpfs/multi-party-ecdsa/tree/curv-0.8)
to demonstrate an issue in the implementation of MtA (Multiplicative to Additive) share conversion protocol
when ed25519 is used as the underlying curve. As it can be seen in the other module, it works fine with secp256k1 curve.

To see the error, run `cargo test`, the output would be like this:

```python

running 18 tests
test utilities::mta::range_proofs::tests::alice_zkp_p256 ... ok
test utilities::mta::range_proofs::tests::alice_zkp_ristretto ... ok
test utilities::mta::range_proofs::tests::alice_zkp_bls12_381_2 ... ok
test utilities::mta::range_proofs::tests::alice_zkp_bls12_381_1 ... ok
test utilities::mta::range_proofs::tests::alice_zkp_secp256k1 ... ok
test utilities::mta::range_proofs::tests::alice_zkp_ed25519 ... ok
test utilities::mta::range_proofs::tests::bob_zkp_ristretto ... FAILED
test utilities::mta::range_proofs::tests::bob_zkp_ed25519 ... FAILED
test utilities::mta::test::test_mta_bls12_381_1 ... ok
test utilities::mta::test::test_mta_ed25519 ... FAILED
test utilities::mta::test::test_mta_p256 ... ok
test utilities::mta::test::test_mta_secp256k1 ... ok
test utilities::mta::test::test_mta_ristretto ... ok
test utilities::mta::test::test_mta_bls12_381_2 ... ok
test utilities::mta::range_proofs::tests::bob_zkp_secp256k1 ... ok
test utilities::mta::range_proofs::tests::bob_zkp_p256 ... ok
test utilities::mta::range_proofs::tests::bob_zkp_bls12_381_1 ... ok
test utilities::mta::range_proofs::tests::bob_zkp_bls12_381_2 ... ok

```
