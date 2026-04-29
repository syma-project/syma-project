# Control Flow

## Comparisons

```syma
3 > 2        (* => True *)
3 < 2        (* => False *)
3 == 3       (* => True *)
3 != 4       (* => True *)
3 <= 3       (* => True *)
3 >= 4       (* => False *)
```

## Logical Operators

```syma
True && False    (* => False *)
True || False    (* => True *)
!True            (* => False *)
```

## If

```syma
If[True, "yes", "no"]     (* => "yes" *)
If[False, "yes", "no"]    (* => "no" *)

abs[x_] := If[x >= 0, x, -x];
abs[-7]          (* => 7 *)
abs[7]           (* => 7 *)
```

## Which

Multi-branch conditional:

```syma
sign[x_] := Which[
    x > 0, 1,
    x < 0, -1,
    True, 0
];

sign[5]     (* => 1 *)
sign[-3]    (* => -1 *)
sign[0]     (* => 0 *)
```

## Pattern-Based Dispatch

Instead of if/else chains, Syma uses pattern matching — different definitions for different inputs:

```syma
collatz[1] := 1;
collatz[n_] /; EvenQ[n] := collatz[n / 2];
collatz[n_] := collatz[3 * n + 1];

collatz[6]     (* => 1 after: 6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1 *)
```

This is the idiomatic Syma way to branch — let the pattern engine choose.
