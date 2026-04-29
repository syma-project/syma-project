# Patterns and Rules

Pattern matching is Syma's most powerful feature. It replaces type checks, conditionals, and destructuring from other languages.

## The Blank `_`

`_` matches anything:

```syma
MatchQ[42, _]       (* => True *)
MatchQ["hello", _]   (* => True *)
```

## Named Blanks

`x_` matches anything and binds the value to `x`:

```syma
square[x_] := x^2
```

## Type-Constrained Blanks

`_Integer` matches only integers, `_Real` matches only reals, etc.:

```syma
f[x_Integer] := x * 2;
f[x_String] := StringLength[x];

f[5]           (* => 10 *)
f["hello"]     (* => 5 *)
f[3.14]        (* unevaluated — no match *)
```

## Rules: `->` and `:>`

A rule pairs a pattern with a replacement:

```syma
x_ -> x^2             (* immediate: x^2 is evaluated when rule is created *)
x_ :> x^2             (* delayed: x^2 is evaluated when rule is applied *)
```

## ReplaceAll `/.`

Apply rules to an expression:

```syma
5 /. x_ -> 42                 (* => 42 *)
{1, 2, 3} /. x_ -> x^2        (* => {1, 4, 9} *)
```

## ReplaceRepeated `//.`

Apply rules repeatedly until the result stops changing:

```syma
(* Symbolic differentiation *)
rules = {
    D[_?ConstantQ, _] :> 0,
    D[x_Symbol, x_Symbol] :> 1,
    D[x_Symbol, y_Symbol] :> 0,
    D[a_ + b_, x_] :> D[a, x] + D[b, x],
    D[a_ * b_, x_] :> a * D[b, x] + b * D[a, x],
    D[x_^n_, x_] :> n * x^(n - 1)
};
```

## Sequence Patterns

`__` matches one or more elements, `___` matches zero or more:

```syma
(* Used in list destructuring *)
{first_, rest___} := process[first, {rest}]
```

## Alternatives `|`

Match one of several patterns:

```syma
colorRedOrBlue[_Red | _Blue] := True
```

## Pattern Guards `/;`

Add a condition to a pattern:

```syma
positive[x_] /; x > 0 := True
```

## Fibonacci with Patterns

```syma
fib[0] := 0;
fib[1] := 1;
fib[n_] := fib[n - 1] + fib[n - 2];
fib[10]      (* => 55 *)
```

This is the classic Syma style — no if/else, just pattern dispatch.
