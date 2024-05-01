<!-- TOC -->
* [Oh](#oh)
  * [Donate](#donate)
  * [Profiles](#profiles)
    * [Desktop](#desktop)
    * [Tilling windows manager](#tilling-windows-manager)
    * [Groups](#groups)
* [Command line interface](#command-line-interface)
* [Documentation](#documentation)
* [Key bindings](#key-bindings)
  * [All prompts](#all-prompts)
  * [Text input](#text-input)
  * [Text prompts](#text-prompts)
  * [Select prompts](#select-prompts)
  * [Multi select prompts](#multi-select-prompts)
  * [Date select prompts](#date-select-prompts)
  * [Editor prompts](#editor-prompts)
* [Installation](#installation)
  * [Set keymap](#set-keymap)
  * [Partitioning](#partitioning)
  * [Update the pacman mirrors](#update-the-pacman-mirrors)
  * [Initialise the pacman keyring](#initialise-the-pacman-keyring)
  * [Install the base system](#install-the-base-system)
    * [Uefi system](#uefi-system)
    * [Bios system](#bios-system)
  * [Install the oh dependencies](#install-the-oh-dependencies)
    * [Setup mode](#setup-mode)
    * [After install mode](#after-install-mode)
  * [Generate fstab](#generate-fstab)
  * [Enter in the new system](#enter-in-the-new-system)
  * [Manage accounts](#manage-accounts)
    * [Create your account](#create-your-account)
    * [Generate root password](#generate-root-password)
    * [Generate your password](#generate-your-password)
    * [Add your account to sudoers file](#add-your-account-to-sudoers-file)
  * [Sign in](#sign-in)
    * [Configure rust](#configure-rust)
    * [Modify the pacman configuration](#modify-the-pacman-configuration)
    * [Refresh repositories](#refresh-repositories)
    * [Paru](#paru)
    * [Manager](#manager)
      * [From GitHub](#from-github)
      * [From Aur](#from-aur)
    * [Run the setup](#run-the-setup)
    * [Quit the fresh new system](#quit-the-fresh-new-system)
    * [Unmounted all partitions](#unmounted-all-partitions)
    * [Reboot](#reboot)
<!-- TOC -->

# Oh

It's a project to install archlinux manually.

It's requiring to know the archlinux system.

## Donate

![donate](https://raw.githubusercontent.com/otechdo/oh/main/donate.png)

[@donate](https://www.paypal.com/donate/?hosted_button_id=4EB2UW73HN8Q2)

## Profiles

### Desktop

| base                                                                     | extra                                                                                | supported |  
|--------------------------------------------------------------------------|--------------------------------------------------------------------------------------|-----------|
| [gnome](https://github.com/otechdo/oh/blob/main/oh/profiles/gnome)       | [gnome-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/gnome-extra)       | `yes`     |
| [kde](https://github.com/otechdo/oh/blob/main/oh/profiles/kde)           | [kde-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/kde-extra)           | `yes`     |
| [cinnamon](https://github.com/otechdo/oh/blob/main/oh/profiles/cinnamon) | [cinnamon-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/cinnamon-extra) | `yes`     |
| [budgie](https://github.com/otechdo/oh/blob/main/oh/profiles/budgie)     | [budgie-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/budgie-extra)     | `yes`     |
| [xfce](https://github.com/otechdo/oh/blob/main/oh/profiles/xfce)         | [xfce-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/xfce-extra)         | `yes`     |
| [lxqt](https://github.com/otechdo/oh/blob/main/oh/profiles/lxqt)         | [lxqt-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/lxqt-extra)         | `yes`     |

### Tilling windows manager

| base                                                                     | extra                                                                                | supported |  
|--------------------------------------------------------------------------|--------------------------------------------------------------------------------------|-----------|
| [i3](https://github.com/otechdo/oh/blob/main/oh/profiles/i3)             | [i3-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/i3-extra)             | `yes`     |
| [xmonad](https://github.com/otechdo/oh/blob/main/oh/profiles/xmonad)     | [xmonad-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/xmonad-extra)     | `yes`     |
| [bspwm](https://github.com/otechdo/oh/blob/main/oh/profiles/bspwm)       | [bspwm-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/bspwm-extra)       | `yes`     |
| [qtile](https://github.com/otechdo/oh/blob/main/oh/profiles/qtile)       | [qtile-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/qtile-extra)       | `yes`     |
| [hyprland](https://github.com/otechdo/oh/blob/main/oh/profiles/hyprland) | [hyprland-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/hyprland-extra) | `yes`     |
| [awesome](https://github.com/otechdo/oh/blob/main/oh/profiles/awesome)   | [awesome-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/awesome-extra)   | `yes`     |

### Groups

| base                                                                                 | extra                                                                                            | do                          | supported |
|--------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|-----------------------------|-----------|
| [admin](https://github.com/otechdo/oh/blob/main/oh/profiles/admin)                   | [admin-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/admin-extra)                   | Add cockpit full support    | `yes`     |
| [container](https://github.com/otechdo/oh/blob/main/oh/profiles/container)           | [container-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/container-extra)           | Add containers support      | `yes`     |
| [virtualization](https://github.com/otechdo/oh/blob/main/oh/profiles/virtualization) | [virtualization-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/virtualization-extra) | Add virtualization packages | `yes`     |
| [3d-printing](https://github.com/otechdo/oh/blob/main/oh/profiles/3d-printing)       | [3d-printing-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/3d-printing-extra)       | Add 3d printers support     | `yes`     |
| [electronic](https://github.com/otechdo/oh/blob/main/oh/profiles/electronic)         | [electronic-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/electronic-extra)         | Add electronic software     | `yes`     |
| [sound](https://github.com/otechdo/oh/blob/main/oh/profiles/sound)                   | [sound-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/electronic-extra)              | Add full sound driver       | `yes`     |
| [developer](https://github.com/otechdo/oh/blob/main/oh/profiles/developer)           | [developer-extra](https://github.com/otechdo/oh/blob/main/oh/profiles/developer-extra)           | Add developer toolkit       | `yes`     |


![zuu](https://github.com/otechdo/oh/actions/workflows/zuu.yml/badge.svg?branch=main)
![continuous](https://github.com/otechdo/oh/actions/workflows/continuous.yml/badge.svg?branch=main)

# Command line interface

| command                       | alias               | do                                        |
|-------------------------------|---------------------|-------------------------------------------|
| `oh`                          | `oh`                | Run installer in update mode              |
| `oh --setup`                  | `oh -s`             | Run installer in first install mode       |
| `oh --new`                    | `oh -n`             | Run installer in new config mode          |
| `oh --install`                | `oh -i`             | Run oh in packages installer mode         |
| `oh --deps`                   | `oh -d`             | Run oh in dependencies installer mode     |
| `oh --uninstall`              | `oh -u`             | Run oh in packages uninstaller mode       |
| `oh --mirrors`                | `oh -m`             | Run oh in mirrors updater mode            |
| `oh --check`                  | `oh -c`             | Run oh in check updates mode              |
| `oh --download`               | `oh -d`             | Run oh in download iso mode               |
| `oh --help`                   | `oh -h`             | Display the help message                  |
| `oh --cache`                  | `oh -C`             | Generate cache files                      |
| `oh --wiki`                   | `oh -w`             | Navigate to the wiki                      |
| `oh --news`                   | `oh -n`             | Navigate to the news                      |
| `oh --aur`                    | `oh -a`             | Navigate to the aur                       |
| `oh --forum`                  | `oh -f`             | Navigate to the forum                     |
| `oh --man`                    | `oh -m`             | Navigate to the man pages                 |
| `oh --vote pkg`               | `oh -v pkg`         | Vote for a aur package                    |
| `oh --search pkg`             | `oh -S pkg`         | Run oh in search package mode             |
| `oh --search pkg`             | `oh -S pkg`         | Run oh in search package mode             |
| `oh --create-usb device_path` | `oh -U devive_path` | Create usb with the current configuration |
| `os --list`                   | `os -l`             | List all boxes                            |
| `os --new`                    | `os -n`             | Create a box                              |
| `os --clean`                  | `os -c`             | Remove all created boxes                  |
| `os --help`                   | `os -h`             | Display help message                      |
| `os --upgrade`                | `os -U`             | Upgrade all created boxes                 |
| `os --stop box`               | `os -s box`         | Stop the box                              |
| `os --use box`                | `os -u box command` | Enter inside the box                      |
| `os --run box command`        | `os -r box command` | Run the command in the box                |

# Documentation

| Locale                                                   | completed     |
|----------------------------------------------------------|---------------|
| [fr](https://github.com/otechdo/oh/blob/main/oh/docs/fr) | `in progress` |
| [es](https://github.com/otechdo/oh/blob/main/oh/docs/es) | `no`          |
| [it](https://github.com/otechdo/oh/blob/main/oh/docs/it) | `no`          |
| [en](https://github.com/otechdo/oh/blob/main/README.md)  | `yes`         |

# Key bindings

This file lists all the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
|----------------------------------|-------------------------|
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Cancelling and interrupting a prompt have two different meanings.

Cancelling is defined specially for when the end user is allowed to skip a prompt.

The library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches
the `CanceledOperation` error transforming it into a `Ok(None)` result.

Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as
termination commands.

## Text input

These key bindings may be used with all prompts that ask the user for text
input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`].

The [`Editor`] prompt is not included because it opens a separate text editor for text input.

| **command**                         | **description**                                 |
|-------------------------------------|-------------------------------------------------|
| <kbd>character</kbd>                | Insert the character into the input.            |
| <kbd>left</kbd>                     | Move the cursor back one character.             |
| <kbd>right</kbd>                    | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>   | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>  | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                     | Move cursor to the start of the line.           |
| <kbd>end</kbd>                      | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                   | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd> | Delete one word to the right of the cursor.     |

\* Key bindings aren't supported on [`Select`] and [`MultiSelect`] prompts.

## Text prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
| <kbd>enter</kbd>     | Submit the current current text input.                        |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.       |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.     |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.      |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.    |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.   |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
| <kbd>enter</kbd>     | Submit the current highlighted option.                        |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Multi select prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
| <kbd>enter</kbd>     | Submit the options currently selected.                        |
| <kbd>space</kbd>     | Toggle the selection of the current highlighted option.       |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| <kbd>left</kbd>      | Deselect all options.                                         |
| <kbd>right</kbd>     | Select all options.                                           |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Date select prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.

| **command**                              | **description**                                               |
|------------------------------------------|---------------------------------------------------------------|
| <kbd>space bar</kbd> or <kbd>enter</kbd> | Submit the current highlighted date.                          |
| <kbd>up</kbd>                            | Move cursor one row up.                                       |
| <kbd>down</kbd>                          | Move cursor one row down.                                     |
| <kbd>left</kbd>                          | Move cursor one column to the left.                           |
| <kbd>right</kbd>                         | Move cursor one column to the right.                          |
| <kbd>k</kbd>                             | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>                             | Move cursor one row down when vim mode is enabled.            |
| <kbd>h</kbd>                             | Move cursor one column to the left when vim mode is enabled.  |
| <kbd>l</kbd>                             | Move cursor one column to the right when vim mode is enabled. |
| <kbd>ctrl</kbd> + <kbd>up</kbd>          | Move calendar back by one year.                               |
| <kbd>ctrl</kbd> + <kbd>down</kbd>        | Move calendar forward by one year.                            |
| <kbd>ctrl</kbd> + <kbd>left</kbd>        | Move calendar back by one month.                              |
| <kbd>ctrl</kbd> + <kbd>right</kbd>       | Move calendar forward by one month.                           |

## Editor prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                                |
|------------------|----------------------------------------------------------------|
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |

# Installation

Let's begin the [archlinux](https://archlinux.org) installation!

## Set keymap

```bash
loadkeys <keymap>
```

## Partitioning

```bash
cgdisk /dev/sda
```

| N° | Size  | Code | Name        | Formating sample           | Mount path      | Mount command                   | Description       |
|----|-------|------|-------------|----------------------------|-----------------|---------------------------------|-------------------|
| 1  | +1G   | EF00 | `/boot/efi` | `mkfs.fat -F 32 /dev/sda1` | `/mnt/boot/efi` | `mount /dev/sda1 /mnt/boot/efi` | The efi boot part |
| 2  | +4G   | 8300 | `/boot`     | `mkfs.xfs /dev/sda2`       | `/mnt/boot`     | `mount /dev/sda2 /mnt/boot`     | The boot part     |
| 3  | +100G | 8300 | `/`         | `mkfs.xfs /dev/sda3`       | `/mnt`          | `mount /dev/sda3 /mnt`          | The root part     |
| 4  | +50%  | 8300 | `/home`     | `mkfs.xfs /dev/sda4`       | `/mnt/home`     | `mount /dev/sda4 /mnt/home`     | The home part     |
| 5  | +50%  | 8300 | `/mnt/save` | `mkfs.ext4 /dev/sda5`      | `/mnt/mnt/save` | `mount /dev/sda5 /mnt/mnt/save` | The save part     |

## Update the pacman mirrors

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Initialise the pacman keyring

```bash
pacman-key --init && pacman-key --populate ohlinux
```

## Install the base system

Add your GPU driver and processor ucode package.

### Uefi system

```bash
pacstrap /mnt base base-devel linux linux-firmware less networkmanager efivars bootmgr xfsprogs os-prober fish feh pkgfile pacman-contrib ohlinux-wallpaper grub os-prober
```

### Bios system

```bash
pacstrap /mnt base base-devel linux linux-firmware networkmanager grub xfsprogs os-prober fish pkgfile less pacman-contrib feh wallutils ohlinux-wallpaper 
```

## Install the oh dependencies

`paru` it's also required.

### Setup mode

```bash
pacstrap /mnt w3m rustup wget rsync archiso git arch-wiki-lite archinstall man-pages arch-install-scripts aurpublish distrobox toolbox vim
```

### After install mode

```bash
sudo pacman -S --needed w3m rustup git wget archiso rsync git arch-wiki-lite archinstall man-pages arch-install-scripts aurpublish distrobox toolbox vim
```

## Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

## Enter in the new system

```bash
oh-chroot /mnt && cd ~
```

## Manage accounts

### Create your account

```bash
useradd -m -g wheel -c 'YOUR REAL NAME' -s <shell> <username>
```

### Generate root password

```bash
passwd root
```

### Generate your password

```bash
passwd <username>
```

### Add your account to sudoers file

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

## Sign in

```bash
su - <username>
```

### Configure rust

```bash
rustup default stable
```

### Modify the pacman configuration

```bash
sudo vim /etc/pacman.conf
```

### Refresh repositories

```bash
sudo pacman -Sy
```

### Paru

```bash
git clone https://aur.ohlinux.org/paru 
cd paru
makepkg -si 
cd ..
rm -rf paru
```

### Manager

#### From GitHub

```bash
git clone https://github.com/otechdo/oh 
cd oh 
make
sudo make install
```

#### From Aur

```bash
paru -Syu oh
```

### Run the setup

```bash
oh --setup
```

### Quit the fresh new system

```bash
exit
```

### Unmounted all partitions

```bash
umount -R /mnt
```

### Reboot

```bash
reboot
```