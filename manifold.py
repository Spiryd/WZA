import numpy as np
from mpl_toolkits.mplot3d import Axes3D
import matplotlib.pyplot as plt

# Define the functions representing the manifolds
def manifold_func1(x, y):
    return x**2 + y**2

def manifold_func2(x, y):
    return np.sqrt(x**2 + y**2)

def manifold_func3(x, y):
    return x**2 - y**2

# Generate data points for x, y, and z coordinates
x = np.linspace(-10, 10, 100)
y = np.linspace(-10, 10, 100)
z = np.linspace(-10, 10, 100)
X, Y = np.meshgrid(x, y)

# Create a 3D plot
fig = plt.figure(figsize=(12, 8))

# Plot the manifolds
ax1 = fig.add_subplot(221, projection='3d')
Z1 = manifold_func1(X, Y)
ax1.plot_surface(X, Y, Z1, cmap='viridis')
ax1.set_title('V(z - x^2 - y^2)')

ax2 = fig.add_subplot(222, projection='3d')
Z2 = manifold_func2(X, Y)
ax2.plot_surface(X, Y, Z2, cmap='viridis')
ax2.set_title('V(z^2 - x^2 - y^2)')

ax3 = fig.add_subplot(223, projection='3d')
Z3 = manifold_func3(X, Y)
ax3.plot_surface(X, Y, Z3, cmap='viridis')
ax3.set_title('V(z - x^2 + y^2)')

ax4 = fig.add_subplot(224, projection='3d')
X, Z = np.meshgrid(x, z)
Y = np.zeros_like(X)
ax4.plot_surface(X, Y, Z, cmap='viridis')
ax4.plot(np.zeros_like(y), y, np.zeros_like(y))
ax4.set_title('V(xy, yz)')

# Show the plot
plt.show()
