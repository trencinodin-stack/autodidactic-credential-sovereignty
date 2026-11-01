# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, circuit under-constraint flaw, formal proof unsoundness, or execution-membrane logic bypass affecting `autodidactic-credential-sovereignty`, please **do not open a public issue**.

Submit reports through either of the following private channels:

1. **GitHub Private Vulnerability Reporting (Recommended):**
   * Go to the **Security** tab of this repository (`trencinodin-stack/autodidactic-credential-sovereignty`).
   * Click **Report a vulnerability**.
   * Fill out the form to submit a private report directly to maintainers.

2. **Email:**
   * Contact us at **security@arcstoneos.com**.

---

## What to Include

Please include, where applicable:
- A concise description of the vulnerability, circuit constraint gap, or formal proof unsoundness.
- Minimal reproduction steps, unit test (`cargo test`), Coq script, or Circom witness trace demonstrating the issue.
- The affected domain suite, release tag (`v1.0.0`), or git commit hash anchored under `A-77-DELTA-SHIELD-LOCKED`.
- Whether the issue involves:
  * ACS-1 JSON Schema validation or bounding discrepancies;
  * ACS-2 `#![no_std]` heap allocation leakage ($M_{alloc} > 0$) or target cross-compilation harness defects;
  * ACS-3 Coq theorem unsoundness, hidden axiom imports, or logical premise inconsistency;
  * ACS-4 Circom constraint under-specification (e.g., missing `<==` / `===` bounds, unconstrained signals, or Poseidon preimage collisions in `dae_verifier.circom`);
  * Cryptographic challenge nonce reuse or freshness invalidation;
  * POSIX status code identifier mapping mismatch per **ARC-ERR-2026-001**;
  * Violation of `#![no_std]` or `#![deny(unsafe_code)]` safety guarantees.

---

## Upstream Core Immutability & Errata Protocol

`autodidactic-credential-sovereignty` serves as the upstream public reference credentialing and assurance substrate operating under a strict Read-Only Locked Protocol Policy ($C_{ops} = 0$).

- **No Unilateral Parameter Mutation:** Reporting a defect does not authorize altering baseline thresholds (e.g., $\le 450\,\text{ns}$ quant latency, $\le 30\,\text{ms}$ avionics recovery, $\le 50$ tick clock drift), POSIX status identifiers, or lattice dominance hierarchy without a formal update to **ARC-ERR-2026-001** (Master Canonical Errata, DOI `10.5281/zenodo.23069559`).
- **Preservation of Staged Semantics:** Fixes must preserve the separation between upstream architectural ingress constraints and downstream domain state-vector predicate evaluations.

---

## Scope & Bounded Claims

`autodidactic-credential-sovereignty` is a normative reference benchmark and autograding framework. Its assurance claims are governed strictly by the trust boundaries detailed in [`LIMITATIONS.md`](./LIMITATIONS.md).

The repository explicitly does **not** claim:
- Mechanized compiler extraction between Coq mathematical models and compiled `#![no_std]` Rust binaries;
- Physical hardware root of trust (TPM, SGX, or SEV-SNP) authentication of runtime telemetry feeding the Groth16 circuit;
- Cycle-accurate physical hardware execution on non-simulated clocks within CI runners;
- Universal identity attribution or candidate anti-plagiarism verification beyond cryptographic challenge-response attestation.

---

## Response Timeline

We will acknowledge receipt within **48 hours** and coordinate private technical resolution per our responsible disclosure policy.
