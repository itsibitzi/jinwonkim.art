#!/usr/bin/env bash

mkdir -p "$HOME/.backup/$(date)"
scp -r root@jinwonkim.art:/opt/jinwonkim.art "$HOME/.backup/$(date)"
