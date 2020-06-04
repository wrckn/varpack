#! /bin/sh

x86_64-w64-mingw32-ldd target/x86_64-pc-windows-gnu/debug/varpack-gtk.exe | grep 'lib*' | cut -d "=" -f 1 | awk '{$1=$1;print}' | xargs -I{} cp /usr/x86_64-w64-mingw32/bin/{} target/x86_64-pc-windows-gnu/debug/
