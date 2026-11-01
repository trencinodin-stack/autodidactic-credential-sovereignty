(** Quantitative Finance Invariants: Balance Conservation & Drawdown Bounds **)
Require Import Coq.Init.Nat.
Require Import Coq.Arith.Arith.
Require Import Coq.micromega.Lia.

Definition balance_conserved (initial inflow outflow final : nat) : Prop :=
  initial + inflow = final + outflow.

Theorem balance_conservation_invariant :
  forall init inf out fin,
    balance_conserved init inf out fin ->
    (init + inf >= out) /\ fin = (init + inf) - out.
Proof.
  intros init inf out fin H.
  unfold balance_conserved in H.
  split.
  - lia.
  - lia.
Qed.

Theorem drawdown_ceiling :
  forall peak current max_loss,
    current <= peak ->
    (peak - current <= max_loss) ->
    peak - max_loss <= current.
Proof.
  intros.
  lia.
Qed.
