#!/bin/bash
echo Installing dependences
sudo apt install xorg bspwm polybar ranger kitty feh rofi
echo Done.
echo "$(tput bold setaf 2)First select a wallpaper$(tput sgr0)"

read -p "Path of your wallpaper: " WALLPAPER
./get_colors $WALLPAPER > colors.global
echo Done.
