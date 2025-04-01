# sudo-rs-pq

A post-quantum secure fork of [sudo-rs](https://github.com/trifectatechfoundation/sudo-rs).

This hardens sudo-rs by replacing all usages of classical cryptographic algorithms with post-quantum secure ones.

This is EXPERIMENTAL ⚠️‼️. Therefore, to use this fork of sudo-rs you have to enable the environment variable

`SUDO_RS_IS_PQ` and set it to the value `I accept that quantum computers may break my system unexpectedly`.
