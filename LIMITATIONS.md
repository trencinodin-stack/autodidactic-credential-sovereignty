# Trust Boundaries, Scope & Known Limitations

This document explicitly delineates the Trusted Computing Base (TCB), formal guarantees, and operational bounds of the Autodidactic Credential Sovereignty (ACS) reference implementation.

### 1. Assurance Discontinuity & Model Separation: Constrained Realization vs. Mechanized Refinement
- **Deliberate Paradigm Choice:** The boundary between the abstract inductive definitions in `proofs/` (Coq) and the runtime execution harnesses in `#![no_std]` Rust is a **Constrained Realization by Design**, not an unfinished mechanized refinement pipeline. Full mechanized compiler extraction (e.g., CompCert, CakeML, or Creusot) was evaluated and intentionally omitted to eliminate continuous proof-maintenance overhead and preserve the foundational invariant of **Verification-Cost Inversion (VCI)**: amortizing formal assurance without recurring operational drag ($C_{\text{ops}} = 0$).
- **Bounded Boundary Containment:** Rather than pursuing an infinite proof recursion down to microarchitectural silicon, deterministic runtime safety is guaranteed via physical and structural clamps:
  - Zero-heap allocation ($M_{\text{alloc}} = 0$) preventing fragmentation and dynamic allocator corruption.
  - Bounded static memory envelopes ($S_{\text{max}} \le 4096\,\text{B}$) enforcing compile-time stack limits.
  - Hard real-time evaluation windows ($\tau_{\text{override}} \le 11.99\,\text{ms}$) halting algorithmic stalls at line rate.
  - Actuator decoupling where non-authorization guarantees null side effects: $D(a) = \text{DENY} \implies \Delta_{\text{external}}(a) = 0$.
- **Traceability & Roadmap:** Semantic conformance between specification invariants and Rust harnesses is currently enforced via uniform specification identifiers and static harness assertions. Future integration of translational bridges (e.g., Aeneas MIR-to-Lean extraction) will serve as an optional, offline static analysis layer rather than a dynamic operational dependency.
- **Governing Doctrine:** For the complete theoretical treatment and operational bounds, see:
  - Research Monograph: *The Compiler Trap: Why Real-World AI Safety Requires Bounded Containment, Not Infinite Proof Recursion* ([Substack Monograph](https://founderarcstoneos.substack.com/p/the-compiler-trap-why-real-world))
  - Master Keystone: `ARC-PUB-2026-DAE-01` ([Zenodo DOI: 10.5281/zenodo.23967282](https://doi.org/10.5281/zenodo.23967282))
  - Epistemic Hash Anchor: `A-77-DELTA-SHIELD-LOCKED`

### 2. Telemetry Provenance & Attestation Model
- **ACS-4 ZK Circuit Scope:** The Groth16 circuit (`zk/dae_verifier.circom`) proves that a prover possesses valid witness inputs (latency, allocation, clock drift) that satisfy the R1CS inequality constraints and match the Poseidon commitment bound to a freshness nonce.
- **Hardware Root of Trust:** The reference circuit does not include a hardware-attested measurement environment (e.g., TPM, SGX, or SEV-SNP quotes). In this baseline reference specification, telemetry is treated as prover-witness input. Production physical deployments require binding the commitment to an authenticated hardware enforcer.

### 3. Execution Environment & Timing
- **Simulation vs. Target Hardware:** Performance bounds (≤ 450 ns tick-to-trade, ≤ 30 ms attitude recovery) represent domain-specified normative thresholds tested against logical simulated clock ticks in CI, not cycle-accurate physical bench measurements on flight hardware or dedicated FPGA NICs.
