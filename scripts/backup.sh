#!/usr/bin/env bash

NOW=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
mkdir -p "$HOME/.backup/$NOW"
scp -r root@jinwonkim.art:/opt/jinwonkim.art "$HOME/.backup/$NOW"
