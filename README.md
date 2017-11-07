# P2P Search with Rust

This code search movie torrent from YTS api. (experimental)

## Installation

This repo requires [rust](https://doc.rust-lang.org/1.12.0/book/getting-started.html) to run.

### Usage

```sh
$ cargo build
$ cargo run "thor"

```
### Result be like :

```
====================================================
Name: "Thor (2011)"
Imdb Code: "tt0800369"
Synopsis: "The warrior Thor (Hemsworth) is cast out of the fantastic realm of Asgard by his father Odin (Hopkins) for his arrogance and sent to Earth to live among humans. Falling in love with scientist Jane Foster (Portman) teaches Thor much-needed lessons, and his new-found strength comes into play as a villain from his homeland sends dark forces toward Earth."
Torrents : 3
Torrent URL: "https://yts.ag/torrent/download/482C6F7DACD3859F9790A32DF31AA43E69C24DEA"
Quality: "3D"
Seeds 22
Peers: 5
Size: "1.70 GB"
Torrent URL: "https://yts.ag/torrent/download/F6AE0FE4CBE20E2A1D66D42E725587E0DECF9138"
Quality: "720p"
Seeds 275
Peers: 69
Size: "700.00 MB"
Torrent URL: "https://yts.ag/torrent/download/ABCE88CCC2D0690078840A20398F32077C89FF0E"
Quality: "1080p"
Seeds 221
Peers: 104
Size: "1.61 GB"
```