# Symbolic Expressions

The defining idea of Syma: **everything is a symbolic expression**. Code is data. You can build, inspect, and transform expressions programmatically.

## Expressions Are Lists

```syma
Plus[1, 2]       (* evaluates to 3 *)
Hold[Plus[1, 2]] (* held, stays as expression *)
```

The heads of expressions are themselves symbols:

```syma
Head[Plus[1, 2]]    (* => Plus *)
Head[{1, 2, 3}]     (* => List *)
Head["hello"]       (* => String *)
Head[42]            (* => Integer *)
```

## Building Symbolic Expression Trees

You can build expression trees manually with OOP classes:

```syma
class Expr {
    method evaluate[env_] := 0
    method derivative[var_] := 0
    method toString[] := "?"
}

class Constant extends Expr {
    field value
    constructor[val_] { this.value = val }
    method evaluate[env_] := value
    method toString[] := ToString[value]
}

class Variable extends Expr {
    field name
    constructor[name_] { this.name = name }
    method evaluate[env_] := Lookup[env, name]
    method toString[] := name
}

class Add extends Expr {
    field left; field right
    constructor[l_, r_] { this.left = l; this.right = r }
    method evaluate[env_] := left.evaluate[env] + right.evaluate[env]
    method derivative[var_] := Add[left.derivative[var], right.derivative[var]]
    method toString[] := "(" <> left.toString[] <> " + " <> right.toString[] <> ")"
}

class Multiply extends Expr {
    field left; field right
    constructor[l_, r_] { this.left = l; this.right = r }
    method evaluate[env_] := left.evaluate[env] * right.evaluate[env]
    method derivative[var_] :=
        Add[Multiply[left.derivative[var], right],
            Multiply[left, right.derivative[var]]]
    method toString[] := left.toString[] <> " * " <> right.toString[]
}

(* Build 3x^2 + 2x + 1 *)
expr = Add[
    Multiply[Constant[3], PowerExpr[Variable["x"], Constant[2]]],
    Add[Multiply[Constant[2], Variable["x"]], Constant[1]]
];

expr.toString[]                         (* => (3 * x^2 + (2 * x + 1)) *)
expr.evaluate[<|"x" -> 5|>]            (* => 86 *)
expr.derivative["x"].toString[]        (* => (6 * x + 2) *)
```

## The `match` Expression

```syma
describe[expr_] := match expr {
    Constant[_]      => "constant"
    Variable[_]      => "variable"
    Add[_, _]        => "addition"
    Multiply[_, _]   => "multiplication"
    _                => "unknown"
}
```

## Rule-Based Transformation

Syma's built-in symbolic operations use rules internally:

```syma
Expand[(x + 1)^3]           (* => 1 + 3x + 3x^2 + x^3 *)
Factor[x^2 - 1]             (* => (-1 + x)(1 + x) *)
D[x^3, x]                   (* => 3 x^2 *)
```

The full symbolic expression example is at `syma/examples/advanced/04-expression.syma`.
