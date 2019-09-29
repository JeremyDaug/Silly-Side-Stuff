from matplotlib import pyplot as plt
from matplotlib import animation
import numpy as np
import math as math


E_0 = 1000
radius = 25
D = 0.05
dx = 1
dt = 1

fig, ax = plt.subplots()

x = np.arange(-radius, radius)
E = np.ones((2*radius)) * E_0
line, = ax.plot(x, E)

spellCost = E
for i in range(len(E)):
    if -10 < i < 10:
        spellCost[i] = -100 * dt
    else:
        spellCost = 0


def init():
    line.set_ydata(E)
    return line,


def Seep(data):
    return np.where(data < E_0, data + 1*dt, data)


def Cost(data):
    result = data - spellCost
    return np.where(result < 0, 0, result)


def Flux(data):
    result = data.copy()
    result[1:-1] = data[1:-1] + D * dt * (
        (data[2:] - 2*data[1:-1] + data[:-2]) / dx**2
    )
    return result


def updatefig(i):
    global E
    E = Seep(E)
    E = Flux(E)
    E = Cost(E)
    line.set_ydata(E)
    return line,

ani = animation.FuncAnimation(fig, updatefig, init_func=init, frames=1001, interval=50, blit=True)
plt.show()
