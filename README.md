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
Use numbers 1-9 to scrub through the song, 10% - 90% through the track.

Press backspace to delete a track. The next track starts immediately. There's no undoing a deletion.

Press escape to exit Crateful.

<img width="2654" height="1646" alt="cute-tui" src="https://github.com/user-attachments/assets/08ce79a9-8802-4983-a721-97dfc616bf56" />

## Installation
Clone this repository. You also need to install Rust & Cargo. Run Cargo build --release to build the released version. In the project folder, the file /target/release/crateful can be moved & ran wherever you like. (just use the command "./crateful" to run it if you're in that directory)

If you're on <b>Linux</b> make this file, "Crateful.desktop" in ~/.local/share/applications. Then you can launch the app with rofi. 
>[Desktop Entry]<br>
Exec=/absolute_path/to/crateful<br>
Type=Application<br>
Terminal=true<br>
Categories=Music<br>
Name=Crateful<br>

## Pairs Great with SoulSeek
In SoulSeek settings, set your "Finished Downloads" folder to the same folder that you're sorting in Crateful, so you can sort music as it finishes downloading. (Set the "incomplete" downloads somewhere else, so you don't accidentally sort a half downloaded track!)

Crateful will automagically look for more downloaded files when the "to sort" list gets shorter than 5 songs.

Cheers to full crates!
