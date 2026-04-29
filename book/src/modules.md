# Modules

Modules organize code into namespaces with explicit exports.

## Defining a Module

```syma
module MathUtils {
    export square, cube, clamp

    square[x_] := x ^ 2
    cube[x_]   := x ^ 3

    clamp[v_, lo_, hi_] := If[v < lo, lo, If[v > hi, hi, v]]
}
```

Only exported symbols are visible to importers. `square`, `cube`, and `clamp` are public; anything not listed in `export` is private.

## Importing

Import all exports:

```syma
import MathUtils
square[5]         (* => 25 *)
cube[3]           (* => 27 *)
clamp[15, 0, 10]  (* => 10 *)
```

Selective import:

```syma
import Geometry.{circleArea}
circleArea[5]     (* works *)
(* rectArea not imported *)
```

Aliased import:

```syma
import StringOps as S
S.shout["hello"]  (* => "HELLO" *)
S.whisper["HELLO"] (* => "hello" *)
```

## How Modules Work

A module evaluates to a `Module[name, {exports...}]` expression. It creates a scope where definitions are evaluated, then only the named exports are exposed. Importing binds those names in the current scope.

## Nested Modules

```syma
module Graphics {
    export Color, Shape

    module Color {
        export red, green, blue
        red   := {255, 0, 0}
        green := {0, 255, 0}
        blue  := {0, 0, 255}
    }

    module Shape {
        export circle, rect
        circle[r_] := {Type -> "circle", Radius -> r}
        rect[w_, h_] := {Type -> "rect", Width -> w, Height -> h}
    }
}

import Graphics.Color.{red, green, blue}
import Graphics.Shape.{circle, rect}
```
