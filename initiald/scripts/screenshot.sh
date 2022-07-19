#/bin/bash
name="Screenshot-"
date=`date +%Y-%m-%d.%H:%M:%S`
ext=".png"
`import -window root /home/juampa/Pictures/Screenshots/$name$date$ext`
echo $today
