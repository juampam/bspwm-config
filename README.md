# bspwm-config
## Installing BSPWM on Debian
### Installing Debian
- Get the ISO file in the official site.
- Made a booteable pendrive
- Install debian
### Solving issues
- Cannot mount CDROM

  ```
  sed -i '/cdrom/d' /etc/apt/sources.list
  ```
- User is not in the sudoers file
  
  ```
  su -
  usermod -aG sudo username
  reboot
  ```
  or

  ```
  adduser username sudo
  ```
 ## Installing BSPWM
  comming soon xd
## Polybar config example
```
[bar/mainbar-bspwm]
monitor = ${env:MONITOR}
;monitor-fallback = HDMI1
width = 100%
height = 20
;offset-x = 1%
;offset-y = 1%
radius = 0.0
fixed-center = true
bottom = false
separator = 

background = ${colors.background}
foreground = ${colors.foreground}

line-size = 2
line-color = #f00

wm-restack = bspwm
override-redirect = true

; Enable support for inter-process messaging
; See the Messaging wiki page for more details.
enable-ipc = true

border-size = 0
;border-left-size = 0
;border-right-size = 25
;border-top-size = 0
;border-bottom-size = 25
border-color = #00000000

padding-left = 1
padding-right = 1

module-margin-left = 0
module-margin-right = 0

;https://github.com/jaagr/polybar/wiki/Fonts
font-0 = "UbuntuMono Nerd Font:size=10;2"
font-1 = "UbuntuMono Nerd Font:size=16;3"
font-2 = "Font Awesome 5 Free:style=Regular:pixelsize=8;1"
font-3 = "Font Awesome 5 Free:style=Solid:pixelsize=8;1"
font-4 = "Font Awesome 5 Brands:pixelsize=8;1"

modules-left = bspwm xwindow
modules-center = 
modules-right = arrow1 networkspeedup networkspeeddown arrow2 memory2 arrow3 cpu2 arrow2 pavolume arrow3 arch-aur-updates arrow2 date

tray-detached = false
tray-offset-x = 0
tray-offset-y = 0
tray-position = right
tray-padding = 2
tray-maxsize = 20
tray-scale = 1.0
tray-background = ${colors.background}

scroll-up = bspwm-desknext
scroll-down = bspwm-deskprev
```
