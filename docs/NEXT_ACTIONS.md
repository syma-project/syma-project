# NEXT_ACTIONS (auto-generated 2026-11-11 01:01:52 +08)

由 scripts/progress_check.sh 依据 docs/wl-list.txt 缺口生成；按类别聚类，映射到建议文件。
每次检查覆盖写入；人工可在下方“Manual notes”追加。

## 缺口聚类（Top 40）
```
Abort	# control
Abs	# algebra
AbsoluteFileName	# io/notebook
AbsolutePointSize	# graphics
AbsoluteTime	# io/notebook
AbsoluteTiming	# io/notebook
Accumulate	# statistics
Activate	# control
AdjacencyList	# graph
AdjacencyMatrix	# graph
Adjugate	# linalg
AiryAiPrime	# specialfunc
AiryAi	# specialfunc
AiryBiPrime	# specialfunc
AiryBi	# specialfunc
Alphabet	# strings/patterns
And	# boolean
AndersonDarlingTest	# statistics
AntisymmetricMatrixQ	# linalg
Apart	# algebra
AppellF1	# specialfunc
Apply	# strings/patterns
Arg	# algebra
ArgMax	# calculus
ArgMax	# optimization
ArgMin	# calculus
ArgMin	# optimization
Association	# data
AssociationMap	# data
AssociationQ	# predicates
AssociationThread	# data
Assuming	# algebra
Assuming	# control
Assumptions	# algebra
Assumptions	# control
AtomQ	# predicates
Attributes	# io/notebook
BarabasiAlbertGraphDistribution	# graph
BarcodePlot	# io/notebook
BarLegend	# graphics
```

## 建议顺序（按类别→文件）
- graph → syma/src/builtins/graphtheory.rs
- calculus/transforms → syma/src/builtins/{symbolic,symbolicmanip,calculusmore,transforms}.rs
- algebra/solve → syma/src/builtins/{polyreduce,polynomial,symbolicmanip}.rs
- special → syma/src/builtins/{specialfunctions,specialfuncmore}.rs
- number theory/combinatorics → syma/src/builtins/{number_theory,combinatorics}.rs
- linalg/tensor/sparse → syma/src/builtins/{linalg,linalgmore,tensor}.rs
- stats/probability/ml → syma/src/builtins/{statistics,statsmore,fit,timeseries,mllite}.rs
- io/notebook/streams → syma/src/builtins/io/*, {notebook,streams}.rs
- runtime → syma/src/{eval,bytecode,jit,ffi}/*

## Manual notes
- (append here)
