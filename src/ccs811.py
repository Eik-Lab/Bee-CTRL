import board
import adafruit_ccs811



class CCS811:
    def __init__(self, addresses: list ):
        i2c = board.I2C()
        sensors = [adafruit_ccs811.CCS811(i2c, addr) for addr in addresses]
        self.sensors = sensors
    
    def __collect_co2(self):
        return [sensor.eco2 for sensor in self.sensors]

    def __collect_tvoc(self):
        return [sensor.tvoc for sensor in self.sensors]
    
    def collect_data(self):
        return self.__collect_co2() , self.__collect_tvoc()

if __name__ == "__main__":
    ccs811 = CCS811(addresses = [0x5b])
    print(ccs811.collect_data())
