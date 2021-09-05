from typing import Optional
import adafruit_tmp117
import typing
import board


class TMP117:
    def __init__(self, addresses: typing.Union[int, list] ):
        """Initialize the sensor.

        Args:
            addresses ([list[int]]): [List or value containing the different sensor addresses.]
        """
        i2c = board.I2C()
        sensors = [adafruit_tmp117.TMP117(i2c, addr) for addr in addresses]
        self.sensors = sensors

    def collect_data(self):
        """Return a list of all the temperature reading from the different sensors .

        Returns:
            [type]: [description]
        """
        return [sensor.temperature for sensor in self.sensors]

if __name__ == "__main__":
    addresses = [0x49]
    tmp117 = TMP117(addresses)
    tmp117.collect_data()
