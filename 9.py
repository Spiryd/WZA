import numpy as np
import matplotlib.pyplot as plt
import sympy as sp

def plot_curve(eq, x_range, y_range, title, filename=None):
    x, y = sp.symbols('x y')
    f = sp.lambdify((x, y), eq, "numpy")

    X, Y = np.meshgrid(np.linspace(x_range[0], x_range[1], 400), np.linspace(y_range[0], y_range[1], 400))
    Z = f(X, Y)

    plt.figure(figsize=(8, 6))
    plt.contour(X, Y, Z, levels=[0], colors='b')
    plt.title(title)
    plt.xlabel('x')
    plt.ylabel('y')
    plt.grid(True)
    plt.axhline(0, color='black',linewidth=0.5)
    plt.axvline(0, color='black',linewidth=0.5)
    if filename:
        plt.savefig(filename)
    else:
        plt.show()

# 1. Krzywa algebraiczna: (x^2 + y^2 + 4y)^2  - 16(x^2 + y^2) = 0
x, y = sp.symbols('x y')
eq1 = (x**2 + y**2 + 4 * y)**2 - 16 * (x**2 + y**2)
plot_curve(eq1, (-7, 7), (-9, 3), "Krzywa: (x^2 + y^2 + 4y)^2  - 16(x^2 + y^2) = 0", "wykresy/ex9_1.png")

# 2. Krzywa algebraiczna: 2(x^2 + 9)(y^2 - 16) + (x^2 - 9)^2 + (y^2 - 16)^2 = 0
eq2 = 2 * (x**2 + 9) * (y**2 - 16) + (x**2 - 9)**2 + (y**2 - 16)**2
plot_curve(eq2, (-8, 8), (-5, 5), "Krzywa: 2(x^2 + 9)(y^2 - 16) + (x^2 - 9)^2 + (y^2 - 16)^2 = 0", "wykresy/ex9_2.png")

# 3. Krzywa algebraiczna: 350x^2y^2 - 15^2(x^2 + y^2) + 12^2(x^4 + y^4) + 81 = 0
eq3 = 350 * x**2 * y**2 - 15**2 * (x**2 + y**2) + 12**2 * (x**4 + y**4) + 81
plot_curve(eq3, (-2, 2), (-2, 2), "Krzywa: 350x^2y^2 - 15^2(x^2 + y^2) + 12^2(x^4 + y^4) + 81 = 0", "wykresy/ex9_3.png")

# Wyznaczanie punktów osobliwych

def func_eq1(x, y):
    return (x**2 + y**2 + 4 * y)**2 - 16 * (x**2 + y**2) == 0
def func_eq2(x, y):
    return 2 * (x**2 + 9) * (y**2 - 16) + (x**2 - 9)**2 + (y**2 - 16)**2 == 0
def func_eq3(x, y):
    return 350 * x**2 * y**2 - 15**2 * (x**2 + y**2) + 12**2 * (x**4 + y**4) + 81 == 0

def find_singular_points(eq, func):
    x, y = sp.symbols('x y')
    fx = sp.diff(eq, x)
    fy = sp.diff(eq, y)
    singular_points = sp.solve([fx, fy], (x, y), dict=True)
    points = []
    for point in singular_points:
        if func(point[x], point[y]):
            points.append(point)
            #singular_points.remove(point)
    return points

# 1. Punkty osobliwe
singular_points_1 = find_singular_points(eq1, func_eq1)
print("Punkty osobliwe krzywej 1:", singular_points_1)

# 2. Punkty osobliwe
singular_points_2 = find_singular_points(eq2, func_eq2)
print("Punkty osobliwe krzywej 2:", singular_points_2)

# 3. Punkty osobliwe
singular_points_3 = find_singular_points(eq3, func_eq3)
print("Punkty osobliwe krzywej 3:", singular_points_3)