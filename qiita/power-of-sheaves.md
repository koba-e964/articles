層$(A, \mathrm{E}_A, \rceil_A), (B, \mathrm{E}_B, \rceil_B)$があるとき、層の巾$B^A$を定義する。

まず台集合$B^A$を定義する。
$B^A := \\{ (f, V) \mid f \colon A \to B, \mbox{function}, V \in O(X),
\forall a\in A\ldotp (\mathrm{E}(f(a)) = \mathrm{E}(a) \cap V \wedge \forall U \in O(X)\ldotp f(a \rceil_A U) = f(a) \rceil_B U \\}$
次に$B^A$の上の層の構造、つまり ${\mathrm{E}} \colon B^A \to O(X)$, $\rceil_{B^A}\colon B^A \times O(X) \to B^A$を以下で定義する。

$\mathrm{E}(f, V) := V$, $(f, V) \rceil_{B^A} U := (\lambda a\ldotp f(a \rceil_A U), V \cap U)$.

この時、これらの関数はwell-definedであることを示す。
(i) $\mathrm{E}$ は well-defined: 自明。
(ii) $\rceil$ は well-defined: 任意の$(f, V) \in B^A, U \in O(X)$に対して、$(\lambda a\ldotp f(a \rceil_A U), V \cap U) \in B^A$を示せばよい。$\lambda a\ldotp f(a \rceil_A U)$は当然$A \to B$の関数であり、$V \cap U$は$X$の開集合である。これらが $B^A$ の defining equations を満たすことを示そう。
(ii-i) Goal: $\forall a \in A\ldotp \mathrm{E}(f(a \rceil_A U)) = \mathrm{E}(a) \cap V \cap U$
$$ \mathrm{E}(f(a \rceil_A U)) = \mathrm{E}(a \rceil_A U) \cap V \\\\
= \mathrm{E}(a) \cap U \cap V$$ より、明らか。
(ii-ii) Goal: $\forall a \in A\ldotp \forall W \in O(X) \ldotp f((a \rceil_A W) \rceil_A U) = f(a \rceil_A U) \rceil_B W$
$$ f((a \rceil_A W) \rceil_A U)
= f(a \rceil_A (W \cap U)) \\\\
= f(a \rceil_A (U \cap W)) \\\\
= f((a \rceil_A U) \rceil W) \\\\
= f(a \rceil_A U) \rceil W$$ より、明らか。

以上により関数$\mathrm{E}, \rceil$が定義された。これらが$B^A$と合わせ層をなすことを示す。　
(PSh1) Goal: 第一の定義で前層であること
(PSh1-0) Goal: $\forall (f_1, V_1)\ (f_2, V_2) \in B^A\ldotp (f_1, V_1) \rceil \emptyset = (f_2, V_2) \rceil \emptyset$
$(f, V) \rceil \emptyset = (\lambda \ldotp f(a \rceil \emptyset), \emptyset)$より自明。
(PSh1-1) Goal: $(f, V) \rceil V = (f, V)$
$(f, V) \rceil V = (\lambda a\ldotp f(a \rceil V), V)$である。
$B^A$の定義から$\mathrm{E}(f(a)) = \mathrm{E}(a) \cap V$であるため、

$$f(a \rceil V) = f(a) \rceil V = f(a) \rceil (\mathrm{E}(a) \cap V) \rceil V \\\\
= f(a) \rceil (\mathrm{E}_A(a) \cap V) \\\\
= f(a)$$
よって示された。
(PSh1-2) Goal: $\mathrm{E}((f, V) \rceil U) = \mathrm{E}(f, V) \cap U$
これは定義から明らか。両辺共に $V \cap U$ である。
(PSh1-3) Goal: $((f, V) \rceil U) \rceil W = (f, V) \rceil (U \cap W)$
$$(\mbox{LHS}) = (\lambda a\ldotp f(a \rceil U), V \cap U) \rceil W \\\\
= (\lambda a\ldotp f((a \rceil W) \rceil U), V \cap U \cap W)$$
$$(\mbox{RHS}) = (\lambda a\ldotp f(a \rceil (U \cap W)), V \cap (U \cap W))$$
前層としての$A$についての(PSh1-3)より、これらは等しい。

(Sh1) Goal: $B^A$が層であること
$F \subseteq B^A$ を、互いに両立する元からなる部分集合と仮定する。このとき、
(1) $\bigcup_{} F$ が存在すること
(2) $\bigcup_{} F$ が唯一であること
を示せばよい。
(1) $\bigcup_{} F$ を構成しよう。
この仮定は以下のように同値変形できる:
$$(\mbox{Asm}) \Leftrightarrow \forall (f_1, V_1), (f_2,V_2) \in B^A\ldotp (f_1,V_1)\rceil V_2 = (f_2, V_2) \rceil V_1 \\\\
\Leftrightarrow 
\forall (f_1, V_1), (f_2,V_2) \in B^A\ldotp (\lambda a\ldotp f_1(a) \rceil V_2, V_1 \cap V_2) = (\lambda a\ldotp f_2(a) \rceil V_1, V_2 \cap V_1)\\\\
\Leftrightarrow \forall (f_1, V_1), (f_2,V_2) \in B^A\ldotp \forall a\in A\ldotp f_1(a) \rceil V_2 = f_2(a) \rceil V_1 
$$
以下のように $(g, W) \in B^A$ を構成する:
$$ W := \bigcup_{(f, V) \in F} V \\\\
F' := \\{f(a) \mid (f, V) \in F\\} \\\\
g(a) := \bigcup_{} \\{f(a) \mid (f, V) \in F\\}$$
ここで、
(1-1) これが well-defined であること
(1-2) これが層の公理を満たすこと
を示せばよい。
(1-1) これが well-defined であることを示す。 
(1-1-1) $W$ が well-defined であることは自明。
(1-1-2) $g$ が well-defined であることについては、 $F' = \\{f(a) \mid (f, V) \in F\\}$ の任意の二元が両立することを示せばよい。これは、任意の $(f_1, V_1), (f_2, V_2) \in F$ に対して、
$$ f_1(a) \rceil \mathrm{E}(f_2(a)) = f_1(a) \rceil (\mathrm{E}(a) \cap V_2)\\\\
= (f_1(a) \rceil E(a)) \rceil V_2 \\\\
= f_1(a \rceil E(a)) \rceil V_2 \\\\
= f_1(a) \rceil V_2$$
および仮定から明らかである。
(1-1-3) $(g, W) \in B^A$ を示す。
(1-1-3-1) Goal: $\forall a \ldotp \mathrm{E}_B(g(a)) =
 W \cap \mathrm{E}( a )$

$$ \mathrm{E}(g(a))
= \bigcup_{u \in F'} \mathrm{E}(u) \\\\
= \bigcup_{(f, V) \in F} (\mathrm{E}(a) \cap V) \\\\
= \mathrm{E}(a) \cap (\bigcup_{(f, V) \in F} V) \\\\
= \mathrm{E}_A(a) \cap W $$
より、示された。

(1-1-3-2) Goal: $\forall a\in A\ldotp \forall U \in O(X)\ldotp g(a \rceil_A U) = g(a) \rceil_B U$
$F'$ の $a$ に $a \rceil U$ を代入したものを $F'' := \\{f(a \rceil U) \mid (f, V) \in F\\}$ とする。
$g(a \rceil U) = \bigcup_{} F''$ と $\bigcup_{} F''$ の唯一性より、 $g(a) \rceil_B U$ も $\bigcup_{} F''$ としての性質を満たすことを示せばよい。
(1-1-3-2-1) Goal: $\forall (f, V) \in F\ldotp (g(a) \rceil U) \rceil \mathrm{E}(f(a \rceil U)) = f(a \rceil U)$
$$(\mbox{LHS}) = g(a) \rceil (U \cap \mathrm{E}(a \rceil U) \cap V) \\\\
= g(a) \rceil (\mathrm{E}(a) \cap V \cap U) \\\\
= g(a) \rceil (\mathrm{E}(f(a)) \cap U) \\\\
= (g(a) \rceil \mathrm{E}(f(a))) \rceil U \\\\
= f(a) \rceil U = (\mbox{RHS})$$ より、示された。
(1-1-3-2-2) Goal: $\mathrm{E}(g(a) \rceil U) = \bigcup_{(f, V) \in F} \mathrm{E}(f(a \rceil U))$
$$(\mbox{LHS})
= \mathrm{E}(g(a)) \cap U \\\\
= (\bigcup_{(f, V) \in F} \mathrm{E}(f(a))) \cap U \\\\
= \bigcup_{(f, V) \in F} (\mathrm{E}(f(a)) \cap U) \\\\
= \bigcup_{(f, V) \in F} \mathrm{E}(f(a) \rceil U) \\\\
= \bigcup_{(f, V) \in F} \mathrm{E}(f(a \rceil U)) \\\\
= (\mbox{RHS})$$ 以上により示された。
(1-2) これが層の公理を満たすことを示す。

(Sh1-1) Goal: $\forall (f, V) \in F\ldotp (g, W) \rceil V = (f, V)$
$$(g, W) \rceil V = (\lambda a \ldotp g(a) \rceil V, W \cap V) \\\\
= (\lambda a \ldotp g(a \rceil \mathrm{E}(a)) \rceil V, V) \\\\
= (\lambda a \ldotp (g(a) \rceil \mathrm{E}(a)) \rceil V, V) \\\\
= (\lambda a \ldotp g(a) \rceil (\mathrm{E}(a) \cap V), V) \\\\
= (\lambda a \ldotp g(a) \rceil \mathrm{E}(f(a)), V) \\\\
= (\lambda a \ldotp f(a), V) \\\\
= (f, V)
$$
より、示された。

(Sh1-2) Goal: $\mathrm{E}((g, W)) = \bigcup_{(f, V) \in F} \mathrm{E}((f, V))$
両辺を変形すると $W = \bigcup_{(f, V) \in F} V$ となるが、これは $W$ の定義より明らか。

(2) 以上の性質を持つ $\bigcup_{} F$ が唯一であることを示す。
$(g_1, W_1), (g_2, W_2)$ が共に上の性質を満たすと仮定し、 $(g_1, W_1) = (g_2, W_2)$ を示せばよい。
(2-1) $W_1 = W_2$ を示す。
性質 (Sh1-2) より、$W_1 = W_2 = \bigcup_{(f, V) \in F} V$ となるため明らか。
以降 $W := W_1 = W_2$ と表記する。
(2-2) $g_1 = g_2$ を示す。
任意の $a \in A$ に対して $g_1(a) = g_2(a)$ を示せばよい。
そのためには、 $F' = \\{f(a) \mid (f, V) \in F\\}$ について、$g_1(a)$ と $g_2(a)$ が共に $\bigcup_{} F'$ であるための条件を満たすことを示せばよい。 
性質 (Sh1-1) については、上の (Sh1-1) の証明で見たように、$(g, W)$ について (Sh1-1) が成立することと、任意の $a \in A$ に対して $g(a)$ について (Sh1-1) が成立することが同値であるため、 $g_1(a)$ も $g_2(a)$ も (Sh1-1) を満たす。
性質 (Sh1-2) については、
$$\mathrm{E}(g_i(a)) = \mathrm{E}(a) \cap W \\\\
= \mathrm{E}(a) \cap \bigcup_{(f, V) \in F} V \\\\
= \bigcup_{(f, V) \in F} (\mathrm{E}(a) \cap V) \\\\
= \bigcup_{(f, V) \in F} \mathrm{E}(f(a)) \\\\
= \bigcup_{u \in F'} \mathrm{E}(u)
$$
であることから、示された。
以上より、 $g_1(a)$ も $g_2(a)$ も $\bigcup_{} F'$ であるための条件を満たすので、 $g_1(a) = g_2(a)$ である。
