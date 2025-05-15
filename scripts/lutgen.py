import math

from PIL import Image

palette = Image.open("32rogues-palette-rgb.png")
palette_colors = palette.getcolors()

lut_dimension = 32
max_y = (lut_dimension - 1) + (lut_dimension - 1) * lut_dimension

linear_lut = Image.new("RGB", (lut_dimension, lut_dimension * lut_dimension))
palette_lut = Image.new("RGB", (lut_dimension, lut_dimension * lut_dimension))

distances = []

for r in range(0, lut_dimension):
    for g in range(0, lut_dimension):
        for b in range(0, lut_dimension):
            pixel = (r, g + b * lut_dimension)
            lin_color = ((r * 255) // (lut_dimension - 1), (g * 255) // (lut_dimension - 1), (b * 255) // (lut_dimension - 1))

            linear_lut.putpixel(pixel, lin_color)

            min_dist = -1
            closest_color = (0, 0, 0)

            for (_, pal_color) in palette_colors:
                if pal_color[3] == 255:
                    rw = 1 #math.sqrt(abs(r - (lut_dimension / 2))) + 1 #math.sqrt(r + 1)
                    gw = 1 #math.sqrt(abs(g - (lut_dimension / 2))) + 1 #math.sqrt(g + 1)
                    bw = 1 #math.sqrt(abs(g - (lut_dimension / 2))) + 1 #math.sqrt(b + 1)

                    dist = math.sqrt(rw * pow(pal_color[0] - lin_color[0], 2) + gw * pow(pal_color[1] - lin_color[1], 2) + bw * pow(pal_color[2] - lin_color[2], 2))

                    if dist < min_dist or min_dist < 0:
                        min_dist = dist
                        closest_color = pal_color
            
            distances.append(min_dist)
            palette_lut.putpixel(pixel, closest_color)

color_count = len(palette_lut.getcolors())

if color_count == 59:
    linear_lut.save("linear-lut.png")
    palette_lut.save("palette-lut.png")

    print("lut saved")
else:
    print("not all palette colors used, increase lut size")

#print(distances)
#print("-----")
