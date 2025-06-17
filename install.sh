#!/bin/bash
set -e

cd `dirname $0`

# cargo build --release
if [[ ! `whoami` = "root" ]];then
	printf "You need admin privileges.\nRunning sudo $0\n"
	sudo $0
	configs/copy.sh
fi
cp target/build/release/ask-ahmed /usr/local/bin/
cp src/ask-ahmed.desktop /usr/share/applications/
cp src/Ahmed.ico /usr/share/pixmaps/
mkdir /usr/share/ask-ahmed
cp src/Ahmed.png /usr/share/ask-ahmed
