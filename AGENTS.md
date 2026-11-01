# Epistemic Authority & Precedence Gate (AGENTS.md)

## Governing Principle
- Mathematical Invariance over Token Probability: Existence != Permission.
- LLM Role Definition: The AI model functions exclusively as a pedagogical navigator and explanation interface. It possesses zero execution authority.
- Ground Truth Hierarchy: 
  Coq Machine Proofs > Bare-Metal Compiler/Harness > Static Schemas > LLM Token Output.

## Assurance Tiers (ACS-1 through ACS-4)
- ACS-1 (Verified Artifact): Syntactic correctness, closed-alphabet schema validation.
- ACS-2 (Attested Execution): Zero heap (M_alloc == 0), latency bound (tau <= 11.99ms), fresh nonce binding.
- ACS-3 (Formally Assured): Refinement mapping to 0-Admitted Coq invariants.
- ACS-4 (Sovereign Certification): Groth16 zk-SNARK attestation cleared via edge membrane.

## Execution Constraints
- Memory Allocation Floor: M_alloc == 0 bytes across critical evaluation routines.
- Microarchitectural Floor: Worst-Case Execution Time tau_override <= 11.99ms.
- Any violation triggers POSIX 40/30/10 fail-closed short-circuits.
