sn=$(cat /sys/firmware/devicetree/base/serial-number)
echo "Your serial number is: $sn"
echo "Adding Serial number to .env file"
echo "$sn" >> .env
