# rmm2
A simple mod manager for The Elder Scrolls and Fallout games on Linux (proton). Allows installing mods and loadorder management. Supports Fomod installers.


## Installing
Download the binary from the [releases](https://github.com/e-k1/rmm2/releases) page, make it executable with `chmod +x rmm2` and copy it to your path.

#### Building from source
You need to have rust installed on your system.

Clone or download the repository. While in the downloaded directory, compile: `cargo build`

Then copy the binary to your path: `cp target/debug/rmm2 <your path>`

## Usage

Start te program by typing rmm2 in the terminal. Choose the game you want to manage and insert the path of the data directory of that game. You must have launched the game at least once in order for the program to work. 

Place the mods you wish to install in the newly created 'mods' directory in your game's root directory and select them in the menu. Mods can be installed from zip archives or directories, though directories are preferred, as the zip feature is unstable and doesn't work most of the time. 

It is recommended to use Kitty terminal, since it allows images to be displayed at full resolution. Other terminals work, but images won't diplay properly.


## Issues

As stated, unpacking archives often fails. If this happens, extract the archive into a directory inside the mods directory and try installing again.

