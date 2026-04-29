# Mathematics

Syma provides mathematical functions and constants for numerical and symbolic computation.

## Constants

```syma
Pi    (* π ≈ 3.14159 *)
E     (* e ≈ 2.71828 *)
```

## Trigonometry

```syma
Sin[0]          (* => 0 *)
Sin[Pi / 6]     (* => 0.5 *)
Cos[0]          (* => 1 *)
Cos[Pi]         (* => -1 *)
Tan[Pi / 4]     (* => 1 *)

ArcSin[1]       (* => π/2 *)
ArcCos[0]       (* => π/2 *)
ArcTan[1]       (* => π/4 *)
```

## Exponentials and Logarithms

```syma
Exp[0]          (* => 1 *)
Exp[1]          (* => E ≈ 2.71828 *)
Log[1]          (* => 0 *)
Log[E]          (* => 1 *)
Log[100]        (* => ~4.605 *)
Log2[8]         (* => 3 *)
Log10[100]      (* => 2 *)
```

## Powers and Roots

```syma
Sqrt[144]       (* => 12 *)
Sqrt[2]         (* => 1.41421 *)
2^10            (* => 1024 *)
16^(1/2)        (* => 4 *)
```

## Numeric Functions

```syma
Abs[-42]        (* => 42 *)
Floor[3.7]      (* => 3 *)
Ceiling[3.2]    (* => 4 *)
Round[3.5]      (* => 4 *)
Max[3, 7, 2]    (* => 7 *)
Min[3, 7, 2]    (* => 2 *)
Mod[10, 3]      (* => 1 *)
GCD[12, 18]     (* => 6 *)
LCM[4, 6]       (* => 12 *)
Factorial[5]    (* => 120 *)
```

## Numeric Approximation

```syma
N[Pi]           (* => 3.14159 *)
```

## Symbolic Operations

```syma
(* Expand an expression *)
Expand[(x + 1)^3]    (* => 1 + 3x + 3x^2 + x^3 *)

(* Factor *)
Factor[x^2 - 1]      (* => (-1 + x)(1 + x) *)

(* Differentiate *)
D[x^3, x]            (* => 3 x^2 *)
D[Sin[x], x]         (* => Cos[x] *)
```

## Numerical Methods

Root finding with Newton's method:

```syma
f[x_] := x^2 - 2;
Nest[# - f[#] / (f[# + 0.001] - f[#]) * 0.001 &, 1.5, 10]
(* Approximates Sqrt[2] *)

(* Numerical integration (midpoint rule) *)
integrate[f_, a_, b_, n_] :=
    (b - a) / n * Total[Table[f[a + (i - 0.5) * (b - a) / n], {i, 1, n}]];
```

See the examples in `syma/examples/math/` for pi series approximations, Monte Carlo methods, and more.
