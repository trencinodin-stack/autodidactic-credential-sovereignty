
# Autodidactic Credential Sovereignty (ACS)
### Sovereign, High-Assurance Technical Credentialing Substrate & Reference Benchmark

Autodidactic Credential Sovereignty (ACS) is an experimental, multi-domain reference architecture and autonomous assessment substrate designed to evaluate technical competence via formal logic, bare-metal memory invariants, and zero-knowledge commitments.

ACS explores an alternative credentialing model: anchoring engineering claims directly in machine-checked mathematical proofs, deterministic bare-metal test execution, and zero-knowledge telemetry commitments rather than self-reported assertions or institutional gatekeeping.

---

## Academic Provenance & Cryptographic Anchor

This repository is the canonical open reference realization of the Deterministic Admissibility Engineering (DAE) academic specification suite (`ARC-SPEC-2026-EDU-01..09`), anchored within the Arcstone research ecosystem.

* **Master Hash Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Canonical Specification Root:** [admissibilityscience.com](https://admissibilityscience.com)
* **Academic Realization Suite (Zenodo DOI):** [10.5281/zenodo.23371581](https://doi.org/10.5281/zenodo.23371581)
* **Layer 0 Primary Authority (Zenodo DOI):** [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852)
* **Normative Errata Specification:** [10.5281/zenodo.23069559](https://doi.org/10.5281/zenodo.23069559) (`ARC-ERR-2026-001`)

---

## Sovereign Governance & Engineering Standards

The foundational doctrine, thermodynamic invariants, and zero-knowledge capability proof standards governing this repository are codified in the canonical charter:

- 📜 **[The Epistemic Elevation Charter](CHARTER.md)** (`ARC-CHARTER-2026-ELEV01`)  
  *Sovereign Standards for High-Consequence Computation & Machine-Verified Engineering*  
  - **Canon I (Thermodynamic Grounding):** Static memory bounds ($M_{\text{alloc}} = 0$, $S_{\max} \le 4096\text{ B}$) and cycle-accurate temporal clamping ($\tau_{\text{override}} \le 11.99\text{ ms}$).
  - **Canon II (Constructive Verification):** Complete elimination of unearned authority and unproven lemmas (`Print Assumptions` $\equiv \emptyset$, `POSIX HALT 0`).
  - **Canon III (The Fail-Closed Membrane):** Actuator airlock enforcement where denial guarantees absolute state conservation ($\mathcal{D}(a) = \text{DENY} \implies \Delta_{\text{external}}(a) = 0$).
  - **Canon IV (Functorial Order Dynamics):** 5-element join-semilattice mapping into kernel-level POSIX execution precedence ($40 \succ 32 \succ 30 \succ 12 \succ 0$).
  - **Canon V (Artifact-Bound Assurance):** Zero-knowledge capability proofs ($\pi_{\text{DAE}}$) and zero operational maintenance overhead ($C_{\text{ops}} = 0$).

---

## The 4-Tier Credential Assurance Model

1. **ACS-1 (Specification Schemas):** Syntactic adherence to domain schemas (`schema/*_pedagogy.json`), governing problem formulation, unit conventions, and physical/operational bounds.
2. **ACS-2 (Attested Execution):** Deterministic execution in `#![no_std]` bare-metal test harnesses enforcing zero dynamic heap allocation (`M_alloc == 0`), bounded execution timing, and anti-replay nonces.
3. **ACS-3 (Formally Assured):** Machine-checked invariant verification via the Coq proof checker (`coqc` 8.16+) with zero unproven lemmas (`zero Admitted.`) and explicit axiom inspection (`Print Assumptions`).
4. **ACS-4 (Cryptographic Attestation):** Circom 2.1.6 Groth16 circuit verification (`zk/dae_verifier.circom`) enforcing R1CS rank-1 equality constraints (`<==`) over Poseidon-hashed telemetry commitments bound to challenge nonces.

---

## Multi-Domain Assurance Suites

The ACS substrate evaluates four specialized assurance suites across heterogeneous engineering domains:

| Domain | Key Invariants & Physical Constraints | Formal Verification (`ACS-3`) | Harness Status (`ACS-2`) |
| :--- | :--- | :--- | :--- |
| **Core Substrate** | Poset lattice fail-closed supremum dominance; zero dynamic allocations | `proofs/dae_invariants.v` | `#![no_std]` Core Harness |
| **Quantitative Finance** | Max 450 ns tick-to-trade latency; 150 bps drawdown ceiling; balance conservation | `domains/quant-finance/proofs/` | `#![no_std]` Quant Harness |
| **Aerospace Avionics** | Max 30 ms attitude recovery window; zero dynamic heap in flight loop | `domains/aerospace-avionics/proofs/` | `#![no_std]` Avionics Harness |
| **Cryptographic Protocols** | Inductive reentrancy mutual exclusion; anti-replay nonces; exact supply conservation | `domains/crypto-protocols/proofs/` | `#![no_std]` Crypto Harness |

---

## Trust Boundaries & Limitations

Before reviewing or adopting this framework, please consult **[`LIMITATIONS.md`](./LIMITATIONS.md)** for explicit documentation of:

* The model-to-code gap (Coq formal models vs. Rust implementations).
* The telemetry provenance boundary (zk-SNARK circuit constraint satisfaction vs. hardware root of trust).
* Simulated logical clock ticks vs. cycle-accurate target hardware measurements.

---

## Operational Governance Policy ($C_{ops} = 0$)

This repository is maintained under a frozen reference standard model:

* **Zero Operational Overhead ($C_{ops} = 0$):** Inbound Pull Requests, Issues, and Discussions are disabled upstream to avoid maintenance fatigue and maintain an immutable reference baseline.
* **Autonomous Forking:** Downstream researchers, labs, and educators are encouraged to fork this repository, extend or adapt domains, and execute the autograding pipeline on their own compute infrastructure.

---

## Repository Structure

```text
autodidactic-credential-sovereignty/
├── .github/workflows/
│   └── substrate-ci.yml             # Automated ACS-2/ACS-3/ACS-4 CI autograder
├── domains/
│   ├── aerospace-avionics/          # Flight recovery & memory invariants
│   ├── crypto-protocols/            # Reentrancy, replay & supply conservation
│   └── quant-finance/               # Low latency & drawdown clamping
├── harness/                         # Core #![no_std] execution harness
├── proofs/                          # Core Poset dominance machine-checked proofs
├── schema/                          # JSON schema definitions for domain pedagogy
├── zk/                              # Circom Groth16 circuits for ACS-4 attestation
├── AGENTS.md                        # Pedagogical AI agent ingress guardrails
├── LIMITATIONS.md                   # Formal trust boundary & TCB documentation
├── llms.txt                         # Machine-readable context file
├── CHARTER.md                       # The Epistemic Elevation Charter
└── README.md
```

## Quickstart & Local Verification

### Prerequisites

**Rust Toolchain:** `rustup default stable` (targets: `thumbv7em-none-eabihf`, `x86_64-unknown-none`)

**Coq Proof Checker:** `coqc` 8.16+ (with `Coq.micromega.Lia` support)

**Circom & SnarkJS:** `circom` 2.1.6+ and node 20+

### Running Verifications Locally

**Verify All Formal Proofs (ACS-3):**

```bash
coqc proofs/dae_invariants.v
coqc domains/quant-finance/proofs/quant_invariants.v
coqc domains/aerospace-avionics/proofs/avionics_invariants.v
coqc domains/crypto-protocols/proofs/crypto_invariants.v
```

**Run Bare-Metal Execution Harnesses (ACS-2):**

```bash
cargo test --manifest-path harness/Cargo.toml --release
cargo test --manifest-path domains/quant-finance/harness/Cargo.toml --release
cargo test --manifest-path domains/aerospace-avionics/harness/Cargo.toml --release
cargo test --manifest-path domains/crypto-protocols/harness/Cargo.toml --release
```

**Compile ACS-4 Zero-Knowledge Circuit:**

```bash
npm install circomlib
circom zk/dae_verifier.circom --r1cs --wasm --sym -o zk/ -l node_modules
```

## Extending the Substrate (Downstream Forks)

To add a domain suite (e.g., biomedical-telemetry, robotics-kinematics) within your own fork:

**Create the Domain Directory Structure:**

```bash
mkdir -p domains/<domain-name>/{harness/src,proofs,schema}
```

**Define the Pedagogy Schema (ACS-1):**

Create `domains/<domain-name>/schema/<domain>_pedagogy.json` defining domain invariant thresholds, bounded variables, and failure categories.

**Formulate Formal Proofs (ACS-3):**

Write `domains/<domain-name>/proofs/<domain>_invariants.v`. Ensure every invariant theorem is discharged without `Admitted.` or hidden axioms.

**Implement Bare-Metal Harness (ACS-2):**

Add a `Cargo.toml` and `#![no_std]` `src/lib.rs` under `domains/<domain-name>/harness/`. Implement positive and boundary tests for each failure mode.

**Register Domain in CI Autograder:**

Append the domain steps to `.github/workflows/substrate-ci.yml`.
