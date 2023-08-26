#!/usr/bin/env bash

cargo deb

scp target/debian/jinwonkim-art_0.1.0_amd64.deb root@jinwonkim.art:/root/jinwonkim-art_0.1.0_amd64.deb

ssh root@jinwonkim.art "dpkg -i jinwonkim-art_0.1.0_amd64.deb"
