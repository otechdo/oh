#!/usr/bin/env bash

REPOSITORY="https://github.com/otechdo/oh.git"
pacman -Sy rustup git base base-devel reflector vim --noconfirm
rustup default stable
git clone "${REPOSITORY}" && cd oh || exit 1
make && make install
pacman -Sl | cut -d ' ' -f 2 > /tmp/packages
pacman -Sg >> /tmp/packages