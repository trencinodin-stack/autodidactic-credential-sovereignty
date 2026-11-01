(** Aerospace Avionics Invariants: Recovery Window Clamping & In-Flight State **)
Require Import Coq.Init.Nat.
Require Import Coq.Arith.Arith.
Require Import Coq.micromega.Lia.

Definition state_recovered (recovery_time bound : nat) : Prop :=
  recovery_time <= bound.

Theorem recovery_bounded :
  forall (r bound extra : nat),
    state_recovered r bound ->
    r <= bound + extra.
Proof.
  intros r bound extra H.
  unfold state_recovered in H.
  lia.
Qed.

Theorem fail_safe_override :
  forall (raw_signal default_signal error_flag : nat),
    (error_flag = 1 -> default_signal = 0) ->
    (error_flag = 1) ->
    default_signal = 0.
Proof.
  intros raw_signal default_signal error_flag H1 H2.
  apply H1.
  exact H2.
Qed.
