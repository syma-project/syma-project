# Basics: Atoms, Arithmetic, Variables

## Atoms

Atoms are the simplest values — they can't be broken down:

```syma
42           (* integer *)
3.14         (* real *)
"hello"      (* string *)
True         (* boolean *)
False
Null         (* null/void *)
Pi           (* mathematical constant *)
E            (* e ~ 2.71828 *)
```

## Arithmetic

Standard arithmetic operators:

```syma
1 + 2        (* addition, => 3 *)
3 * 4        (* multiplication, => 12 *)
10 / 2       (* division, => 5 *)
2 ^ 10       (* power, => 1024 *)
(1 + 2) * 3  (* grouping, => 9 *)
```

Internally, operators are desugared into symbolic calls:

```syma
a + b        (* becomes Plus[a, b] *)
a * b        (* becomes Times[a, b] *)
a - b        (* becomes Plus[a, Times[-1, b]] *)
```

## Variables

Assign with `=`:

```syma
x = 10;
y = 20;
x + y        (* => 30 *)
```

Variables are bound in the current scope and can be reassigned.

## Strings

```syma
StringJoin["hello", " ", "world"]   (* => "hello world" *)
StringLength["Syma"]                  (* => 4 *)
"a" <> "b" <> "c"                     (* => "abc", concat operator *)
```

## Sequences

Semicolons separate statements. The last expression is the result:

```syma
a = 1; b = 2; c = 3; a + b + c   (* => 6 *)
```

## Everything is an Expression

In Syma, every piece of code is a symbolic expression. `1 + 2` is really `Plus[1, 2]`. Functions like `Plus`, `Times`, and `Sqrt` are builtin symbols — you can use them directly:

```syma
Plus[1, 2]       (* => 3 *)
Times[3, 4]      (* => 12 *)
Power[2, 10]     (* => 1024 *)
```
