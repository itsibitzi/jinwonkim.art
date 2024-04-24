#!/usr/bin/env bash
set -e

CURRENT_TARGET=$(rustc -vV | sed -n 's/host: //p')

if [[ "$CURRENT_TARGET" == "aarch64-apple-darwin" ]]; then
    cargo zigbuild --release --target x86_64-unknown-linux-gnu
    cargo deb --target x86_64-unknown-linux-gnu --no-build

    scp target/x86_64-unknown-linux-gnu/debian/jinwonkim-art_0.1.0-1_amd64.deb root@jinwonkim.art:/root/jinwonkim-art_0.1.0_amd64.deb
    ssh root@jinwonkim.art "dpkg -i jinwonkim-art_0.1.0_amd64.deb"
else
    echo 'not on apple arm :('
fi

