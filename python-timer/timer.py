#!/usr/bin/env python
from playsound import playsound
import time

time_to_wait = int(input("How long do you want this timer to be(in minutes)")) * 60

time.sleep(time_to_wait)

playsound('/var/local/timer_ringtones/ringtone.mp3')
