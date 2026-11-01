
# Autodidactic Credential Sovereignty (ACS)
### Sovereign, High-Assurance Technical Credentialing Substrate

Autodidactic Credential Sovereignty (ACS) establishes a universal, domain-agnostic standard that decouples technical verification and credentialing from legacy institutional gatekeepers. ACS anchors skill attainment directly in machine-checkable mathematical proofs, deterministic bare-metal test execution, and zero-knowledge cryptographic commitments.

---

## The 4-Tier Credential Assurance Model

1. **ACS-1 (Verified Artifact):** Syntactic adherence to domain pedagogy schemas (`schema/*_pedagogy.json`), governing problem formulation and physical invariants.
2. **ACS-2 (Attested Execution):** Deterministic execution in a `#![no_std]` bare-metal test harness enforcing zero dynamic heap allocation (`M_alloc == 0`), bounded worst-case execution time (WCET), and execution nonces.
3. **ACS-3 (Formally Assured):** Machine-checked invariant verification via Coq proof checker (`coqc`) with zero `Axiom` or `Admitted.` escapes, guaranteeing fail-closed safety properties.
4. **ACS-4 (Sovereign Certification):** Generation of a Groth16 zero-knowledge proof (`zk/dae_verifier.circom`) cryptographically binding the binary digest and challenge nonce to establish non-replayable, privacy-preserving credential sovereignty.

---

## Multi-Domain Assurance Suites

The ACS substrate concurrently evaluates four specialized assurance suites across heterogeneous engineering domains:

| Domain | Key Invariants & Physical Constraints | Formal Verification (`ACS-3`) | Harness Status (`ACS-2`) |
| :--- | :--- | :--- | :--- |
| **Core Substrate** | Poset lattice fail-closed supremum dominance; zero unbounded dynamic allocations | `proofs/dae_invariants.v` | `#![no_std]` Lib Harness |
| **Quantitative Finance** | Max 450 ns tick-to-trade latency; 150 bps drawdown ceiling; balance conservation | `domains/quant-finance/proofs/` | `#![no_std]` Quant Harness |
| **Aerospace Avionics** | Max 30 ms attitude recovery window; zero dynamic heap in flight loop; telemetry limits | `domains/aerospace-avionics/proofs/` | `#![no_std]` Avionics Harness |
| **Cryptographic Protocols** | Inductive reentrancy mutual exclusion; anti-replay nonces; exact supply conservation | `domains/crypto-protocols/proofs/` | `#![no_std]` Crypto Harness |

---

## Repository Structure

```text
autodidactic-credential-sovereignty/
├── .github/workflows/
│   └── substrate-ci.yml             # Automated ACS-2/ACS-3 CI autograder
├── domains/
│   ├── aerospace-avionics/          # Flight recovery & memory invariants
│   ├── crypto-protocols/            # Reentrancy, replay & supply conservation
│   └── quant-finance/               # Ultra-low latency & drawdown clamping
├── harness/                         # Core #![no_std] execution harness
├── proofs/                          # Core Poset dominance machine-checked proofs
├── schema/                          # JSON schema definitions for domain pedagogy
├── zk/                              # Circom Groth16 circuits for ACS-4 attestation
├── AGENTS.md                        # Pedagogical AI agent ingress guardrails
├── llms.txt                         # Structured machine-readable substrate context
└── README.md
```

## Quickstart & Local Verification

### Prerequisites

- **Rust Toolchain:** `rustup default stable` (targets: `thumbv7em-none-eabihf`, `x86_64-unknown-none`)
- **Coq Proof Checker:** `coqc` 8.16+ (with `Coq.micromega.Lia` support)

### Running Verifications Locally

1. **Verify All Formal Proofs (ACS-3):**

   ```bash
   coqc proofs/dae_invariants.v
   coqc domains/quant-finance/proofs/quant_invariants.v
   coqc domains/aerospace-avionics/proofs/avionics_invariants.v
   coqc domains/crypto-protocols/proofs/crypto_invariants.v
   ```

2. **Run Bare-Metal Execution Harnesses (ACS-2):**

   ```bash
   cargo test --manifest-path harness/Cargo.toml --release
   cargo test --manifest-path domains/quant-finance/harness/Cargo.toml --release
   cargo test --manifest-path domains/aerospace-avionics/harness/Cargo.toml --release
   cargo test --manifest-path domains/crypto-protocols/harness/Cargo.toml --release
   ```

## Onboarding a New Domain Suite

To add a 5th engineering domain (e.g., `biomedical-telemetry`, `robotics-kinematics`, or `satellite-constellations`):

1. **Create the Domain Directory Structure:**

   ```bash
   mkdir -p domains/<domain-name>/{harness/src,proofs,schema}
   ```

2. **Define the Pedagogy Schema (`ACS-1`):**

   Create `domains/<domain-name>/schema/<domain>_pedagogy.json` defining domain-specific invariant thresholds, bounded variables, and failure categories.

3. **Formulate Formal Proofs (`ACS-3`):**

   Write `domains/<domain-name>/proofs/<domain>_invariants.v`. Ensure every invariant theorem is discharged without `Admitted.` or `Axiom`.

4. **Implement Bare-Metal Harness (`ACS-2`):**

   Add a `Cargo.toml` and `#![no_std]` `src/lib.rs` under `domains/<domain-name>/harness/`. Implement positive and boundary tests for each failure mode.

5. **Register Domain in CI Autograder:**

   Append the compilation and test step to `.github/workflows/substrate-ci.yml`:

   ```yaml
   - name: Verify <Domain Name> Domain
     run: |
       coqc domains/<domain-name>/proofs/<domain>_invariants.v
       cargo test --manifest-path domains/<domain-name>/harness/Cargo.toml --release
   ```

6. **Commit & Push:** Submit a branch or push to `main` to trigger verification across the entire suite.
