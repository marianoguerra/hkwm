#!/usr/bin/env sh
SERVO_HOME=$HOME/soft/servo/
SERVO_LIBS=$SERVO_HOME/src/platform/linux

rustc -L $SERVO_LIBS/rust-xlib hkwm.rs
