### Temporary Repository

This is just a test repository based on [this ECDSA branch](https://github.com/tmpfs/multi-party-ecdsa/tree/curv-0.8)
to demonstrate an issue in the implementation of MtA (Multiplicative to Additive) share conversion protocol
when ed25519 is used as the underlying curve. As it can be seen in the other module, it works fine with secp256k1 curve.

To see the error, run `cargo test`, the output would be like this:

```python
test secp256k1::utilities::mta::test::test_mta ... ok
test ed25519::utilities::mta::test::test_mta ... FAILED
test secp256k1::utilities::mta::range_proofs::tests::alice_zkp ... ok
test ed25519::utilities::mta::range_proofs::tests::bob_zkp ... FAILED
test ed25519::utilities::mta::range_proofs::tests::alice_zkp ... FAILED
test secp256k1::utilities::mta::range_proofs::tests::bob_zkp ... ok
```
