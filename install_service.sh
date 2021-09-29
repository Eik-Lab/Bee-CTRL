echo "Asking for password to move bee_ctrl.service to /lib/systemd/system"
sudo cp bee_ctrl.service /lib/systemd/system/
systemctl enable ngrok.service
systemctl start ngrok.service
