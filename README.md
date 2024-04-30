[TOC]



# Achlinux installer

It's a project to install archlinux manually.

It's require to know archlinux system.

[![zuu](https://github.com/otechdo/arch/actions/workflows/zuu.yml/badge.svg)](https://github.com/otechdo/arch/actions/workflows/zuu.yml)

[@official documentation](https://wiki.archlinux.org/title/Installation_guide)

## Set keymap

```bash
loadkeys <keymap>
```

## Partitioning

1. 1024MB  EFI partition         # ef00      /boot/efi    
2. 4096MB  Linux partition    # 8300     /boot
3. 100%       Linux partition    # 8300     /

```bash
cgdisk /dev/sda
```

## Formatting

> /boot/efi

```bash
mkfs.vfat -F 32 /dev/sda1
```

> /boot

```bash
mkfs.ext2 /dev/sda2
```

> /

```bash
mkfs.ext4 /dev/sda3
```

# Mounting

## The root partition

```bash
mount /dev/sda3 /mnt
```

## Create the boot mount point

```bash
mkdir /mnt/boot
```

## Mounting the boot partition

```bash
mount /dev/sda2 /mnt/boot
```

## Create the EFI mount point

```bash
mkdir /mnt/boot/efi
```

## Mounting the EFI partition

```bash
mount /dev/sda1 /mnt/boot/efi
```

# Pacman 

## Update mirrorlist

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Initialise keyring

```bash
pacman-key --init && pacman-key --populate archlinux
```

# GPU

## Nvidia code name

[NVIDIA - ArchWiki](https://wiki.archlinux.org/title/NVIDIA) 

| **Code name**                                                 | **Official Name**                                           | **Nvidia 3D object codename** |
| ------------------------------------------------------------- | ----------------------------------------------------------- | ----------------------------- |
| [NV04](https://nouveau.freedesktop.org/CodeNames.html#NV04)   | Riva TNT, TNT2                                              | Fahrenheit                    |
| [NV10](https://nouveau.freedesktop.org/CodeNames.html#NV10)   | GeForce 256, GeForce 2, GeForce 4 MX                        | Celsius                       |
| [NV20](https://nouveau.freedesktop.org/CodeNames.html#NV20)   | GeForce 3, GeForce 4 Ti                                     | Kelvin                        |
| [NV30](https://nouveau.freedesktop.org/CodeNames.html#NV30)   | GeForce 5 / GeForce FX                                      | Rankine                       |
| [NV40](https://nouveau.freedesktop.org/CodeNames.html#NV40)   | GeForce 6, GeForce 7                                        | Curie                         |
| [NV50](https://nouveau.freedesktop.org/CodeNames.html#NV50)   | GeForce 8, GeForce 9, GeForce 100, GeForce 200, GeForce 300 | Tesla                         |
| [NVC0](https://nouveau.freedesktop.org/CodeNames.html#NVC0)   | GeForce 400, GeForce 500                                    | Fermi                         |
| [NVE0](https://nouveau.freedesktop.org/CodeNames.html#NVE0)   | GeForce 600, GeForce 700, GeForce GTX Titan                 | Kepler                        |
| [NV110](https://nouveau.freedesktop.org/CodeNames.html#NV110) | GeForce 750, GeForce 900                                    | Maxwell                       |
| [NV130](https://nouveau.freedesktop.org/CodeNames.html#NV130) | GeForce 1060, GeForce 1070                                  | Pascal                        |
| [NV140](https://nouveau.freedesktop.org/CodeNames.html#NV140) | NVIDIA Titan V                                              | Volta                         |
| [NV160](https://nouveau.freedesktop.org/CodeNames.html#NV160) | GeForce RTX 2060, GeForce GTX 1660                          | Turing                        |
| [NV170](https://nouveau.freedesktop.org/CodeNames.html#NV170) | GeForce RTX 3060, GeForce RTX 3070                          | Ampere                        |
| [NV190](https://nouveau.freedesktop.org/CodeNames.html#NV190) | GeForce RTX 4060, GeForce RTX 4070                          | Ada Lovelace                  |

| **Code name** | **Official Name**                                                                                                                                                |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| NV10          | GeForce 256<br>Quadro                                                                                                                                            |
| NV11          | GeForce2 Go, MX<br>Quadro2 (EX, MXR)                                                                                                                             |
| NV15          | GeForce2 GTS, Pro, Ti, Ultra<br>Quadro2 Pro                                                                                                                      |
| NV17          | GeForce4 MX 420, MX 440, MX 440-SE (AGP 4x), MX 460<br>Quadro4 500 XGL, 550 XGL, Quadro NVS (100, 200)                                                           |
| NV18          | GeForce4 MX 440-SE (AGP 8x), MX 440-8x, MX 4000, 420 Go, 440 Go, 448 Go, 460 Go, 488 Go, GeForce PCX 4300<br>Quadro4 380 XGL, 580 XGL, Quadro NVS (50, 280, 400) |
| NV1A          | GeForce2 IGP                                                                                                                                                     |
| NV1F          | GeForce4 MX IGP                                                                                                                                                  |

| **Code name** | **Official Name**                                                       |
| ------------- | ----------------------------------------------------------------------- |
| NV20          | GeForce3 (Ti), Quadro DCC                                               |
| NV25          | GeForce4 Ti 4200, Ti 4400, Ti 4600<br>Quadro4 700 XGL, 750 XGL, 900 XGL |
| NV28          | GeForce4 Ti 4200-8X, Ti 4800 (SE), 4200 Go<br>Quadro4 780 XGL, 980 XGL  |
| NV2A          | XBOX GPU                                                                |

| NV30 | GeForce FX 5800 (Ultra)  <br>Quadro FX (1000, 2000)                                                          |
| ---- | ------------------------------------------------------------------------------------------------------------ |
| NV31 | GeForce FX 5600 (Ultra, XT, Go)<br>Quadro FX 700                                                             |
| NV34 | GeForce FX 5100 Go, 5200 (Ultra, Go), 5300, 5500, GeForce PCX 5300<br>Quadro FX (330, 500, 600 PCI), NVS 280 |
| NV35 | GeForce FX 5900 (ZT, XT, SE), 5950 Ultra, GeForce PCX 5900, 5950<br>Quadro FX (1300, 3000, 3000G)            |
| NV36 | GeForce FX 5700 (Ultra, VE, LE, Go), 5750, GeForce PCX 5750<br>Quadro FX 1100##                              |

| **Code name** | **Official Name**                                                                                                                      |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| NV40          | GeForce 6800 (Ultra, GT, GS, XT, LE, GTO)<br>Quadro FX 4000 (SDI), Quadro FX 3400, 4400                                                |
| NV41          | GeForce 6800 (XT, GTO, Go Ultra)<br>Quadro FX 1400                                                                                     |
| NV42          | GeForce 6800 (GS, Go)<br>Quadro FX (3450, 4000 SDI)                                                                                    |
| NV43          | GeForce 6200, 6500, 6600 (LE, GT, Go, Go TE, Go Ultra), 6700 XL<br>Quadro FX (540, 540M, 550), NVS 440                                 |
| NV44          | GeForce 6200 (TC, Go), 6250 Go, 6400 Go, 7100 GS<br>Quadro NVS 285                                                                     |
| NV46 (G72)    | GeForce 7200 (GS, Go), 7300 (LE, GS, Go), 7400 Go, 7500<br>Quadro FX 350(M), NVS (110M, 120M, 300M, 510M)                              |
| NV47 (G70)    | GeForce 7800 (GS, GT, GTX, Go, Go GTX)<br>Quadro FX 4500 (SDI, X2)                                                                     |
| NV49 (G71)    | GeForce 7900 (GS, GT, GTO, GTX, GX2, Go, Go GTX), 7950 (GT, GX2, Go GTX)<br>Quadro FX (1500, 1500M, 3500, 5500, 550 SDI, 2500M, 3500M) |
| NV4A (NV44A)  | GeForce 6200 AGP                                                                                                                       |
| NV4B (G73)    | GeForce 7300 GT, 7600 (GS, GT, Go, Go GT), 7700 Go<br>Quadro FX (550M, 560, 560M)                                                      |
| NV4C (MCP61)  | GeForce 6150LE / nForce 400/405, GeForce 6150SE<br>Quadro NVS 210s / nForce 430                                                        |
| NV4E (C51)    | GeForce 6100 (Go) / nForce 410/430, 6150 (Go) / nForce 430                                                                             |
| NV63 (MCP73)  | GeForce 7050/7100/7150 / nForce 630i                                                                                                   |
| NV67 (MCP67)  | GeForce 7000M / nForce 610M, GeForce 7150M / nForce 630M                                                                               |
| NV68 (MCP68)  | GeForce 7025/7050 / nForce 630a                                                                                                        |

| **Code name**      | **Official Name**                                                                                                                                                                                                                                                  |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| NV50 (G80)         | GeForce 8800 (GTS, GTX, Ultra)<br>Quadro FX (4600 (SDI), 5600)                                                                                                                                                                                                     |
| NV84 (G84)         | GeForce 8600 (GT, GTS, M GT, M GS), 8700M GT, GeForce 9500M GS, 9650M GS<br>Quadro FX (370, 570, 570M, 1600M, 1700), NVS 320M                                                                                                                                      |
| NV86 (G86)         | GeForce 8300 GS, 8400 (GS, M G, M GS, M GT), 8500 GT, GeForce 9300M G<br>Quadro FX 360M, NVS (130M, 135M, 140M, 290)                                                                                                                                               |
| NV92 (G92)         | GeForce 8800 (GT, GS, GTS 512, M GTS, M GTX)<br>GeForce 9600 GSO, 9800 (GT, GTX, GTX+, GX2, M GT, M GTX)<br>GeForce GTS 150(M), GTS 160M, GTS 240, GTS 250, GTX (260M, 280M, 285M), GT (330, 340)<br>Quadro FX (2800M, 3600M, 3700, 3700M, 3800M, 4700 X2), VX 200 |
| NV94 (G94)         | GeForce 9600 (GSO 512, GT, S), 9700M GTS, 9800M GTS, GeForce G 110M, GT 130(M), GT 140<br>Quadro FX (1800, 2700M)                                                                                                                                                  |
| NV96 (G96)         | GeForce 9400 GT, 9500 (GT, M G), 9600 (M GS, M GT), 9650M GT, 9700M GT<br>GeForce G 102M, GT 120<br>Quadro FX (380, 580, 770M, 1700M)                                                                                                                              |
| NV98 (G98)         | GeForce 8400 GS, GeForce 9200M GS, 9300 (GE, GS, M GS)<br>GeForce G 100, G 105M<br>Quadro FX (370 LP, 370M), NVS (150M, 160M, 295, 420, 450)                                                                                                                       |
| NVA0 (GT200)       | GeForce GTX (260, 275, 280, 285, 295)<br>Quadro CX, FX (3800, 4800, 5800)                                                                                                                                                                                          |
| NVA3 (GT215)       | GeForce GT (240, 320, 335M), GTS (250M, 260M, 350M, 360M)<br>Quadro FX 1800M                                                                                                                                                                                       |
| NVA5 (GT216)       | GeForce GT (220, 230M, 240M, 325M, 330M), 315<br>Quadro 400, FX 880M, NVS 5100M                                                                                                                                                                                    |
| NVA8 (GT218)       | GeForce 8400 GS, ION 2, GeForce 205, 210, G 210M, 305M, 310(M), 405<br>Quadro FX (380 LP, 380M), NVS (300, 2100M, 3100M)                                                                                                                                           |
| NVAA (MCP77/MCP78) | GeForce 8100, 8200, 8300 mGPU / nForce 700a series, 8200M G                                                                                                                                                                                                        |
| NVAC (MCP79/MCP7A) | ION, GeForce 9300, 9400 mGPU / nForce 700i series, 8200M G, 9100M, 9400M (G)                                                                                                                                                                                       |
| NVAF (MCP89)       | GeForce 320M                                                                                                                                                                                                                                                       |

| **Code name** | **Official Name**                                                                                                               |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| NVC0 (GF100)  | GeForce GTX (465, 470, 480, 480M)<br>Quadro 4000, 5000[M] (??), 6000                                                            |
| NVC1 (GF108)  | GeForce GT (415M, 420, 420M, 425M, 430, 435M, 520M, 525M, 530, 540M, 550M, 555M, 620, 630M, 635M, 640M LE)<br>Quadro 600, 1000M |
| NVC3 (GF106)  | GeForce GT (440, 445M, 545, 555M, 630M, 635M), GTS 450, GTX 460M<br>Quadro 2000 (D), 2000M                                      |
| NVC4 (GF104)  | GeForce GTX (460, 460 SE, 470M, 485M)<br>Quadro 5000M (??)                                                                      |
| NVC8 (GF110)  | GeForce GTX (560 Ti OEM, 570, 580, 590)<br>Quadro 3000M, 4000M, 5010M                                                           |
| NVCE (GF114)  | GeForce GTX (460 v2, 560, 560 Ti, 570M, 580M, 670M, 675M)                                                                       |
| NVCF (GF116)  | GeForce GTS 450 v2, GTX (550 Ti, 560M)                                                                                          |
| NVD7 (GF117)  | Geforce GT 620M, 625M, (some) 630M, 710M, 720M                                                                                  |
| NVD9 (GF119)  | GeForce 410M, 510 (?), GT (520, 520M, 520MX, 610), 610M<br>Quadro NVS 4200M                                                     |

| **Code name**  | **Official Name**                                                                                                                                                    |
| -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| NVE4 (GK104)   | GeForce GTX (660 Ti, 670[M], 680[M], 690, 760, 760 Ti, 770, 775M, 780M, 860M)<br>Quadro K3000[M], K3100M, K4000[M], K4100[M], K5000[M], K5100M, Tesla K10            |
| NVE7 (GK107)   | GeForce GT (640[M], 645M, 650M, 710M, 720M, 730M, 740[M], 745M, 750M, 755M), GTX (650, 660M)<br>Quadro 410, K500[M], K600, K1000[M], K1100M, K2000[M], NVS 510, 1000 |
| NVE6 (GK106)   | GeForce GTX (645, 650 Ti, 660, 760M, 765M, 770M)<br>Quadro K2100M, K4000                                                                                             |
| NVF0 (GK110)   | GeForce GTX 780, Titan<br>Tesla K20, Quadro K6000                                                                                                                    |
| NVF1 (GK110B)  | GeForce GTX 780 Ti, Titan Z<br>Tesla K40                                                                                                                             |
| NV106 (GK208B) | GeForce GT 720                                                                                                                                                       |
| NV108 (GK208)  | GeForce GT 630, 635, 640, 710M, 720M, 730M, 735M, 740M, 920M<br>Quadro K510M, K610M                                                                                  |
| NVEA (GK20A)   | Tegra K1                                                                                                                                                             |
| NV??? (GK210)  | Tesla K80                                                                                                                                                            |

| **Code name** | **Official Name**                                                                                                              |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| NV117 (GM107) | GeForce GTX (745, 750, 750 Ti, 840M, 845M, 850M, 860M, 950M, 960M)<br>Quadro K620, K1200, K2200, M1000M, M1200M; GRID M30, M40 |
| NV118 (GM108) | GeForce 830M, 840M, 930M, 940M[X]                                                                                              |
| NV120 (GM200) | GeForce GTX Titan X                                                                                                            |
| NV124 (GM204) | GeForce GTX (970, 980)                                                                                                         |
| NV126 (GM206) | GeForce GTX (950, 960)                                                                                                         |
| NV12B (GM20B) | Tegra X1                                                                                                                       |

| **Code name** | **Official Name**                         |
| ------------- | ----------------------------------------- |
| NV132 (GP102) | NVIDIA Titan (X, Xp), GeForce GTX 1080 Ti |
| NV134 (GP104) | GeForce GTX (1070, 1080)                  |
| NV136 (GP106) | GeForce GTX 1060                          |
| NV137 (GP107) | GeForce GTX (1050, 1050 Ti)               |
| NV138 (GP108) | GeForce GT 1030                           |

| NV140 (GV100) | NVIDIA Titan V, NVIDIA Quadro GV100 |
| ------------- | ----------------------------------- |

| **Code name** | **Official Name**                                   |
| ------------- | --------------------------------------------------- |
| NV162 (TU102) | NVIDIA Titan RTX, GeForce RTX 2080 Ti               |
| NV164 (TU104) | GeForce RTX (2070 Super, 2080, 2080 Super)          |
| NV166 (TU106) | GeForce RTX (2060, 2060 Super, 2070)                |
| NV168 (TU116) | GeForce GTX (1650 Super, 1660, 1660 Ti, 1660 Super) |
| NV167 (TU117) | GeForce GTX 1650                                    |

| **Code name** | **Official Name**                        |
| ------------- | ---------------------------------------- |
| NV172 (GA102) | GeForce RTX (3080, 3090)                 |
| NV174 (GA104) | GeForce RTX (3060 Ti, 3070, 3080 Mobile) |
| NV176 (GA106) | GeForce RTX (3050, 3060)                 |
| NV177 (GA107) | GeForce RTX 3050                         |

| **Code name** | **Official Name**           |
| ------------- | --------------------------- |
| NV192 (AD102) | GeForce RTX 4090            |
| NV193 (AD103) | GeForce RTX 4080            |
| NV194 (AD104) | GeForce RTX (4070, 4070 Ti) |
| NV196 (AD106) | GeForce RTX 4060 Ti         |
| NV197 (AD107) | GeForce RTX 4060            |

| Driver name                                      | Kernel                      | Base driver       | OpenGL             | OpenGL (multilib)        |
| ------------------------------------------------ | --------------------------- | ----------------- | ------------------ | ------------------------ |
| Maxwell (NV110) series and newer                 | linux or linux-lts          | nvidia            | nvidia-utils       | lib32-nvidia-utils       |
| Maxwell (NV110) series and newer                 | not linux and not linux-lts | nvidia-dkms       | nvidia-utils       | lib32-nvidia-utils       |
| Kepler (NVE0) series                             | any                         | nvidia-470xx-dkms | nvidia-470xx-utils | lib32-nvidia-470xx-utils |
| GeForce 400/500/600 series cards [NVCx and NVDx] | any                         | nvidia-390xx-dkms | nvidia-390xx-utils | lib32-nvidia-390xx-utils |
| Tesla (NV50/G80-90-GT2XX)                        | any                         | nvidia-340xx-dkms | nvidia-340xx-utils | lib32-nvidia-340xx-utils |

## Amdgpu code name

[AMDGPU - ArchWiki](https://wiki.archlinux.org/title/AMDGPU)

| Family           | Chipset name                                | Microarchitecture[[4]](https://wiki.gentoo.org/wiki/AMDGPU#cite_note-4) | ISA[[5]](https://wiki.gentoo.org/wiki/AMDGPU#cite_note-5) | Product name                                                 |
| ---------------- | ------------------------------------------- | ------------------------------------------------------------ | --------------------------------------------------------- | ------------------------------------------------------------ |
| Southern Islands | CAPE VERDE, PITCAIRN, TAHITI, OLAND, HAINAN | GCN1.0+                                                      | DCE 6.x                                                   | HD7750-HD7970, R9 270, R9 280, R9 370X, R7 240, R7 250       |
| Sea Islands      | BONAIRE, KABINI, KAVERI, HAWAII, MULLINS    | GCN2.x                                                       | DCE 8.x                                                   | HD7790, R7 260, R9 290, R7 360, R9 390                       |
| Volcanic Islands | CARRIZO, FIJI, STONEY, TONGA, TOPAZ, WANI   | GCN3.x                                                       | DCE 10/11.x                                               | R9 285, R9 380, R9 380X, R9 Fury, R9 Nano, R9 Fury X, Pro Duo |
| Arctic Islands   | POLARIS10/11/12, VEGAM                      | GCN4.x                                                       | DCE 11.2                                                  | RX 460, RX 470, RX 480, RX 540, RX 550, RX 560, RX 570, RX 580, RX 590, Pro WX 3200 |
| Vega             | VEGA10/11/12/20                             | GCN5.x                                                       | DCE 12.x                                                  | RX Vega 56, RX Vega 64, Radeon Vega II, Radeon VII           |
| Vega             | RAVEN                                       | GCN5.x                                                       | DCN 1.0                                                   | Raven Ridge APU series                                       |
| Vega             | RENOIR                                      | GCN5.x                                                       | DCN 2.1                                                   | Renoir, Lucienne, and Cezanne APU series                     |
| Navi             | NAVI10/14                                   | RDNA                                                         | DCN 2.0                                                   | RX 5500, RX 5500 XT, RX 5600, RX 5600 XT, RX 5700, RX 5700 XT |
| Navi             | NAVI21/22/23/24                             | RDNA2                                                        | DCN 3.0                                                   | RX 6500 XT, RX 6600, RX 6600 XT, RX 6650 XT, RX 6700, RX 6700 XT, RX 6750 XT, RX 6800, RX 6800 XT, RX 6900 XT, RX 6950 XT |

## Drivers installation

#### Nvidia

##### Maxwell (NV110) series and newer

###### Base driver

```bash
pacstrap /mnt nvidia-dkms linux linux-firmware
```

###### OpenGL

```bash
pacstrap /mnt nvidia-utils linux linux-firmware
```

#### Kepler (NVE0) series

##### Base driver

```bash
pacstrap /mnt nvidia-470xx-dkms linux linux-firmware
```

##### OpenGL

```bash
pacstrap /mnt nvidia-470xx-utils linux linux-firmware
```

##### GeForce 400/500/600 series cards [NVCx and NVDx]

###### Base driver

```bash
pacstrap /mnt nvidia-390xx-dkms
```

###### OpenGL

```bash
pacstrap /mnt nvidia-390xx-utils
```

##### Tesla (NV50/G80-90-GT2XX)

###### Base driver

```bash
pacstrap /mnt nvidia-340xx-dkms linux linux-firmware
```

###### OpenGL

```bash
pacstrap /mnt nvidia-340xx-utils linux linux-firmware
```

#### AMD

##### Base driver

```bash
pacstrap /mnt xf86-video-amdgpu linux linux-firmware mesa
```

##### Vulkan support

```bash
pacstrap /mnt linux linux-firmware mesa vulkan-radeon
```

##### Accelerated video decoding support

```bash
pacstrap /mnt mesa-vdpau libva-mesa-driver lib32-libva-mesa-driver lib32-mesa-vdpau 
```

## Minimum system

```bash
pacstrap /mnt base base-devel pacman-contrib linux linux-firmware vim efibootmgr sudo grub networkmanager reflector amd-ucode|intel-ucode rustup
```

##  Arch dependencies

```bash
pacstrap /mnt
```



# Installer key bindings

This file lists all of the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
| -------------------------------- | ----------------------- |
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Cancelling and interrupting a prompt have two different meanings. Cancelling is defined specially for when the end user is allowed to skip a prompt, the library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result. Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. The [`Editor`] prompt is not included because it opens a separate text editor for text input.

| **command**                         | **description**                                 |
| ----------------------------------- | ----------------------------------------------- |
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

\* Key bindings not supported on [`Select`] and [`MultiSelect`] prompts.

## Text prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                              |
| -------------------- | ------------------------------------------------------------ |
| <kbd>enter</kbd>     | Submit the current current text input.                       |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.      |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.    |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.     |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.   |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.  |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                              |
| -------------------- | ------------------------------------------------------------ |
| <kbd>enter</kbd>     | Submit the current highlighted option.                       |
| <kbd>up</kbd>        | Move cursor one row up.                                      |
| <kbd>down</kbd>      | Move cursor one row down.                                    |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.             |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.           |
| <kbd>page up</kbd>   | Move cursor one page up.                                     |
| <kbd>page down</kbd> | Move cursor one page down.                                   |
| <kbd>home</kbd>      | Move cursor to the first option.                             |
| <kbd>end</kbd>       | Move cursor to the last option.                              |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Multi select prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                              |
| -------------------- | ------------------------------------------------------------ |
| <kbd>enter</kbd>     | Submit the options currently selected.                       |
| <kbd>space</kbd>     | Toggle the selection of the current highlighted option.      |
| <kbd>up</kbd>        | Move cursor one row up.                                      |
| <kbd>down</kbd>      | Move cursor one row down.                                    |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.             |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.           |
| <kbd>page up</kbd>   | Move cursor one page up.                                     |
| <kbd>page down</kbd> | Move cursor one page down.                                   |
| <kbd>home</kbd>      | Move cursor to the first option.                             |
| <kbd>end</kbd>       | Move cursor to the last option.                              |
| <kbd>left</kbd>      | Deselect all options.                                        |
| <kbd>right</kbd>     | Select all options.                                          |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Date select prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.

| **command**                              | **description**                                              |
| ---------------------------------------- | ------------------------------------------------------------ |
| <kbd>space bar</kbd> or <kbd>enter</kbd> | Submit the current highlighted date.                         |
| <kbd>up</kbd>                            | Move cursor one row up.                                      |
| <kbd>down</kbd>                          | Move cursor one row down.                                    |
| <kbd>left</kbd>                          | Move cursor one column to the left.                          |
| <kbd>right</kbd>                         | Move cursor one column to the right.                         |
| <kbd>k</kbd>                             | Move cursor one row up when vim mode is enabled.             |
| <kbd>j</kbd>                             | Move cursor one row down when vim mode is enabled.           |
| <kbd>h</kbd>                             | Move cursor one column to the left when vim mode is enabled. |
| <kbd>l</kbd>                             | Move cursor one column to the right when vim mode is enabled. |
| <kbd>ctrl</kbd> + <kbd>up</kbd>          | Move calendar back by one year.                              |
| <kbd>ctrl</kbd> + <kbd>down</kbd>        | Move calendar forward by one year.                           |
| <kbd>ctrl</kbd> + <kbd>left</kbd>        | Move calendar back by one month.                             |
| <kbd>ctrl</kbd> + <kbd>right</kbd>       | Move calendar forward by one month.                          |

## Editor prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                              |
| ---------------- | ------------------------------------------------------------ |
| <kbd>e</kbd>     | Open the editor.                                             |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |

# Installation

## GPU driver

```bash
pacstrap /mnt driver
```

## Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

## Enter in the new system

```bash
arch-chroot /mnt && cd ~
```

### Manage accounts

#### Create your account

```bash
useradd -m -g wheel -c 'YOUR REAL NAME' -s <shell> <username>
```

#### Generate root password

```bash
passwd root
```

#### Generate your password

```bash
passwd <username>
```

#### Add your account to sudoers file

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

### Sign in

```bash
su - <username>
```

#### Create icons directory

```bash
mkdir .icons
```

#### Download arch icon

```bash
cd .icons
wget https://raw.githubusercontent.com/otechdo/arch/main/arch/icons/arch.png
```

#### Configure rust

```bash
rustup default stable
```

#### Modify pacman.conf

```bash
sudo vim /etc/pacman.conf
```

#### Refresh repositories

```bash
sudo pacman -Sy
```

#### Installation of paru

```bash
git clone https://aur.archlinux.org/paru 
cd paru
makepkg -si 
cd ..
rm -rf paru
```

### Installation of arch

#### From GitHub

```bash
git clone https://github.com/otechdo/arch 
cd arch 
make
sudo make install
```

#### From Crates.io

```bash
cargo install arch 
install -m 755 "$HOME/.cargo/bin/arch" /usr/bin/arch
```

#### From Aur

```bash
yay -Syu manager
```

#### Arch desktop configured

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)
- [@xmonad](https://wiki.archlinux.org/title/xmonad)
- [@i3](https://wiki.archlinux.org/title/i3)

### Run the setup

```bash
arch --setup
```

#### Rebuild a complete arch

```bash
arch --new-config
```

#### Quit the fresh new system

```bash
exit
```

#### Unmounted all partitions

```bash
umount -R /mnt
```

#### Reboot

```bash
reboot
```



# Command line interface 

## Arch management

### Install all selected packages

```bash
arch --install
arch -S <pkg> <pkg>
```

### Setup a new arch

```bash
arch -i
```

```bash
arch --setup
```

### Setup a new configuration

```bash
arch --setup-new-config
```

### Remove packages

```bash
arch -R <pkg> <pkg>
```

```bash
arch --uninstall
```

### Install new packages

```bash
arch -S <pkg> <pkg>
```

```bash
arch --install
```

### Update mirrorlist

```bash
arch -M
```

```bash
arch --mirrors
```

### Check updates

```bash
arch -C
```

```bash
arch --check
```

### Install packages as dependencies

```bash
arch -d
```

```bash
arch --deps
```

### Update archlinux

```bash
arch
```

```bash
arch -u
```

```bash
arch --update
```

### Search a package

```bash
arch -s <pkg>
```

```bash
arch --search <pkg>
```

### Show arch current version

```bash
arch -v
```

```bash
arch --version
```

### Download updates

```bash
arch -d
```

```bash
arch --download-updates
```

### Show help message

```bash
arch -h
```

```bash
arch --help
```

### Cancel the upgrade reboot

```bash
arch -x
```

```bash
arch --cancel
```

### Upgrade the system and reboot

```bash
arch -U
```

```bash
arch --upgrade
```

### Generate arch packages cache

```bash
arch -c
```

```bash
arch --cache
```

## Navigate on websites

### News

```bash
arch -n
```

```bash
arch --news
```

### Aur

```bash
arch -a
```

```bash
arch --aur
```

### Forum

```bash
arch -f
```

```bash
arch --forum
```

### Man

```bash
arch -m
```

```bash
arch --man
```

```bash
arch --woman
```

### Wiki

```bash
arch -w
```

```bash
arch --wiki
```

# Distrobox management

## Distrobox containers 

Distrobox guests tested successfully with the following container images:

| Distro                | Version                                            | Images                                                                                                                                                                                                                                                                                                                  |
| --------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AlmaLinux (Toolbox)   | 8<br>9                                             | quay.io/toolbx-images/almalinux-toolbox:8<br>quay.io/toolbx-images/almalinux-toolbox:9<br>quay.io/toolbx-images/almalinux-toolbox:latest                                                                                                                                                                                |
| Alpine (Toolbox)      | 3.16<br>3.17<br>3.18<br>edge                       | quay.io/toolbx-images/alpine-toolbox:3.16<br>quay.io/toolbx-images/alpine-toolbox:3.17<br>quay.io/toolbx-images/alpine-toolbox:3.18<br>quay.io/toolbx-images/alpine-toolbox:edge<br>quay.io/toolbx-images/alpine-toolbox:latest                                                                                         |
| AmazonLinux (Toolbox) | 2<br>2022                                          | quay.io/toolbx-images/amazonlinux-toolbox:2<br>quay.io/toolbx-images/amazonlinux-toolbox:2023<br>quay.io/toolbx-images/amazonlinux-toolbox:latest                                                                                                                                                                       |
| Archlinux (Toolbox)   |                                                    | quay.io/toolbx/arch-toolbox:latest                                                                                                                                                                                                                                                                                      |
| Bazzite Arch          |                                                    | ghcr.io/ublue-os/bazzite-arch:latest<br>ghcr.io/ublue-os/bazzite-arch-gnome:latest                                                                                                                                                                                                                                      |
| Centos (Toolbox)      | stream8<br>stream9                                 | quay.io/toolbx-images/centos-toolbox:stream8<br>quay.io/toolbx-images/centos-toolbox:stream9<br>quay.io/toolbx-images/centos-toolbox:latest                                                                                                                                                                             |
| Debian (Toolbox)      | 10<br>11<br>12<br>testing<br>unstable<br>          | quay.io/toolbx-images/debian-toolbox:10<br>quay.io/toolbx-images/debian-toolbox:11<br>quay.io/toolbx-images/debian-toolbox:12<br>quay.io/toolbx-images/debian-toolbox:testing<br>quay.io/toolbx-images/debian-toolbox:unstable<br>quay.io/toolbx-images/debian-toolbox:latest                                           |
| Fedora (Toolbox)      | 37<br>38<br>39<br>Rawhide                          | registry.fedoraproject.org/fedora-toolbox:37<br>registry.fedoraproject.org/fedora-toolbox:38<br>registry.fedoraproject.org/fedora-toolbox:39<br>registry.fedoraproject.org/fedora-toolbox:rawhide                                                                                                                       |
| openSUSE (Toolbox)    |                                                    | registry.opensuse.org/opensuse/distrobox:latest                                                                                                                                                                                                                                                                         |
| RedHat (Toolbox)      | 8<br>9                                             | registry.access.redhat.com/ubi8/toolbox<br>registry.access.redhat.com/ubi9/toolbox<br>quay.io/toolbx-images/rhel-toolbox:latest                                                                                                                                                                                         |
| Rocky Linux (Toolbox) | 8<br>9                                             | quay.io/toolbx-images/rockylinux-toolbox:8<br>quay.io/toolbx-images/rockylinux-toolbox:9<br>quay.io/toolbx-images/rockylinux-toolbox:latest                                                                                                                                                                             |
| Ubuntu (Toolbox)      | 16.04<br>18.04<br>20.04<br>22.04                   | quay.io/toolbx/ubuntu-toolbox:16.04<br>quay.io/toolbx/ubuntu-toolbox:18.04<br>quay.io/toolbx/ubuntu-toolbox:20.04<br>quay.io/toolbx/ubuntu-toolbox:22.04<br>quay.io/toolbx/ubuntu-toolbox:latest                                                                                                                        |
| AlmaLinux             | 8<br>8-minimal<br>9<br>9-minimal                   | docker.io/library/almalinux:8<br>docker.io/library/almalinux:9                                                                                                                                                                                                                                                          |
| Alpine Linux          | 3.15<br>3.16                                       | docker.io/library/alpine:3.15<br>docker.io/library/alpine:3.16<br>docker.io/library/alpine:latest                                                                                                                                                                                                                       |
| AmazonLinux           | 1<br>2<br>2023                                     | public.ecr.aws/amazonlinux/amazonlinux:1<br>public.ecr.aws/amazonlinux/amazonlinux:2<br>public.ecr.aws/amazonlinux/amazonlinux:2023                                                                                                                                                                                     |
| Archlinux             |                                                    | docker.io/library/archlinux:latest                                                                                                                                                                                                                                                                                      |
| CentOS Stream         | 8<br>9                                             | quay.io/centos/centos:stream8<br>quay.io/centos/centos:stream9                                                                                                                                                                                                                                                          |
| CentOS                | 7                                                  | quay.io/centos/centos:7                                                                                                                                                                                                                                                                                                 |
| Chainguard Wolfi      | Small note: sudo is missing, use su-exec instead.  | cgr.dev/chainguard/wolfi-base:latest                                                                                                                                                                                                                                                                                    |
| ClearLinux            |                                                    | docker.io/library/clearlinux:latest<br>docker.io/library/clearlinux:base                                                                                                                                                                                                                                                |
| Crystal Linux         |                                                    | registry.getcryst.al/crystal/misc/docker:latest                                                                                                                                                                                                                                                                         |
| Debian                | 7<br>8<br>9<br>10<br>11<br>12                      | docker.io/debian/eol:wheezy<br>docker.io/library/debian:buster-backports<br>docker.io/library/debian:bullseye-backports<br>docker.io/library/debian:bookworm-backports<br>docker.io/library/debian:stable-backports                                                                                                     |
| Debian                | Testing                                            | docker.io/library/debian:testing<br>docker.io/library/debian:testing-backports                                                                                                                                                                                                                                          |
| Debian                | Unstable                                           | docker.io/library/debian:unstable                                                                                                                                                                                                                                                                                       |
| deepin                | 20 (apricot)<br>23 (beige)                         | docker.io/linuxdeepin/apricot                                                                                                                                                                                                                                                                                           |
| Fedora                | 36<br>37<br>38<br>39<br>Rawhide                    | quay.io/fedora/fedora:36<br>quay.io/fedora/fedora:37<br>quay.io/fedora/fedora:38<br>quay.io/fedora/fedora:39<br>quay.io/fedora/fedora:rawhide                                                                                                                                                                           |
| Gentoo Linux          | rolling                                            | docker.io/gentoo/stage3:latest                                                                                                                                                                                                                                                                                          |
| KDE neon              | Latest                                             | invent-registry.kde.org/neon/docker-images/plasma:latest                                                                                                                                                                                                                                                                |
| Kali Linux            | rolling                                            | docker.io/kalilinux/kali-rolling:latest                                                                                                                                                                                                                                                                                 |
| Mint                  | 21.1                                               | docker.io/linuxmintd/mint21.1-amd64                                                                                                                                                                                                                                                                                     |
| Neurodebian           | nd100                                              | docker.io/library/neurodebian:nd100                                                                                                                                                                                                                                                                                     |
| openSUSE              | Leap                                               | registry.opensuse.org/opensuse/leap:latest                                                                                                                                                                                                                                                                              |
| openSUSE              | Tumbleweed                                         | registry.opensuse.org/opensuse/distrobox:latest<br>registry.opensuse.org/opensuse/tumbleweed:latest<br>registry.opensuse.org/opensuse/toolbox:latest                                                                                                                                                                    |
| Oracle Linux          | 7<br>7-slim<br>8<br>8-slim<br>9<br>9-slim          | container-registry.oracle.com/os/oraclelinux:7<br>container-registry.oracle.com/os/oraclelinux:7-slim<br>container-registry.oracle.com/os/oraclelinux:8<br>container-registry.oracle.com/os/oraclelinux:8-slim<br>container-registry.oracle.com/os/oraclelinux:9<br>container-registry.oracle.com/os/oraclelinux:9-slim |
| RedHat (UBI)          | 7<br>8<br>9                                        | registry.access.redhat.com/ubi7/ubi<br>registry.access.redhat.com/ubi8/ubi<br>registry.access.redhat.com/ubi8/ubi-init<br>registry.access.redhat.com/ubi8/ubi-minimal<br>registry.access.redhat.com/ubi9/ubi<br>registry.access.redhat.com/ubi9/ubi-init<br>registry.access.redhat.com/ubi9/ubi-minimal                 |
| Rocky Linux           | 8<br>8-minimal<br>9                                | quay.io/rockylinux/rockylinux:8<br>quay.io/rockylinux/rockylinux:8-minimal<br>quay.io/rockylinux/rockylinux:9<br>quay.io/rockylinux/rockylinux:latest                                                                                                                                                                   |
| Scientific Linux      | 7                                                  | docker.io/library/sl:7                                                                                                                                                                                                                                                                                                  |
| SteamOS               |                                                    | ghcr.io/linuxserver/steamos:latest                                                                                                                                                                                                                                                                                      |
| Ubuntu                | 14.04<br>16.04<br>18.04<br>20.04<br>22.04<br>23.04 | docker.io/library/ubuntu:14.04<br>docker.io/library/ubuntu:16.04<br>docker.io/library/ubuntu:18.04<br>docker.io/library/ubuntu:20.04<br>docker.io/library/ubuntu:22.04                                                                                                                                                  |
| Vanilla OS            | VSO                                                | ghcr.io/vanilla-os/vso:main                                                                                                                                                                                                                                                                                             |
| Void Linux            |                                                    | ghcr.io/void-linux/void-glibc-full:latest                                                                                                                                                                                                                                                                               |

### List all boxes

```bash
os --list
```

### Create a new box

```bash
os --new
```

### Enter in a box

```bash
os --use <name>
```

### Stop a box

```bash
os --pause <name>
```

### Stop all boxes

```bash
os --stop
```

### Run a command in a box

```bash
os --run <name> ls
```

### Stop a box

```bash
os --stop <name>
```

### Remove  all boxes

```bash
os --clean
```

### Remove a box

```bash
os --remove <name>
```

### Upgrade all boxes

```bash
os --upgrade
```

### Display help

```bash
os --help
```
