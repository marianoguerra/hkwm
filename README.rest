hkwm
====

this is not a window manager (no, really, it's not)

::

    sudo apt-get install build-essential libx11-dev xserver-xephyr

    Xephyr :33 -ac -host-cursor &
    ./build.sh
    DISPLAY=:33 ./hkwm
    DISPLAY=:33 ./xterm

and watch it crash!
