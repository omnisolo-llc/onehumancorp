import sympy

def compute_y(M, d, f, r, t, a):
    p_0 = sympy.sqrt(a + d - f)
    y = M * (-sympy.sin(p_0) * sympy.sin(d * t - r) + sympy.cos(d * t) * sympy.sin(p_0 - r))
    return y
