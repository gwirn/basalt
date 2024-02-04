#!/bin/bash
if [ `wmctrl -l | grep -c "Mozilla Firefox"` != 0 ]  
then
    wmctrl -a "Mozilla Firefox"
else
    firefox &
fi
