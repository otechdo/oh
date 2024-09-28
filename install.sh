#!/usr/bin/env bash

REPOSITORY="https://github.com/otechdo/oh.git"
pacman -Sy rustup git  base base-devel reflector --noconfirm
rustup default stable
git clone "${REPOSITORY}" && cd oh || exit 1
make && make install