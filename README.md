# bspwm-config
## Installing BSPWM on Debian
### Installing Debian
- Get the ISO file in the official site.
- Made a booteable pendrive
- Install debian
### Solving issues
- Cannot mount CDROM
  First, list your disks
  ```
  lsblk
  ```
  Look where is installed your system and then execute:
  Change the *sda* value from your correct partition name.
  ```
  mount -t vfat /dev/sdb /cdrom
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
