<img src="https://raw.githubusercontent.com/otechdo/arch/main/arch/archlinux.svg" alt="archlinux" align="right" width="250">

Un installateur d'archlinux aisi qu'un manager pour les utilisateurs et utilisatrices avancés de linux.


- [@documentation](https://github.com/otechdo/arch/blob/main/arch/docs/)
    - [@en](https://raw.githubusercontent.com/otechdo/arch/main/README.md)
    - [@fr](https://github.com/otechdo/arch/blob/main/arch/docs/fr/README.md)
    - [@es](https://github.com/otechdo/arch/blob/main/arch/docs/es/README.md)
    - [@it](https://github.com/otechdo/arch/blob/main/arch/docs/it/README.md)
- [@archlinux](https://archlinux.org)
    - [@guide](https://wiki.archlinux.org/title/Installation_guide)
    - [@wiki](https://wiki.archlinux.org/)
    - [@download](https://archlinux.org/download/)
- [@arch](https://github.com/otechdo/arch/)
    - [@issues](https://github.com/otechdo/arch/issues)
    - [@discussions](https://discord.gg/jWHjkpRJPw)

##  Definir le clavier

```bash
loadkeys fr
```

##  Créer trois partitions:

* 1 1024MB  EFI partition   # ef00
* 2 4096MB  Linux partition # 8300
* 3 100%    Linux partition # 8300

```bash
cgdisk /dev/sda
```

## Formatage

### /boot/efi

```bash
mkfs.vfat -F 32 /dev/sda1
```

### /boot

```bash
mkfs.ext2 /dev/sda2
```

### /

```bash
mkfs.ext4 /dev/sda3
```

## Liste de toutes les partitions pour le montage

```bash
lsblk --fs
```

## Montage des partitions

```bash
mount /dev/sda3 /mnt
mkdir /mnt/boot
mount /dev/sda2 /mnt/boot
mkdir /mnt/boot/efi
mount /dev/sda1 /mnt/boot/efi
```

## Mettre a jour la liste des mirroirs

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Mise a jours des clef gpg de archlinux

```bash
pacman-key --init && pacman-key --populate archlinux
```

## Mise a jour de la base de données des clefs

```bash
pacman-key --refresh-keys
```

## Installation du système minimal pour arch 

```bash
pacstrap /mnt base base-devel wget git linux linux-firmware vim efibootmgr rustup sudo grub networkmanager w3m archiso reflector <shell> <ucode> <graphics_driver>
```

##  Generer le fichier fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

##  Entre dans le nouveau système 

```bash
arch-chroot /mnt && cd ~
```

## Création de votre compte

### Ajout de l'utilisateur

```bash
useradd -m -U -c 'REAL NAME' -s <shell> <username>
```

### Création d'un mot de passe pour root
```bash
passwd root
```

### Création d'un mot de passe pour votre compte

```bash
passwd <username>
```

### Ajout du compte au sudoers

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

## Se connecter en tant qu'utilisateur du nouveau compte

```bash
su - <username>
```

## Configuration de la toolchain

```bash
rustup default stable
```

## Activation du depôt multilib

```bash
vim /etc/pacman.conf
```

## Rafraichisemment des miroirs

```bash
sudo pacman -Sy
```

## Installation de yay

```bash
git clone https://aur.archlinux.org/yay && cd yay && makepkg -si && cd .. && rm -rf yay
```

## Installation de arch
 
### A partir de GitHub

```bash
git clone https://github.com/otechdo/arch && cd arch && make setup
```

### A partir de Crates.io

```bash
cargo install arch && install -m 755 "$HOME/.cargo/bin/arch" /usr/bin/arch && arch setup
```

### A partir d'Aur

```bash
paru -Syu manager && arch setup
```

## Setup the new arch

```bash
arch setup
```

## Desktop préconfigurés

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)
- [@xmonad](https://wiki.archlinux.org/title/xmonad)
- [@i3](https://wiki.archlinux.org/title/i3)

## Afin d'installer des paquets

```bash
arch --install-packages
```

## Afin d'installer des dépendances 

```bash
arch --install-dependencies
```

## Afin de supprimer des paquets

```bash
arch --remove-packages
```

## Afin de mettre a jour la liste des miroirs

```bash
arch --update-mirrors
```

## Verifier les mises à jours

```bash
arch --check-updates
```

## Afin de mettre à jour arch

```bash
arch --update
```

## Afin de mettre à jour et de redémarrer

```bash
arch --update-and-reboot
arch --update -r
arch -r --update
```

## Annuler le redémarrage

```bash
arch --cancel-reboot
```

## Mise à jour du cache des paquets

```bash
sudo arch --refresh-cache
```

## Télécharger les mise à jours seulement

```bash
arch --download-updates
```

## Quiter arch-chroot

```bash
exit
```

## Démontage des partitions

```bash
umount -R /mnt
```

## Redémarrage

```bash
reboot
```
