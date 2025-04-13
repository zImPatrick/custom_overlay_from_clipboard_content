# Custom Overlay from Clipboard Content
The key to toggle the overlay by default is set to `F`. Pressing `G` will output the value via TTS.

To compile this on Debian install `libinput-dev libxtst-dev libx11-dev libxtst-dev libudev-dev`.
If you want to use the tts output on Debian install `libspeechd-dev`.

## Linux quirks
If you're on Linux, you might have to run the program as root using sudo, as a regular user can't capture global key input.  
However, by default, using `sudo` runs the program using Xwayland, which breaks maximizing the window on Wayland, so you'll have to use `sudo -E` to preserve your enviroment variables to restore that functionality.
