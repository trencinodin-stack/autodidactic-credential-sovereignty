# Technical Specification, Normative Authority & Invariant Grounding

## 1. Normative Precedence & Authority Hierarchy

To prevent ambiguity during autonomous machine ingestion, agent-based reasoning, and formal third-party audit, authoritative priority across this repository flows strictly in the following order:

1. **Layer 0 Formal Proofs (`proofs/dae_invariants.v`):** Mechanized mathematical definitions evaluated without unproven axioms under Coq 8.18+.
2. **Deterministic Schemas (`schema/*_pedagogy.json`):** Machine-enforceable JSON constraints governing problem boundaries, units, and parameter envelopes.
3. **Bare-Metal Test Harnesses (`harness/`):** `#![no_std]` Rust reference execution vectors, property test assertions, and allocation boundary checks.
4. **Cryptographic Circuits (`zk/dae_verifier.circom`):** Groth16 R1CS zero-knowledge constraint verification for off-chain execution telemetry.
5. **Informative Documentation (`README.md`, `AGENTS.md`, `llms.txt`, `LIMITATIONS.md`):** High-level architectural descriptions, navigational guides, and trust-boundary disclosures.

---

## 2. Formal Invariant Definitions

### Primary Conservation Axiom (Invariant I1)
$$D(a) = \text{DENY} \implies \Delta_{\text{external}}(a) = 0$$

* **External State Boundary ($\Delta_{\text{external}}$):** Defined strictly as protected external side effects, application memory, actuator control registers, persistent balances, ledger state, and credential state vectors.
* **Observable Telemetry Exclusion:** Informational audit logging, cryptographic rejection telemetry, monotonic denial counters, and process return codes do **not** constitute external state mutation.
* **Complete Mediation:** In accordance with `#![no_std]` execution constraints, an authorization verdict of `DENY` short-circuits execution before invoking any external driver, hardware register, or state-mutating actuator.

---

## 3. Four-Tier Assurance & Enforcement Matrix

| Tier | Substrate | Primary Invariant | Verification Method | Enforcement Target |
| :--- | :--- | :--- | :--- | :--- |
| **ACS-1** | Declarative Schemas | Syntactic Admissibility | JSON Schema Draft 2020-12 | Parameter envelope validation |
| **ACS-2** | Bare-Metal Rust | $M_{\text{alloc}} = 0$, $S_{\max} \le 4096\text{ B}$ | `#![no_std]`, zero heap | Microarchitectural test harness |
| **ACS-3** | Coq Proof Corpus | Supremum Dominance | Axiom-Free Coq Proofs | Model-level state transition system |
| **ACS-4** | Circom zk-SNARK | Telemetry Attestation | Groth16 R1CS Circuits | Private witness constraint verification |

---

## 4. Status Code Mapping (ARC-ERR-2026-001)

All status codes represent deterministic internal operational states:

* **`POSIX 0` (PASS):** Nominal execution; invariant compliance; action authorized.
* **`POSIX 10` (FREEZE):** Internal runtime scheduling yield / soft hold outside the pre-ingress lattice.
* **`POSIX 12` (PWC):** Proportional Windowing Control active; queue rate throttling engaged.
* **`POSIX 30` (CORRUPT):** Framing corruption or buffer limit breach ($S > 4096\text{ B}$).
* **`POSIX 32` (REFUSAL):** In-band policy refusal or front-edge queue saturation overflow.
* **`POSIX 40` (BREACH):** Invariant breach, clock drift violation, or hardware circuit-breaker lockout.

---

## 5. Scope & Boundary Disclosures

* **Model-to-Code Correspondence:** The formal proofs in `proofs/` verify properties of the mathematical transition model. The `#![no_std]` Rust harness represents an executable reference implementation conforming to the same parameter boundaries; no automated mechanized extraction is claimed.
* **Simulated vs. Physical Clamps:** The temporal threshold ($\tau_{\text{override}} \le 11.99\,\text{ms}$) and vector clock drift limit ($< 50\,\text{ticks}$) are verified against synthetic CI test harnesses and simulated clock frameworks. Hard real-time execution guarantees on target silicon remain contingent on hardware-specific WCET analysis.
