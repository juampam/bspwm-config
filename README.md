# Easy WM setup
<div align="center">

![Commit Activity](https://img.shields.io/github/commit-activity/w/juampam/bspwm-config?style=for-the-badge)
![Contributors](https://img.shields.io/github/contributors/juampam/bspwm-config?style=for-the-badge)
![Languages](https://img.shields.io/badge/Languages-Rust-orange?style=for-the-badge)
![repo size](https://img.shields.io/github/repo-size/juampam/bspwm-config?style=for-the-badge)

</div>     
With this tool we can write in a few steps all the configuration files.
## Steps
- Run the `setup.sh` script
- select an image
- enjoy your WM

After that, based on the selected image, the prograsm will set the colors to the next applications:
- Neovim
- Kitty
- Polybar
- BSPWM
- Rofi
- Ranger

Note:
Some of these actions are not ready yet.

## Installing manual

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
🦀
