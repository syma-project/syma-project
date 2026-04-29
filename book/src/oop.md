# Object-Oriented Programming

Syma has class-based OOP: classes, fields, constructors, methods, inheritance, and mixins.

## Basic Class

```syma
class Point {
    field x: Real
    field y: Real

    constructor[x_, y_] {
        this.x = x
        this.y = y
    }

    method distance[] := Sqrt[x^2 + y^2]
    method translate[dx_, dy_] := Point[x + dx, y + dy]
    method toString[] := "(" <> ToString[x] <> ", " <> ToString[y] <> ")"
}

p = Point[3, 4]
p.distance[]      (* => 5 *)
```

## Field Defaults

```syma
class Circle {
    field radius: Real
    field color: String = "black"

    constructor[r_] {
        this.radius = r
    }

    method area[] := Pi * radius^2
}

c = Circle[5]
c.color           (* => "black" *)
```

## Inheritance

```syma
class Shape {
    field name: String = "shape"
    method area[] := 0
    method describe[] := name <> " with area " <> ToString[area[]]
}

class Rectangle extends Shape {
    field width
    field height

    constructor[w_, h_] {
        this.width = w
        this.height = h
        this.name = "rectangle"
    }

    method area[] := width * height
    method isSquare[] := width == height
}

class Square extends Rectangle {
    constructor[size_] {
        this.width = size
        this.height = size
        this.name = "square"
    }
}

r = Rectangle[3, 4]
r.area[]          (* => 12 *)
r.describe[]      (* => "rectangle with area 12" *)

s = Square[5]
s.area[]          (* => 25 *)
s.isSquare[]      (* => True *)
```

## Mixins

Mixins contribute methods without constructors:

```syma
mixin Printable {
    method print[] := Print[toString[]]
}

mixin Serializable {
    method toJSON[] := "{ \"type\": \"" <> Head[] <> "\" }"
}

class Vector with Printable {
    field components: List
    constructor[comps_] { this.components = comps }
    method toString[] := "Vector[" <> ToString[components] <> "]"
}

v = Vector[{1, 2, 3}]
v.print[]         (* prints: Vector[{1, 2, 3}] *)

(* Multiple mixins *)
(* class Matrix with Printable, Serializable { ... } *)
```

## Operator Overloading

Special methods overload operators:

```syma
class Vector2D {
    field x
    field y

    constructor[x_, y_] { this.x = x; this.y = y }

    method __add__[other_Vector2D] := Vector2D[x + other.x, y + other.y]
    method __mul__[scalar_] := Vector2D[x * scalar, y * scalar]
    method __eq__[other_Vector2D] := x == other.x && y == other.y
    method __repr__[] := "Vector2D[" <> ToString[x] <> ", " <> ToString[y] <> "]"
    method __len__[] := 2
    method __getitem__[i_] := If[i == 1, x, y]
}
```

## Class as Pattern

Class names work as type patterns:

```syma
areaOfShape[c_Circle] := Pi * c.radius^2
areaOfShape[r_Rectangle] := r.width * r.height
```

## Object Internals

Objects are `Object[ClassName, {field -> value, ...}]` — the same symbolic structure as everything else. They participate in pattern matching and rule application.

See `syma/examples/advanced/02-oop.syma` and `03-banking.syma` for complete examples.
