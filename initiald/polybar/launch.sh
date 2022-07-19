#!/usr/bin/env bash

# Terminate already running bar instances
killall -q polybar
# If all your bars have ipc enabled, you can also use 
polybar-msg cmd quit

# Launch bar1 and bar2
echo "---" | tee -a /tmp/polybar1.log /tmp/polybar2.log
polybar example 2>&1 | tee -a /tmp/polybar1.log & disown
polybar yui 2>&1 | tee -a /tmp/polybar2.log & disown
polybar center 2>&1 | tee -a /tmp/polybar2.log & disown
polybar table 2>&1 | tee -a /tmp/polybar2.log & disown
echo "Bars launched..."


#!/bin/bash

# Terminate already running bar instances
#killall -q polybar

# Wait until the processes have been shut down
#while pgrep -u $UID -x polybar >/dev/null; do sleep 1; done

#polybar -r bar &

#echo "Polybar launched..."
