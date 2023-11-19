# Introduction

I found that remembering melodies is easier than words.

When I was organizing sing-alongs with some friends I thought is was useful to display lyrics on a screen instead of working with printed material. So I wrote applications that assisted in storing and updating lyrics and playlists, selecting a lyric to be displayed. Together they are caled Lipl. It is a sort of abbreviation of **L**yr**i**c **Pl**ay.

Lipl consist of three applications

1. An application for storing the lyrics and playlists, called [lipl-storage](./lipl-storage.md), accessible through a REST interface.
2. An application for displaying part of a lyric on a screen, called [lipl-display](./lipl-display.md).
3. An application for maintaining the collection of lyrics and playlists, and selecting a lyric or playlist that should be displayed on a screen, called [lipl-control](./lipl-control.md).

The first component is written in rust. It runs on a private virtual linux server accessible through the internet. It has a restful interface for creating, reading, updating and deleting of lyrics and playlists.

The second component is also written in rust. The application runs on a Raspberry Pi raspbian installed.
After automatic login of a user, the software starts a gatt peripheral. Bluez has a dbus interface that is used to define the gatt peripheral.

The third component is written in Dart and uses Flutter. It runs on an Android phone. It also provides a web interface used to maintain the lyrics and playlists used.

## Sing-along

The sing-alongs are not at home. When I go to a sing-along I bring with me
- my smartphone with lipl-control installed and data synchronized with lipl-storage
- a raspberry Pi with lipl-display installed

The Raspberry Pi gets connected to a display with the help of its HDMI connector. After booting the Pi advertises a Gatt Service.

Lipl-control can listen for advertisements and connect to the Pi. When I choose Play on lipl-control the parts and status are written to Gatt Characteristics on lipl-display. Lipl control can also write to a characteric that controls font size and theme.