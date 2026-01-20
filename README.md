# Crateful - For Music Collectors & DJs

Let's say you've acquired a trove of new music files. Sorting through them, deciding what to save and what to delete, can take hours.

This app streamlines the process.

## When First Opening Crateful

You will be prompted to select the folder of music to sort. Read the instructions to navigate the file explorer (it's easy!). The first track will start playing immediately.

Next, you need to asign where to save your "keepers". Press 'a' and you'll have another file explorer menu. 

While a track is playing, pressing 'a' will now save that track to the folder selected. Next track starts right away (no backsies).

You can also assign folders to the 'd' and 'g' keys. So one folder for metal tracks, one for jazz, one for EDM - however you want.

Pressing spacebar will bring up the pause menu, where you can re-assign all these folders.

## Scrubbing & Deleting
Use numbers 1-9 to seek through the song, 10% - 90% through the track.

The left/right arrow keys (or h & l) will scrub forwards and back by 2 seconds.

Press backspace to delete a track. The next track starts immediately. There's no undoing a deletion!

Press escape to exit Crateful.

<img width="2654" height="1646" alt="cute-tui" src="https://github.com/user-attachments/assets/08ce79a9-8802-4983-a721-97dfc616bf56" />

## Volume
Adjust the volume with the up/down arrow keys (or j for up, k for down).

## Installation
You can <a href="https://ruffolo.pro">download & run the Crateful binary from my website</a>, without having to worry about Rust or Git. 

Using your terminal, navigate to your downloads folder. (use the command "cd Downloads")

Linux (& Mac) users: run the command "chmod +x crateful" (or "chmod +x crateful-mac"). Then "./crateful" ("./crateful-mac") will start the TUI.


### Developer Installation
Clone this repository. You also need to <a href="https://doc.rust-lang.org/cargo/getting-started/installation.html">install Rust & Cargo</a>. Run Cargo build --release to build the released version. In the project folder, the file /target/release/crateful can be moved & ran wherever you like. (just use the command "./crateful" to run it if you're in that directory)

If you're on <b>Linux</b> make this file, "Crateful.desktop" in ~/.local/share/applications. Then you can launch the app with rofi. 
>[Desktop Entry]<br>
Exec=/absolute_path/to/crateful<br>
Type=Application<br>
Terminal=true<br>
Categories=Music<br>
Name=Crateful<br>

## Pairs Great with SoulSeek
In SoulSeek settings, set your "Finished Downloads" folder to the same folder that you're sorting in Crateful, so you can sort music as it finishes downloading. (Set the "incomplete" downloads somewhere else, so you don't accidentally sort a half downloaded track!)

If the number of "tracks to sort" gets lower than 5, Crateful will reload the "to sort" folder, so you can continuously download & sort tracks without reloading Crateful.

Crateful will get all tracks in the folder selected for sorting, even tracks in nested folders, however it does not delete empty folders on exiting.

Cheers to full crates!
