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
