(** Cryptographic Protocol Invariants: Reentrancy Locks & Supply Conservation **)
Require Import Coq.Init.Nat.
Require Import Coq.Arith.Arith.
Require Import Coq.micromega.Lia.

Inductive LockState : Type :=
  | Unlocked : LockState
  | Locked   : LockState.

Definition enter_lock (s : LockState) : option LockState :=
  match s with
  | Unlocked => Some Locked
  | Locked   => None
  end.

Theorem reentrancy_mutual_exclusion :
  forall s s',
    enter_lock s = Some s' ->
    s = Unlocked /\ s' = Locked.
Proof.
  intros s s' H.
  destruct s.
  - inversion H. auto.
  - inversion H.
Qed.

Theorem total_supply_conservation :
  forall a b transfer,
    transfer <= a ->
    (a - transfer) + (b + transfer) = a + b.
Proof.
  intros a b transfer H.
  lia.
Qed.
