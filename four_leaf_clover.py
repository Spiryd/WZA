import numpy as np
import matplotlib.pyplot as plt

# Definicja funkcji r(θ)
def r(theta):
    return np.sin(2 * theta)

# Tworzenie wartości kąta od 0 do 2*pi
theta_values = np.linspace(0, 2 * np.pi, 1000)

# Obliczenie wartości promienia r dla każdego kąta
r_values = r(theta_values)

# Konwersja współrzędnych biegunowych na kartezjańskie
x_values = r_values * np.cos(theta_values)
y_values = r_values * np.sin(theta_values)

# Narysowanie wykresu
plt.figure(figsize=(8, 8))
plt.plot(x_values, y_values, label='r(θ) = sin(2θ)')
plt.title('Krzywa czterolistna w układzie współrzędnych kartezjańskich')
plt.xlabel('x')
plt.ylabel('y')
plt.axis('equal')
plt.grid(True)
plt.legend()
plt.show()
