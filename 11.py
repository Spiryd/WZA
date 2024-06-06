import numpy as np
import matplotlib.pyplot as plt
import sympy as sp

def plot_curve(eq, x_range, y_range, title, filename=None):
    x, y = sp.symbols('x y')
    f = sp.lambdify((x, y), eq, "numpy")

    X, Y = np.meshgrid(np.linspace(x_range[0], x_range[1], 400), np.linspace(y_range[0], y_range[1], 400))
    Z = f(X, Y)

    plt.figure(figsize=(6, 6))
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

t = np.linspace(-500, 500, 10000)
x = (1 - t**2) / (1 + t**2)
y = 2 * t / (1 + t**2)

plt.figure(1, figsize=(6, 6))
plt.plot(x, y)
plt.title('Cicrcle implicite representation')
plt.xlim(-1.5, 1.5)
plt.ylim(-1.5, 1.5)
plt.grid(True)
plt.savefig('wykresy/ex11_1.png')

x, y = sp.symbols('x y')
circle = x ** 2 + y ** 2 - 1
plot_curve(circle, (-1.5, 1.5), (-1.5, 1.5), "Circle x^2 + y^2 = 1", "wykresy/ex11_2.png")