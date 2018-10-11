## pigrust

This is a rust wrapper around `pigpiod`

## Dependencies
- `pigpiod` , obviously
- Raspberry Pi 3 or similar hardware.  Tested with RPi3, RPi 0 W. It probably works on other Pi hardware, but hasn't been tested.

### Install `pigpio`
Raspbian "stretch" comes with pigpio preinstalled but you may need to enable it (see below)
Prior versions did not come with pigpio preinstalled.

```
sudo apt-get update
sudo apt-get install pigpio python-pigpio python3-pigpio
```
Strictly speaking only pigpio itself is required.
Install the python packages if you think you'll use python to access pigpio as well.

### Setup `pigpiod` to start on reboot
It's handy to have `pigpiod` restart automatically when the system boots.

- `sudo update-rc.d pigpiod defaults`
- `sudo update-rc.d pigpiod enable`

### Verify pigpiod is enabled
- After reboot, verify with: `sudo service pigpiod status`
- If pigpiod is not enabled, use `sudo systemctl enable pigpiod`

