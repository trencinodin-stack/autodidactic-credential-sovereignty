# Autodidactic Credential Sovereignty (ACS)
### De-Institutionalized High-Assurance Systems Verification

Autodidactic Credential Sovereignty (ACS) decouples high-assurance systems engineering education and verification from academic intermediaries.

## The 4-Tier Credential Model
- **ACS-1 (Verified Artifact):** Syntactic adherence to `schema/domain_pedagogy_v1.json`.
- **ACS-2 (Attested Execution):** Bare-metal execution satisfying zero heap (`M_alloc == 0`) and execution limits (tau <= 11.99ms) bound to challenge nonce.
- **ACS-3 (Formally Assured):** Verification of `proofs/dae_invariants.v` via Coq with zero `Admitted.` statements.
- **ACS-4 (Sovereign Certification):** Generation of a Groth16 proof via `zk/dae_verifier.circom` binding challenge nonce to binary digest.

## Quickstart in Visual Studio Code
1. Extract this repository into your workspace root.
2. Open with VS Code: `code .`
3. Check Rust harness: `cargo test --manifest-path harness/Cargo.toml`
4. Verify Coq proofs: `coqc proofs/dae_invariants.v`
