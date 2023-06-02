# Easy WM setup

With this tool we can write in a few steps all the configuration files.
## Steps
- Run the `setup.sh` script
- select an image
- enjoy your WM

## Installing BSPWM on Debian
### Installing Debian
- Get the ISO file in the official site.
- Made a booteable pendrive
- Install debian
### Solving issues
- Source in Live instaler
  ```
  nano /etc/apt/sorces.list
  ```
  Comment cdrom line adding #
  ```
   # deb cdrom:[Debian_version_blah_blah.iso]
  ```
- Cannot mount CDROM

  ```
  sed -i '/cdrom/d' /etc/apt/sources.list
  ```
- User is not in the sudoers file

  **Note.** the word *"username"* in this file means the username, **is not a command**. 
  
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

