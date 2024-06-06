import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# Generate data points
u = np.linspace(-2, 2, 1000)
v = np.linspace(-2, 2, 1000)
u, v = np.meshgrid(u, v)

x = u * v
y = u
z = v ** 2

# Plot the surface
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.plot_surface(x, y, z, cmap='viridis')

# Labels
ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.set_zlabel('Z')

plt.title('Whitneys Umbrella')
plt.savefig('wykresy/ex10.png')