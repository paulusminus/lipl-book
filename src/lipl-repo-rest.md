# Lipl Repo Rest

Program that starts a webserver accepting requests to create, read, update of delete lyrics or playlists.
You can use multiple ways of storing data, e.g. file, postgres, redis and memory. The last one is non persistent and is especially useful in test scenario's.
Handling of authentication, encryption or compression can be done by a reverse proxy server, for example apache.

The handling of the request is using [warp](https://crates.io/crates/warp) or [axum](https://crates.io/crates/axum).

The rest api is documented in a swagger.json file.

[The Source code of Lipl Repo Rest](https://www.github.com/paulusminus/lipl-repo-rest) can be found on Github.