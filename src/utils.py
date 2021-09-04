from math import exp
def gaussian(x, a, b, c, d=0):
    """Gaussian formula

    Args:
        x ([type]): [description]
        a ([type]): [description]
        b ([type]): [description]
        c ([type]): [description]
        d (int, optional): [description]. Defaults to 0.

    Returns:
        [type]: [description]
    """
    return a * exp(-((x - b) ** 2) / (2 * c ** 2)) + d


def gradient(x, width, cmap, spread=1):
    width = float(width)
    r = sum(
        gaussian(x, p[1][0], p[0] * width, width / (spread * len(cmap)))
        for p in cmap
    )

    g = sum(
        gaussian(x, p[1][1], p[0] * width, width / (spread * len(cmap)))
        for p in cmap
    )

    b = sum(
        gaussian(x, p[1][2], p[0] * width, width / (spread * len(cmap)))
        for p in cmap
    )
    
    return int(constrain(r * 255, 0, 255)) , int(constrain(g * 255, 0, 255)), int(constrain(b * 255, 0, 255))

def constrain(val, min_val, max_val):
    return min(max_val, max(min_val, val))


def map_value(x, in_min, in_max, out_min, out_max):
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
