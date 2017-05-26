# Ultrasonic Pi Piano with Gesture Control

As featured on [Hackaday](https://hackaday.com/2017/04/22/ultrasonic-raspberry-pi-piano/)! This project for the Raspberry Pi uses cheap HC-SR04 ultrasonic sensors as inputs and turns them into keys on a MIDI keyboard. There's something pretty neat about using sound to make sound. It's like fighting fire with fire, but much safer.

There is a detailed [instructable](https://www.instructables.com/id/Ultrasonic-Pi-Piano-With-Gesture-Controls/) with full information on how to make this project.

# Video

[![Raspberry Pi Octasonic Piano](https://img.youtube.com/vi/eXgCb6xm2Ug/0.jpg)](https://youtu.be/eXgCb6xm2Ug)

# Running in headless mode

To have the code start up when you boot the Raspberry Pi (without needing keyboard, mouse, and monitor attached) add these lines to your `/etc/rc.local` file and reboot.

```
. /home/pi/.cargo/env
cd /home/pi/UltrasonicPiPiano
./run.sh > /var/log/ultrasonic-pi.log 2>&1

```

In my case, the full `/etc/rc.local` file looked like this after adding those lines:

```
#!/bin/sh -e
#
# rc.local
#
# This script is executed at the end of each multiuser runlevel.
# Make sure that the script will "exit 0" on success or any other
# value on error.
#
# In order to enable or disable this script just change the execution
# bits.
#
# By default this script does nothing.

# Print the IP address
_IP=$(hostname -I) || true
if [ "$_IP" ]; then
  printf "My IP address is %s\n" "$_IP"
fi

. /home/pi/.cargo/env
cd /home/pi/UltrasonicPiPiano
./run.sh > /var/log/ultrasonic-pi.log 2>&1

exit 0
```

If the code doesn't start running on bootup, check the log at `/var/log/ultrasonic-pi.log` for error messages.

# Stopping the program from running

To stop the program from running in the background, run the following command:

```
sudo killall -9 ultrasonic_piano
```

