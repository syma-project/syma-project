# Syma 开发进度（live）

> 计划见 [`LONG_TERM_PLAN.md`](LONG_TERM_PLAN.md)。每轮完成后勾选并记录指标。
> 状态：`[ ]` 待办 · `[~]` 进行中 · `[x]` 完成 · `[-]` 跳过（附原因）

## 当前指标（基线：R0）

| 指标 | 基线 | 最新 |
|---|---|---|
| 语料类别（`syma/tests/wl_math.rs`，需全绿） | 31 | 31 |
| 名称覆盖（对照 `wl-list.txt`） | 860/861 | 860/861 |
| 单元测试（`syma/src`） | 3130 | — |
| 集成测试（`syma/tests`） | 713 | — |
| `allow(dead_code|clippy)` | 76 | — |
| 热点性能（Simplify[Cos²−Sin²] 等，见 `scripts/bench.sh`） | 待建 | — |

## 轮次台账

### Phase 0 — 稳定基线
- [~] R1 Rubi 伪规则 + 缺失谓词 — *in-flight*
- [~] R2 `Integrate[x^x]` 未求值 — *in-flight*
- [x] R3 flaky `parallel` 测试 — 串行锁；5× 稳定
- [~] R4 去重重叠注册 — *in-flight*
- [~] R5 门禁/计划/进度脚本 — 本文档 + `scripts/`
- [ ] R6 基线指标 + CI 接入

### Phase 1 — 微积分与分析
- [ ] R7 `Sum` 有限硬化
- [~] R8 `Sum` 无穷（ζ/p-级数/交错调和/阶乘/几何∞） — *in-flight*
- [ ] R9 `Product` 符号
- [~] R10 `Limit` 基础（单侧/∞∞/00/指数主导） — *in-flight*
- [~] R11 `Limit` 迭代 L'Hôpital + 级数法 — *in-flight（并入 R10 agent）*
- [ ] R12 `Series` 展开点/负幂/多变量
- [ ] R13 `SeriesData` 算术/`Normal`
- [ ] R14 `SeriesCoefficient` 符号阶
- [ ] R15 `Residue`/极点
- [ ] R16 `Integrate` 有理函数
- [ ] R17 `Integrate` 三角/双曲
- [ ] R18 `Integrate` exp×poly/trig
- [ ] R19 `Integrate` 根式/Euler 换元
- [ ] R20 `Integrate` 广义定积分
- [ ] R21 `Integrate` 特殊形
- [ ] R22 `NIntegrate` 自适应/无穷/振荡
- [ ] R23 `D` 高阶/偏导/向量
- [~] R24 `DSolve` 一二阶线性 + IVP — *in-flight*
- [~] R25 `DSolve` 高阶常系数/非齐次 — *in-flight（并入 R24 agent）*
- [ ] R26 `DSolveValue`/`NDSolve`/`RSolve`

### Phase 2 — 代数与求解
- [ ] R27–R42（见计划）
- [~] R36 `Reduce` 二元二次型 CAD-lite — *in-flight*
- [~] R37 `Resolve[Exists]` 判别式 — *in-flight（并入 R36 agent）*

### Phase 3 — 化简/假设/变换
- [ ] R43–R54
- [~] R44 `Simplify` 三角规范化 — *in-flight*
- [~] R49 假设贯通 Simplify/Refine — *in-flight（并入 R44 agent）*

### Phase 4 — 特殊函数/数论/组合
- [ ] R55–R70
- [~] R61 超几何/MeijerG — *in-flight*

### Phase 5–10
- [ ] 见计划（R71–R128）

## 进行中（agent 工作流，共 18）

| 轮次 | 文件（owner） | 备注 |
|---|---|---|
| R1 | `SystemFiles/**`, `integration.rs` | Rubi 伪规则 |
| R2 | `symbolic.rs` (Integrate 区) | x^x 未求值 |
| R4 | `mod.rs`(独占) + 重复源 | 去重 |
| R8 | `list.rs` (Sum 区) | 无穷级数 |
| R10/R11 | `symbolicmanip.rs` (Limit 区) | 极限深度 |
| R22 | `eval/numeric.rs` | NIntegrate 自适应/无穷 |
| R24/R25 | `dsolve.rs` | DSolve 扩类 |
| R36/R37 | `polyreduce.rs` | Reduce CAD-lite |
| R44/R49 | `symbolic.rs` (Simplify 区), `expression.rs` | 三角/假设 |
| R61 | `specialfuncmore.rs` | MeijerG |
| R64/R65 | `number_theory.rs` | 因数/符号/连分数 |
| R85 | `fit.rs` | 拟合/插值 |
| R86 | `timeseries.rs` | 时间序列运算 |
| R91/R92 | `graphtheory.rs` | 加权路径/流/生成器 |
| R93 | `regions.rs` | 几何/区域测度 |
| R94 | `units.rs` | 量纲分析 |
| R95 | `calendar.rs` | 日期/时区 |

## 已完成提交（近期）

- `9404b1e` 大幅深度波（检查点，门禁全绿：fmt+clippy+workspace+语料31）
- `674db1e` dedupe + expanded gaps（前序）

## 进度快照（checkpoint `9404b1e`，门禁全绿）

**本轮已完成并回报的轮次**：R1 Rubi 伪规则｜R2 `Integrate[x^x]`｜R3 flaky｜R4 去重(~60处)｜R8 无穷级数｜R10/R11 Limit/Series｜R22 NIntegrate(G7-K15)｜R24/R25 DSolve｜R36/R37 Reduce CAD-lite｜R44/R49 三角+假设｜R61 MeijerG｜R64/R65 数论(Pollard)｜R70 离散变换(FFT)｜R76 张量｜R85 拟合｜R86 时间序列｜R87 随机｜R89 ML-lite｜R91/R92 图论｜R93 几何｜R94 单位(精确折算)｜R95 日期｜R96 字符串模式｜R102 消息系统。

**指标**（checkpoint 实测）：`--lib` 3520 全过；workspace 4245 全过；`wl_math` 31 类全绿；fmt/clippy `-D warnings` 干净。

**遗留 item（下一轮）**：Rubi 符号指数形 `x^m(a+b x^n)^p` 及 `1/(1+x^2)^2` 等仍循环（移植丢 `_.` 可选点）；`Direction->1` 语义与 WL 文档相反；`calculusmore` 2 个序列测试疑顺序相关（需 ≥128MB 栈）。

**仍进行中**：R43/R45/R46/R47/R50/R51/R52/R53/R54/R55/R56/R58/R59/R62/R63/R66/R67/R68/R69/R71-75/R77/R78/R81-84/R88/R90/R97/R98/R99/R100/R101/R103-112/R113-128 的部分。

- 访问实时：`git -C syma log --oneline -10`
