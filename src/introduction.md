# Introduction

I found that remembering melodies is easier than words.

When I was organizing sing-alongs with some friends I thought is was useful to display lyrics on a screen instead of working with printed material. So I wrote applications that assisted in storing and updating lyrics and playlists, selecting a lyric to be displayed. Together they are caled Lipl. It is a sort of abbreviation of **L**yr**i**c **Pl**ay.

The system has three components

1. A component for storing the lyrics and playlists, called [lipl-repo-rest](./lipl-repo-rest.md).
2. A component for displaying part of a lyric on a screen, called [lipl-display](./lipl-display.md).
3. A component for maintaining the collection of lyrics and playlists, and selecting a lyric or playlist that should be displayed on a screen, called [lipl-control](./lipl-control.md).

The first component was first written in C#. Later on I rewrote it in rust. It runs on a private virtual linux server accessible through the internet. It has a restful interface for creating, reading, updating and deleting of lyrics and playlists.

The second component was first written in python and vala using dbus for communcation between the python process and the vala program. The software runs on a Raspberry Pi Zero W with raspbian installed.
After automatic login of a user, the software starts a gatt peripheral. Bluez has a dbus interface that is used to define the gatt peripheral.
One of the goals is to minimize energy consumption so you can power the raspberry pi through USB.

The third component was first written in kotlin and runs on an Android phone. Later on I rewrote it in flutter.
The application can synchronize data with the first component. Bluetooth is used to send the lyrics that needs to be displayed to the display-server.

## Sing-along

The sing-alongs are not at home. When I go to a sing-along I bring with me
- my smartphone with lipl-control installed and data synchronized with lipl-repo-rest
- a raspberry Pi Zero W with lipl-display installed

The Raspberry Pi Zero gets connected to a display with the help of its HDMI connector. After booting the Pi advertises a Gatt Service.

Lipl-control can listen for advertisements and connect to the Pi. When I choose Play on lipl-control the parts and status are written to Gatt Characteristics on lipl-display. Lipl control can also write to a characteric that controls font size and theme.