import adafruit_tmp117
import board


class TMP117:
    def __init__(self, addresses):
        i2c = board.I2C()
        sensors = []
        for addr in addresses:
            sensors.append(adafruit_tmp117.TMP117(i2c, addr))
        self.sensors = sensors

    def collect_data(self):
        data = []
        for sensor in self.sensors:
            data.append(sensor.temperature)
        return data

if __name__ == "__main__":
    addresses = [0x48]
    tmp117 = TMP117(addresses)
    tmp117.collect_data()
