import adafruit_mlx90640
import board
import utils
import numpy as np
from PIL import Image

class mlx90640:
    def __init__(
        self,
        MINTEMP=25.0,
        MAXTEMP=45.0,
        COLORDEPTH=1000,
        INTERPOLATE=10,
        REFRESH_RATE=0b000,
    ):
        """Initialize the adafruit board .

        Args:
            MINTEMP (float, optional): [description]. Defaults to 25.0.
            MAXTEMP (float, optional): [description]. Defaults to 45.0.
            COLORDEPTH (int, optional): [description]. Defaults to 1000.
            INTERPOLATE (int, optional): [description]. Defaults to 10.
            REFRESH_RATE ([type], optional): [description]. Defaults to 0b000.
        """
        self.MINTEMP = MINTEMP
        self.MAXTEMP = MAXTEMP
        self.COLORDEPTH = COLORDEPTH
        self.INTERPOLATE = INTERPOLATE
        i2c = board.I2C()
        camera = adafruit_mlx90640.MLX90640(i2c)
        camera.refresh_rate = REFRESH_RATE
        self.camera = camera
        heatmap = (
            (0.0, (0, 0, 0)),
            (0.20, (0, 0, 0.5)),
            (0.40, (0, 0.5, 0)),
            (0.60, (0.5, 0, 0)),
            (0.80, (0.75, 0.75, 0)),
            (0.90, (1.0, 0.75, 0)),
            (1.00, (1.0, 1.0, 1.0)),
        )
        colormap = [0] * COLORDEPTH
        for i in range(self.COLORDEPTH):
            colormap[i] = utils.gradient(i, self.COLORDEPTH, heatmap)
        self.colormap = colormap

    def __frame(self):
        """Get a frame from the camera .

        Args:
            camera ([type]): [description]

        Returns:
            [type]: [description]
        """
        success = False
        while not success:
            frame = [0] * 768
            try:
                self.camera.getFrame(frame)
                success = True
            except ValueError:
                continue
            self.frame = frame

    def __transform(self):
        """Transform the pixel to the pixel map .

        Args:
            frame (list[int]): [list containing the raw temperature reading from the sensor]

        Returns:
            [list[int]]: [list containing the transformed frame]
        """
        pixels = [0] * 768
        for i, pixel in enumerate(self.frame):
                coloridx = utils.map_value(pixel, self.MINTEMP, self.MAXTEMP, 0, self.COLORDEPTH - 1)
                coloridx = int(utils.constrain(coloridx, 0, self.COLORDEPTH - 1))
                pixels[i] = self.colormap[coloridx]
        self.pixels = pixels


    
    def __create_rgb_image(self):
        """
        Transform the image to a new RGB image
        """
        self.__frame()
        self.__transform()
        image = Image.new("RGB", (32, 24))
        image.putdata(self.pixels)
        self.rgb_image = image
        del image


    def save_rgb_image(self, filename):
        """
        Save the current color image to a PNG file .
        Args:
            filename ([type]): [description]
        """
        self.__create_rgb_image()
        img = self.rgb_image.transpose(Image.FLIP_TOP_BOTTOM)
        del self.rgb_image
        img = img.resize((32 * self.INTERPOLATE, 24 * self.INTERPOLATE), Image.BICUBIC)
        img.save(f"{filename}.jpg")

        



if __name__ == "__main__":
    mlx = mlx90640()
    mlx.save_rgb_image("test2")
