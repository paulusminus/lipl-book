# Lipl Display

The application is using the primary UI thread for the UI. A second thread is started that starts a Gatt peripheral that handles characteristic writes, converts them to messages and passes them to the UI thread.

The gatt peripheral part defines a Gatt service with three characeristics that are writeable, namely text, status and control.

Text is used to get the part of the lyrics that needs to be shown.

Status is used to get the text for the statusbar that is shown on the bottom of the screen, typically the title, the current part and the total parts of het lyric being displayed.

Control is used to select a theme, black letters on a white screen or white letters on a black screen, or to increase or decrease the fontsize.

Future plan is to use a custom yocto distribution instead of raspbian. With a custom yocto distribution I hope to minimize boot time and energy consumption.