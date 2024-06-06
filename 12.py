import numpy as np
import matplotlib.pyplot as plt

# Definicja funkcji r dla conchoidy Slusa
def r_conchoid(theta, a):
    return 1/np.cos(theta) + a * np.cos(theta)

# Funkcja do rysowania wykresów
def plot_conchoid(a, filename=None):
    theta = np.linspace(-np.pi/2, np.pi/2, 1000)  # theta in (-pi/2, pi/2)
    plt.figure(figsize=(10, 10))

    r = r_conchoid(theta, a)
    x = r * np.cos(theta)
    y = r * np.sin(theta)
    plt.plot(x, y, label=f'a={a}')

    plt.xlabel('x')
    plt.ylabel('y')
    plt.title(f'Conchoida Slusa dla wartości a={a}')
    plt.axhline(0, color='black',linewidth=0.5)
    plt.axvline(0, color='black',linewidth=0.5)
    plt.xlim(-10, 10)
    plt.ylim(-10, 10)
    plt.grid(True)
    plt.legend()
    if filename:
        plt.savefig(filename)
    else:
        plt.show()

def plot_conchoid_for_all_a(a_values, filename=None):
    theta = np.linspace(-np.pi/2, np.pi/2, 1000)  # theta in (-pi/2, pi/2)
    plt.figure(figsize=(10, 10))

    for a in a_values:
        r = r_conchoid(theta, a)
        x = r * np.cos(theta)
        y = r * np.sin(theta)
        plt.plot(x, y, label=f'a={a}')

    plt.xlabel('x')
    plt.ylabel('y')
    plt.title(f'Conchoida Slusa dla wartości a={a}')
    plt.axhline(0, color='black',linewidth=0.5)
    plt.axvline(0, color='black',linewidth=0.5)
    plt.xlim(-10, 10)
    plt.ylim(-10, 10)
    plt.grid(True)
    plt.legend()
    if filename:
        plt.savefig(filename)
    else:
        plt.show()

# Rysowanie wykresów dla a ∈ {-4, -2, 0, 1, 2, 3}
a_values = [-4, -2, 0, 1, 2, 3]
for a in a_values:
    plot_conchoid(a, f"wykresy/ex12_a_{a}.png")

plot_conchoid_for_all_a(a_values, "wykresy/ex12_all.png")