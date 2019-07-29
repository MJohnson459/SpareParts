# SpareParts

## Problem
I have a raspberry pi 3. I have a picoborg-rev motor controller and two DC motors attached. This
repo is an exercise in getting this developable with as little headache as possible. There are a few
problems that need to be solved:
1. dev machine is x86, rpi is armv7.
2. dev machine has a different OS than rpi (different libc).

## Plan
Cross compile with Rust `--target` argument.
Copy binary to armv7 docker image
Push to a local docker registry
Pull image from rpi

## Resources
https://kmdouglass.github.io/posts/how-i-built-a-cross-compilation-workflow-for-the-raspberry-pi.html
https://github.com/japaric/rust-cross
https://stackoverflow.com/questions/47645522/cross-compile-multi-arch-containers/49100890
