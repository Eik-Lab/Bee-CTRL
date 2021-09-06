from adafruit_bme280 import basic as adafruit_bme280
import typing
import board

class BME280:
    def __init__(self, addresses: list) -> None:
        i2c = board.I2C()
        sensors = [adafruit_bme280.Adafruit_BME280_I2C(i2c, addr) for addr in addresses]
        self.sensors = sensors
    
    def __collect_temps(self):
        return [sensor.temperature for sensor in self.sensors]
    
    def __collect_pressures(self):
        return [sensor.pressure for sensor in self.sensors]

    def __collect_humidities(self):
        return [sensor.relative_humidity for sensor in self.sensors]

    def __collect_altitudes(self):
        return [sensor.altitude for sensor in self.sensors]
    
    def collect_data(self):
        return self.__collect_temps(), self.__collect_pressures(), self.__collect_humidities(), self.__collect_altitudes()


if __name__ == "__main__":
    bme = BME280([0x77])
    print(bme.collect_data())
