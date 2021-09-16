# ADX Tools

A set of tools for working with ADX audio files v3 and v4. 

## Description

adx_tools is a package that contains multiple crates for working with afs and adx files. ADX is a compressed audio format commonly used in videogames consoles like Play Station 2 and Dreamcast, but can also be found on other mediums.

Normally this files get packed into an AFS file which may be compressed.

Currently, there are 3 main crates:

* adx_player: A command line tool which opens and plays ADX files.
* afs_extract: A command line tool which extracts adx files from afs files.
* adx_decoder: A library for decoding adx audio files.
 

## Usage

### adx_player

`adx_player file.adx`

Once playing, you can exit by pressing `q`

### afs_extract

afs_extract uses clap library for generating usage information. An example of usage is:

```
afs_extact file.afx --output-dir adx_folder
```

### adx_decoder
You just need to `adx_tools::adx_file::AdxFile` into your project.

Look at `adx_player/main.rs` for an example on using the decoder.

## Future tools

* Would be nice to also have an encoder to produce adx files from other audio formats.
* Playing audio files directly from afs files.
* AFS packer.