# Trust Boundaries, Scope & Known Limitations

This document explicitly delineates the Trusted Computing Base (TCB), formal guarantees, and operational bounds of the Autodidactic Credential Sovereignty (ACS) reference implementation.

### 1. Assurance Discontinuity & Model Separation
* **ACS-3 (Coq) vs. ACS-2 (Rust):** The formal theorems in `proofs/` verify mathematical transition systems and lattice properties of the domain models. They do not utilize mechanized extraction (e.g., CompCert/Creusot) to compile directly into the `#![no_std]` Rust harnesses. The Rust harnesses serve as empirical conformance tests against the pedagogical schemas.
* **Traceability:** Conformance between schema invariants, harness assertions, and Coq models is currently maintained through uniform specification identifiers rather than an automated refinement proof.

### 2. Telemetry Provenance & Attestation Model
* **ACS-4 ZK Circuit Scope:** The Groth16 circuit (`zk/dae_verifier.circom`) proves that a prover possesses valid witness inputs (latency, allocation, clock drift) that satisfy the R1CS inequality constraints and match the Poseidon commitment bound to a freshness nonce.
* **Hardware Root of Trust:** The reference circuit does not include a hardware-attested measurement environment (e.g., TPM, SGX, or SEV-SNP quotes). In this baseline reference specification, telemetry is treated as prover-witness input. Production physical deployments require binding the commitment to an authenticated hardware enforcer.

### 3. Execution Environment & Timing
* **Simulation vs. Target Hardware:** Performance bounds ($\le 450\,\text{ns}$ tick-to-trade, $\le 30\,\text{ms}$ attitude recovery) represent domain-specified normative thresholds tested against logical simulated clock ticks in CI, not cycle-accurate physical bench measurements on flight hardware or dedicated FPGA NICs.
