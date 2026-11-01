(** Formal Machine-Checked Invariants for DAE / ACS **)
Require Import Coq.Init.Nat.
Require Import Coq.Lists.List.

Inductive PosetLevel : Type :=
  | PassLevel   : PosetLevel (* POSIX 0 *)
  | FreezeLevel : PosetLevel (* POSIX 10 *)
  | PwcLevel    : PosetLevel (* POSIX 12 *)
  | MemoryLevel : PosetLevel (* POSIX 30 *)
  | RefusalLevel: PosetLevel (* POSIX 32 *)
  | BreachLevel : PosetLevel (* POSIX 40 *).

Definition poset_val (p : PosetLevel) : nat :=
  match p with
  | PassLevel    => 0
  | FreezeLevel  => 10
  | PwcLevel     => 12
  | MemoryLevel  => 30
  | RefusalLevel => 32
  | BreachLevel  => 40
  end.

Definition poset_supremum (p1 p2 : PosetLevel) : PosetLevel :=
  if Nat.leb (poset_val p1) (poset_val p2) then p2 else p1.

Theorem breach_is_supremum : forall p : PosetLevel,
  poset_supremum p BreachLevel = BreachLevel.
Proof.
  intros p.
  destruct p; simpl; reflexivity.
Qed.

Theorem fail_closed_dominance : forall p1 p2 : PosetLevel,
  poset_val (poset_supremum p1 p2) >= poset_val p1.
Proof.
  intros p1 p2.
  unfold poset_supremum.
  destruct (Nat.leb (poset_val p1) (poset_val p2)) eqn:Hleb.
  - apply Nat.leb_le in Hleb. exact Hleb.
  - apply Nat.le_refl.
Qed.
