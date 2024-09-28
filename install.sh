#!/usr/bin/env bash

REPOSITORY="https://github.com/otechdo/oh.git"
pacman -S rustup --noconfirm
rustup default stable
git clone "${REPOSITORY}" && cd oh || exit 1
make && make install