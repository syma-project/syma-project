# Lists

Lists are the core data structure — written as comma-separated values in braces:

```syma
{1, 2, 3, 4, 5}
{}
{"a", "b", "c"}
```

Internally, `{1, 2, 3}` is `List[1, 2, 3]`.

## Accessing Elements

```syma
nums = {10, 20, 30, 40, 50};

First[nums]     (* => 10 *)
Last[nums]      (* => 50 *)
Rest[nums]      (* => {20, 30, 40, 50} *)
Most[nums]      (* => {10, 20, 30, 40} *)
Length[nums]    (* => 5 *)
Part[nums, 2]   (* => 20 *)
```

## Building Lists

```syma
Append[{1, 2, 3}, 4]      (* => {1, 2, 3, 4} *)
Prepend[{1, 2, 3}, 0]      (* => {0, 1, 2, 3} *)
Join[{1, 2}, {3, 4}]       (* => {1, 2, 3, 4} *)
```

## Transforming Lists

```syma
Reverse[{1, 2, 3}]              (* => {3, 2, 1} *)
Sort[{3, 1, 4, 1, 5, 9}]        (* => {1, 1, 3, 4, 5, 9} *)
Flatten[{{1, 2}, {3, {4, 5}}}]   (* => {1, 2, 3, 4, 5} *)
```

## Generating Lists

```syma
Range[5]          (* => {1, 2, 3, 4, 5} *)
Range[1, 10]      (* => {1, 2, 3, 4, 5, 6, 7, 8, 9, 10} *)

Table[i^2, {i, 1, 5}]   (* => {1, 4, 9, 16, 25} *)
```

## Aggregating

```syma
Total[{1, 2, 3, 4, 5}]   (* => 15 *)
```

## Set Operations

```syma
Union[{1, 2, 3}, {2, 3, 4}]          (* => {1, 2, 3, 4} *)
Intersection[{1, 2, 3}, {2, 3, 4}]   (* => {2, 3} *)
Complement[{1, 2, 3, 4}, {2, 4}]     (* => {1, 3} *)
```

## List Utilities

```syma
Take[{1, 2, 3, 4, 5}, 3]      (* => {1, 2, 3} *)
Drop[{1, 2, 3, 4, 5}, 2]      (* => {3, 4, 5} *)

MemberQ[{1, 2, 3}, 2]         (* => True *)
Count[{1, 2, 2, 3, 2}, 2]     (* => 3 *)
Position[{1, 2, 2, 3}, 2]     (* => {2, 3} *)

Tally[{1, 1, 2, 3, 3, 3}]     (* => {{1, 2}, {2, 1}, {3, 3}} *)

PadLeft[{1, 2, 3}, 5]         (* => {0, 0, 1, 2, 3} *)
PadRight[{1, 2, 3}, 5]        (* => {1, 2, 3, 0, 0} *)
```

Associations (hash maps):

```syma
assoc = <|"name" -> "Syma", "version" -> 1|>;
Keys[assoc]         (* => {"name", "version"} *)
Values[assoc]       (* => {"Syma", 1} *)
Lookup[assoc, "name"]   (* => "Syma" *)
KeyExistsQ[assoc, "name"]   (* => True *)
```
