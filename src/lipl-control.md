# Lipl Control

First written in Kotlin. Now written in Flutter (Dart). The reason for a rewrite is that Flutter is targetting more platforms, i.e. IOS.

If the device the application is running on has an Internet Connection, the application can synchronize its data with Lipl Repo Rest. Synchronization is important so the application can also be used when there is no Internet Connection.

On Android or IOS the application can establish a connection with Lipl Display. Lipl Display uses advertising so the device can found.
If the connection is established, you can use the play button to display parts of a lyric or playlist.

Flutter can also build a web app. This can be handy for people maintaining the data. Bluetooth connection are not possible though.

[Source code for Lipl Control](https://www.github.com/paulusminus/lipl-control-flutter) can be found on Github.