import math
import matplotlib.pyplot as plt
import pandas as pd
from pathlib import Path
import os


path_10 = os.path.join(Path(__file__).parent, f"output/Ave_L10T100000.txt")
dt_10 = pd.read_table(path_10, header=0, names=["p", "p_flow", "s_max"], delim_whitespace=True)
ax = dt_10.plot("p", "p_flow", label="L = 10", style="r^", markersize=3)
bx = dt_10.plot("p", "s_max", label="L = 10", style="r^", markersize=3)

path_50 = os.path.join(Path(__file__).parent, f"output/Ave_L50T100000.txt")
dt_50 = pd.read_table(path_50, header=0, names=["p", "p_flow", "s_max"], delim_whitespace=True)
ax = dt_50.plot("p", "p_flow", label="L = 50", style="bs", ax=ax, markersize=3)
bx = dt_50.plot("p", "s_max", label="L = 50", style="bs", ax=bx, markersize=3)

path_100 = os.path.join(Path(__file__).parent, f"output/Ave_L100T100000.txt")
dt_100 = pd.read_table(path_100, header=0, names=["p", "p_flow", "s_max"], delim_whitespace=True)
ax = plot_pflow = dt_100.plot("p", "p_flow", label="L = 100", style="go", ax=ax, markersize=3)
bx = plot_smax = dt_100.plot("p", "s_max", label="L = 100", style="go", ax=bx, markersize=3)

# path_1000 = os.path.join(Path(__file__).parent, f"output/Ave_L1000T10000.txt")
# dt_1000 = pd.read_table(path_1000, header=0, names=["p", "p_flow", "s_max"], delim_whitespace=True)
# axplot_pflow = dt_1000.plot("p", "p_flow", label="L = 1000", style="yv", ax=ax, markersize=3)
# plot_smax = dt_1000.plot("p", "s_max", label="L = 1000", style="yv", ax=bx, markersize=3)

plot_pflow.set_ylabel(r"$ P_{flow} $")
plot_smax.set_ylabel(r"$ \langle s_{max} \rangle $")
plot_pflow.set_xlabel(r"$ p $")
plot_smax.set_xlabel(r"$ p $")
plt.show()

# for L in [10000]:
#     # for (p, c, m) in zip([0.2, 0.3, 0.4, 0.5], ["y", "r", "g", "b"], ["^", "o", "v", "x"]):
#     #     dist = os.path.join(Path(__file__).parent, f"output/Dist_p{p}L{L}T1000.txt")
#     #     dt = pd.read_table(dist, header=0, names=["s", "n"], delim_whitespace=True)
#     #     plt.scatter(x=dt["s"], y=dt["n"], c=c, label=f"p = {p}", s=10, marker=m)

#     # plt.xlabel("s")
#     # plt.ylabel(f"n(s, p, L={L})")
#     # plt.grid()
#     # plt.legend()
#     # plt.yscale("log")

#     # plt.show()

#     plt.close()
#     dist = os.path.join(Path(__file__).parent, f"output/Dist_p0.592746L{L}T1000.txt")
#     dt = pd.read_table(dist, header=0, names=["s", "n"], delim_whitespace=True)
#     plt.scatter(x=dt["s"], y=dt["n"], c="b", label=f"p = 0.592746", s=10, marker="o")

#     plt.yscale("log")
#     plt.xlabel("s")
#     plt.ylabel(f"n(s, p=0.592746, L={L})")
#     plt.grid()
#     plt.legend()
#     plt.yscale("log")
#     plt.savefig(f'dist_p059_L{L}')

#     for (p, c, m) in zip([0.6, 0.7, 0.8], ["y", "r", "g"], ["^", "x", "v"]):
#         dist = os.path.join(Path(__file__).parent, f"output/Dist_p{p}L{L}T1000.txt")
#         dt = pd.read_table(dist, header=0, names=["s", "n"], delim_whitespace=True)
#         plt.scatter(x=dt["s"], y=dt["n"], c=c, label=f"p = {p}", s=10, marker=m)

#     plt.xlabel("s")
#     plt.ylabel(f"n(s, p, L={L})")
#     plt.grid()
#     plt.legend()
#     plt.yscale("log")
#     plt.show()