import sympy
from solution import compute_y

def test_compute_y():
    M, d, f, r, t, a = sympy.symbols('M d f r t a')

    # Calculate y
    y = compute_y(M, d, f, r, t, a)

    # Test t = 0
    y_t0 = y.subs(t, 0)
    p_0 = sympy.sqrt(a + d - f)
    # y = M * (-sin(p_0) * sin(-r) + cos(0) * sin(p_0 - r))

    expected_y_t0 = M * (-sympy.sin(p_0) * sympy.sin(-r) + sympy.cos(0) * sympy.sin(p_0 - r))

    assert sympy.simplify(y_t0 - expected_y_t0) == 0
    print("Test passed for t=0")

    print("y expression:", y)

test_compute_y()
