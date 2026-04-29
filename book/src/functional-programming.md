# Functional Programming

Syma embraces functional programming. Higher-order functions like `Map`, `Fold`, and `Select` are central.

## Map — Apply to Every Element

The `/@` operator is syntactic sugar for `Map`:

```syma
sq[x_] := x^2;
Map[sq, {1, 2, 3, 4, 5}]    (* => {1, 4, 9, 16, 25} *)

(* Using /@ operator *)
Sqrt /@ {1, 4, 9, 16, 25}   (* => {1, 2, 3, 4, 5} *)

(* With pure functions *)
#^2 & /@ {1, 2, 3, 4, 5}    (* => {1, 4, 9, 16, 25} *)
```

## Fold — Reduce to a Single Value

`Fold[f, init, list]` applies `f` cumulatively from left to right:

```syma
Fold[Plus, 0, {1, 2, 3, 4, 5}]         (* => 15 *)

(* Without initial value — uses first element *)
Fold[Plus, {1, 2, 3, 4, 5}]            (* => 15 *)

(* Product *)
prod[a_, b_] := a * b;
Fold[prod, 1, {1, 2, 3, 4, 5}]         (* => 120 *)
```

## Select — Filter by Predicate

```syma
positive[x_] := x > 0;
Select[{-3, -1, 0, 2, 5, -4}, positive]   (* => {2, 5} *)

even[x_] := x / 2 * 2 == x;
Select[{1, 2, 3, 4, 5, 6}, even]          (* => {2, 4, 6} *)
```

## Scan — Execute Side Effects

Like `Map` but returns `Null`:

```syma
Print /@ {1, 2, 3}
(* prints: 1, 2, 3 *)
```

## Nest — Repeated Application

```syma
Nest[# + 1 &, 0, 10]   (* => 10 *)

(* Repeated squaring *)
Nest[#^2 &, 2, 3]      (* => 256, i.e. 2^2^2^2 *)
```

## Combinators

```syma
(* Compose: apply functions in sequence *)
Compose[f, g, h][x]     (* => f[g[h[x]]] *)
```

## Putting It Together

```syma
(* Sum of squares of even numbers *)
evens = Select[Range[10], even];         (* {2, 4, 6, 8, 10} *)
squares = #^2 & /@ evens;               (* {4, 16, 36, 64, 100} *)
Total[squares]                           (* => 220 *)

(* All in one line: *)
Total[#^2 & /@ Select[Range[10], #/2*2 == # &]]
```
