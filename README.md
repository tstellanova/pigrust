## pigrust

This is a rust wrapper around `pigpiod`

## Dependencies
- `pigpiod` , obviously
- Raspberry Pi 3 or similar hardware.  It probably works on other Pi hardware, but hasn't been tested.

### Install `pigpio`
Currently raspbian doesn't come with pigpio preinstalled.

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
- After reboot, verify with: ` sudo service pigpiod status`

