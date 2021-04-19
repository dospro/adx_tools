# ADX Tools

A set of tools for working with ADX audio files v3 and v4. 

## Description

adx_tools is a package that contains multiple crates for working with afs and adx files. ADX is a compressed audio format commonly used in videogames consoles like Play Station 2 and Dreamcast, but can also be found on other mediums.

Normally this files get packed in an AFS file which can even be compressed.

Currently there are 2 main crates:

* adx_player: A command line tool which opens and plays ADX files.
* afs_extract: A command line tool which extracts adx files from afs files.
 

## Usage

### adx_player

`adx_player file.adx`

Once playing, you can exit by pressing `q`

### afs_extract

This uses clap library for generating usage information. En example of usage is:

```
afs_extact file.afx --output-dir adx_folder
```

## Future tools

* Probably split adx_player, so the decoding capabilities can be used in other projects.
* Would be nice to also have an encoder, to produce adx files from other audio formats.
* AFS packer.