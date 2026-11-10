# Syma 长期开发计划（≥128 轮）

> 目标：把 Syma 从"名称覆盖高、语义深度浅"推进到**旗舰函数在常见输入上与 Mathematica 一致**、
> 全仓可回归、质量债收敛、子系统可用。以"轮"为单位持续推进，每轮有硬门禁。

- 状态追踪：[`docs/PROGRESS.md`](PROGRESS.md)
- 每轮门禁：[`scripts/gate.sh`](../scripts/gate.sh)
- 语言规范（真源）：`syma/syma-lang.md`
- 一致性语料（硬指标）：`syma/tests/wl_math.rs`

---

## 0. 原则

1. **门禁驱动**：任何一轮的产出必须通过 `scripts/gate.sh` 全绿才可提交。
2. **一人一文件**：并发 agent 时，一个文件同一时刻只归一个 owner；跨文件改动显式协调。
3. **永不回退**：修好的旗舰行为立即写入 `wl_math.rs` 语料锁定；未支持的一律"返回未求值"，绝不返回错值。
4. **可回归优先于广度**：每轮先补语料/单测，再落实现。
5. **小步提交**：每轮结束即提交（`syma` 子模块），超项目指针随子模块一起 bump。

## 1. 轮次协议（每轮固定流程）

```
R#  侦察 → 设计 → 分派 → 实现 → 门禁 → 提交 → 记录
```

1. **侦察**：`grep`/单测确认缺口的确切位置与根因（贴证据）。
2. **设计**：确定权威实现文件与语义（对照 WL 参考；歧义时以"返回未求值"兜底）。
3. **分派**：把该轮拆成 k 个互不重叠的文件工作流，交给 k 个 subagent（`background=true`）。
4. **实现**：agent 各自实现 + 单测，跑 `cargo check` / 相关 `--lib` / 相关语料类别。
5. **门禁**：协调者跑 `scripts/gate.sh`（fmt + clippy -D + workspace test + 语料）。
6. **提交**：`cargo fmt` → 提交 `syma`，bump 超项目 gitlink，更新 `PROGRESS.md`，推送。
7. **记录**：在 `PROGRESS.md` 标记轮次状态、指标变化、遗留项。

## 2. 单轮完成定义（DoD）

- [ ] 目标行为通过（语料类别 + 单元测试；数值用例 `N` 容差内一致）。
- [ ] 未支持路径返回未求值（无错值）。
- [ ] `scripts/gate.sh` 全绿。
- [ ] 无新增 `allow(dead_code|clippy)`（除有注释的理由）。
- [ ] `PROGRESS.md` 更新；代码已提交。

## 3. 指标（每轮后自动记录）

| 指标 | 采集方式 |
|---|---|
| 语料类别数 / 全绿 | `grep -c '#\[test\]' syma/tests/wl_math.rs` 且 `--test wl_math` 通过 |
| 名称覆盖 | `scripts/coverage.sh`（对照 `docs/wl-list.txt`） |
| 单元/集成测试数 | `grep -rc '#\[test\]' syma/src syma/tests` |
| 质量债 | `grep -rc 'allow(dead_code\|allow(clippy' syma/src` |
| 热点性能 | `scripts/bench.sh`（Integrate/Simplify/Solve/Series） |

---

## 4. 阶段与轮次

### Phase 0 — 稳定基线与工程地基（R1–R6）
| R | 目标 | 主要文件 | 验收 |
|---|---|---|---|
| R1 | 修复 Rubi 伪规则（`xRule`/`xRulex`/…）与缺失谓词（`TrinomialMatchQ`） | `SystemFiles/Kernel/Calculus/*.syma`, `integration.rs` | 伪规则生效、终止；积分语料全绿 |
| R2 | 修 `Integrate[x^x]` 错值 → 未求值 | `symbolic.rs` | 返回未求值，无错值 |
| R3 | 修 flaky `parallel::test_launch_kernels_zero_clamped_via_pool` | `parallel.rs` | 连跑 5× 稳定 |
| R4 | 去重重叠注册（VectorQ/StringCases/HarmonicNumber/Dimensions/ColorNegate…） | `mod.rs`+重复源 | 同名单注册，行为不变 |
| R5 | 门禁脚本 + 计划 + 进度表 + 覆盖率/基准脚本 | `scripts/`, `docs/` | 可一键跑 |
| R6 | 建立基线指标（覆盖/语料/测试/性能）并接入 CI | `.github/`, `docs/` | 指标入库 |

### Phase 1 — 微积分与分析深度（R7–R26）
| R | 目标 | 主要文件 |
|---|---|---|
| R7 | `Sum` 有限：Faulhaber/几何/裂项硬化 | `list.rs` |
| R8 | `Sum` 无穷：p-级数→ζ、交错调和→Log[2]、阶乘→E、几何∞ | `list.rs` |
| R9 | `Product` 符号：阶乘比/Gamma 形 | `list.rs` |
| R10 | `Limit` 基础：单侧/∞∞/00/指数主导 | `symbolicmanip.rs` |
| R11 | `Limit`：迭代 L'Hôpital + 级数法 | `symbolicmanip.rs` |
| R12 | `Series`：展开点/负幂/多变量 | `symbolicmanip.rs` |
| R13 | `SeriesData` 算术与 `Normal` | `symbolicmanip.rs` |
| R14 | `SeriesCoefficient` 符号阶 | `symbolicmanip.rs` |
| R15 | `Residue`/`ResidueSum`/极点 | `symbolicmanip.rs` |
| R16 | `Integrate`：有理函数（部分分式/Risch-lite） | `symbolic.rs` |
| R17 | `Integrate`：三角/双曲化简 | `symbolic.rs` |
| R18 | `Integrate`：exp×poly、exp×trig（分部） | `symbolic.rs` |
| R19 | `Integrate`：根式换元/Euler 换元 | `SystemFiles`, `symbolic.rs` |
| R20 | `Integrate`：广义定积分（±∞/奇点） | `symbolic.rs` |
| R21 | `Integrate`：特殊形（Erf/Gamma/椭圆） | `symbolic.rs`, `SystemFiles` |
| R22 | `NIntegrate`：自适应/无穷/振荡 | `eval/numeric.rs` |
| R23 | `D`：高阶/偏导/向量矩阵/隐式 | `symbolic.rs` |
| R24 | `DSolve`：一二阶线性 + IVP | `dsolve.rs` |
| R25 | `DSolve`：高阶常系数/非齐次 | `dsolve.rs` |
| R26 | `DSolveValue`/`NDSolve`/`RSolve` 扩展 | `dsolve.rs` |

### Phase 2 — 代数与求解（R27–R42）
| R | 目标 | 主要文件 |
|---|---|---|
| R27 | 多项式因式分解（Q 上，无平方/有理根/二次） | `polynomial.rs` |
| R28 | 分解：三/四次、分圆、多元简单 | `polynomial.rs` |
| R29 | `FactorTerms`/`FactorList`/`Decompose`/`HornerForm` | `symbolicmanip.rs` |
| R30 | `Solve`：全次数（根式+Root）、重根 | `polyreduce.rs` |
| R31 | `Solve`：线性系统精确、超/欠定 | `polyreduce.rs` |
| R32 | `Solve`：多元多项式组（Gröbner） | `polyreduce.rs` |
| R33 | `Eliminate`/`Resultant`/`Discriminant`/`Subresultants` | `polyreduce.rs` |
| R34 | `GroebnerBasis`（序、monic、Reduce） | `polyreduce.rs` |
| R35 | `Reduce`：单变量不等式/方程 | `polyreduce.rs` |
| R36 | `Reduce`：二元二次型 CAD-lite | `polyreduce.rs` |
| R37 | `Reduce`/`Resolve`：量词消元基础 | `polyreduce.rs` |
| R38 | `Abs`/`Piecewise`/`Min`/`Max` in Solve/Reduce | `polyreduce.rs` |
| R39 | 三角方程求解 | `polyreduce.rs` |
| R40 | Log/Exp 方程、定义域约束 | `polyreduce.rs` |
| R41 | `SolveAlways`/`FindInstance`/`Roots`/`ToRules` | `polyreduce.rs` |
| R42 | `SolveValues`/`ReduceValues`/`NRoots` 一致性 | `polyreduce.rs` |

### Phase 3 — 化简/假设/变换（R43–R54）
| R | 目标 | 主要文件 |
|---|---|---|
| R43 | `Simplify`：同类项/展开/约分硬化 | `symbolic.rs` |
| R44 | `Simplify`：三角规范化（倍角/平方和） | `symbolic.rs` |
| R45 | `Simplify`：exp/log 规范化 | `symbolic.rs` |
| R46 | `Simplify`：根式/去嵌套/有理化 | `symbolic.rs` |
| R47 | `FullSimplify`：多策略复杂度搜索硬化 | `expression.rs` |
| R48 | 假设引擎：谓词规范化与蕴含 | `symbolic.rs` |
| R49 | 假设贯通 Simplify/Refine（Sqrt/Abs/Sign/Conjugate/Power） | `symbolic.rs` |
| R50 | 假设贯通 Solve/Integrate/Limit | `symbolic.rs`, `polyreduce.rs` |
| R51 | `ComplexExpand`/`PowerExpand`/`FunctionExpand`/`TrigExpand`/`TrigReduce`/`TrigFactor` | `symbolic.rs` |
| R52 | `TrigToExp`/`ExpToTrig` | `symbolic.rs` |
| R53 | `Apart`/`Together`/`Cancel`；`PolynomialQuotient/Remainder` | `symbolicmanip.rs` |
| R54 | `Collect`/`Coefficient(List)` 多元 | `symbolicmanip.rs` |

### Phase 4 — 特殊函数/数论/组合（R55–R70）
| R | 目标 | 主要文件 |
|---|---|---|
| R55 | Gamma/Beta/PolyGamma 精确+数值精度 | `specialfunctions.rs` |
| R56 | Zeta/Eta/HurwitzZeta 精确+续拓 | `specialfunctions.rs` |
| R57 | Erf 族/Fresnel/Dawson | `specialfunctions.rs` |
| R58 | Bessel 族（J/Y/I/K/H、球） | `specialfuncmore.rs` |
| R59 | Airy/Kelvin/Coulomb/ParabolicCylinder | `specialfuncmore.rs` |
| R60 | 正交多项式族 | `specialfuncmore.rs` |
| R61 | 超几何/MeijerG 表与数值 | `specialfuncmore.rs` |
| R62 | 椭圆积分/Theta/模函数 | `specialfuncmore.rs` |
| R63 | `ProductLog`/`LerchPhi`/`PolyLog`/广义 `HarmonicNumber` | `specialfuncmore.rs` |
| R64 | 数论：素性/因式/同余硬化 | `number_theory.rs` |
| R65 | 数论：积性函数/除数和/`PartitionsQ` | `number_theory.rs` |
| R66 | 数论：连分数/渐近/Pell | `number_theory.rs` |
| R67 | 组合：分拆/置换/群/Stirling | `combinatorics.rs` |
| R68 | 组合：Bell/Euler/错排/拉丁 | `combinatorics.rs` |
| R69 | 离散：离散和/积、生成函数 | `discrete.rs` |
| R70 | 离散变换（Fourier/DCT/Hadamard/小波） | `discrete.rs` |

### Phase 5 — 线性代数/张量/稀疏（R71–R80）
| R | 目标 | 主要文件 |
|---|---|---|
| R71 | 精确 det/inverse/rref/rank（有理） | `linalg.rs` |
| R72 | 精确分解 LU/QR/Cholesky | `linalgmore.rs` |
| R73 | 特征：精确有理/特征多项式/Jordan/SVD | `linalgmore.rs` |
| R74 | Schur/Hessenberg/MatrixExp/Log/Function | `linalgmore.rs` |
| R75 | 矩阵谓词/范数/条件数 | `linalgmore.rs` |
| R76 | 张量：Contract/Product/Transpose/Symmetrize | `tensor.rs` |
| R77 | 稀疏数组：构造/运算/linalg | `linalgmore.rs` |
| R78 | linalg 选项/符号矩阵 | `linalg*.rs` |
| R79 | 向量分析：Grad/Div/Curl/Laplacian | `calculusmore.rs` |
| R80 | 坐标/变换矩阵硬化 | `linalgmore.rs` |

### Phase 6 — 统计/概率/ML（R81–R90）
| R | 目标 | 主要文件 |
|---|---|---|
| R81 | 分布族参数/统计/MGF/CF | `statistics.rs` |
| R82 | PDF/CDF/Quantile/InverseCDF 精度+精确 | `statistics.rs` |
| R83 | 估计量精确（Location/Dispersion） | `statsmore.rs` |
| R84 | 假设检验族（t/z/χ²/F/KS/AD/Shapiro） | `statsmore.rs` |
| R85 | 回归/拟合族 | `fit.rs` |
| R86 | 时间序列运算硬化 | `timeseries.rs` |
| R87 | 随机：种子/分布/数组/`RandomVariate` | `random.rs` |
| R88 | 数据摘要：Commonest/Histogram/相关 | `statsmore.rs` |
| R89 | ML-lite：层/`NetGraph`/训练 | `mllite.rs` |
| R90 | 核密度/平滑/`FindDistribution` | `statsmore.rs` |

### Phase 7 — 领域：图/几何/单位/日期/字符串/数据（R91–R100）
| R | 目标 | 主要文件 |
|---|---|---|
| R91 | 图算法完备 + 加权 | `graphtheory.rs` |
| R92 | 图同构/围长/连通/流基础 | `graphtheory.rs` |
| R93 | 几何：区域/测度/交并/变换 | `regions.rs` |
| R94 | 单位：量纲分析/换算/温度 | `units.rs` |
| R95 | 日期时间：时区/历法/运算 | `calendar.rs` |
| R96 | 字符串：模式/正则/替换/MatchQ | `stringpatterns.rs` |
| R97 | 文本：分词/编辑距离/词频 | `text.rs` |
| R98 | 关联：类型化键 `Value::Assoc` 重构 | `value.rs`, `association.rs` |
| R99 | Dataset/Query 语言深度 | `dataset.rs` |
| R100 | IO：格式/图像/笔记本导入导出 | `io/*` |

### Phase 8 — 运行时与系统（R101–R112）
| R | 目标 | 主要文件 |
|---|---|---|
| R101 | 模块系统：Get/Needs/BeginPackage/限定名 | `introspect.rs`, `parser.rs` |
| R102 | 错误/消息系统；`Check`/`$Aborted` | `value.rs`, `control.rs` |
| R103 | 模式引擎：性能/重复/条件/Orderless | `pattern.rs` |
| R104 | 求值器：属性/上值/作用域正确性 | `eval/*` |
| R105 | 内存/性能：驻留/哈希/结构共享 | `value.rs`, `env.rs` |
| R106 | 字节码 VM 完备 + 差分测试 | `bytecode/*` |
| R107 | JIT 覆盖与回退 | `jit/*` |
| R108 | FFI：类型/编组/安全 | `ffi/*` |
| R109 | 并行：池/`ParallelMap`/确定性 | `parallel.rs` |
| R110 | IO/流/文档；Notebook/Cell/BoxData | `streams.rs`, `notebook.rs` |
| R111 | Dynamic/`Manipulate`（JS+counter）基础 | `dynamic.rs` |
| R112 | `Compile`：类型/加速/`CompiledFunction` | `compile.rs` |

### Phase 9 — 质量与工程（R113–R120）
| R | 目标 | 主要文件 |
|---|---|---|
| R113 | 拆分 `builtins/mod.rs` 为注册子模块 | `builtins/registration/*` |
| R114 | 消除 `allow(dead_code/clippy)` | 全仓 |
| R115 | 死代码/重复实现清扫 | 全仓 |
| R116 | 巨型文件拆分为子模块 | `symbolicmanip/eval/parser` |
| R117 | 错误处理审计（builtin 无 unwrap/panic） | 全仓 |
| R118 | 输出形状一致（Solve 形状、InputForm 往返） | `value.rs`, `polyreduce.rs` |
| R119 | 文档同步（语言规范/README/开发者指南） | `docs/`, `syma-lang.md` |
| R120 | 打包：xtask/dist/MSRV/feature/dep 审计 | `xtask/`, `Cargo.toml` |

### Phase 10 — 验证与性能（R121–R128）
| R | 目标 | 主要文件 |
|---|---|---|
| R121 | 语料扩至 ≥150 类，旗舰行为全锁定 | `tests/wl_math.rs` |
| R122 | 差分测试：VM vs tree-walk vs JIT | `tests/` |
| R123 | 属性测试：Simplify 幂等 / D∘Integrate / Expand∘Factor | `tests/` |
| R124 | InputForm 往返 fuzz + 解析器鲁棒 fuzz | `tests/` |
| R125 | 基准 + 性能回归门禁 | `benches/`, `scripts/bench.sh` |
| R126 | CI 矩阵（feature/无网）/缓存/超时 | `.github/` |
| R127 | 对照 ~2000 名称清单补漏 | `docs/wl-list.txt` |
| R128 | 收尾：全门禁绿 + 发布说明 | 全仓 |

---

## 5. 里程碑

- **M1（R1–R6）**：稳定基线；门禁/指标/CI 就绪。
- **M2（R7–R26）**：微积分与分析达标（Sum/Limit/Series/Integrate/D/DSolve 常见形）。
- **M3（R27–R42）**：代数与求解达标（Factor/Solve/Reduce/Groebner）。
- **M4（R43–R54）**：化简与假设引擎达标。
- **M5（R55–R90）**：特殊函数/数论/组合/线代/统计/ML 达标。
- **M6（R91–R112）**：领域与运行时子系统达标。
- **M7（R113–R128）**：质量收敛、验证完备、性能与 CI 就绪。

## 6. 风险与对策

| 风险 | 对策 |
|---|---|
| CAD/DSolve 泛化、MeijerG 属深水区 | 常见形正确 + 未支持返回未求值；语料锁定 |
| `Value::Assoc` 类型化键影响面大 | 单列 R98，隔离推进，feature 开关过渡 |
| 多 agent 争用 `eval/*`/`mod.rs` | 一人一文件；轮内显式协调；必要时后置合并 |
| 超大文件阻碍并行 | R116 先拆分再深化 |
| 性能回退 | R125 基准门禁 |

## 7. 执行状态

见 [`docs/PROGRESS.md`](PROGRESS.md)。每轮完成后在该文件勾选并记录指标变化。
