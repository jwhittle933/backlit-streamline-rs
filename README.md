# Streamline MPEG Library for Rust

# RFC
## Introduction
The following high-level features are central to the initial usability of this library:
1. The API should have little to no "paperwork". Libraries at this level of complexity tend to require all kinds of setup before you can do what you want. There should a usability derived from elegance and simplicity. Something as simple as `MP4::from_stream(stream).shorten(Duration::from_secs(5)`.
2. Data streams not bulk file content. This will enable operations on larger data files. File I/O is the concern of a program, not a library.
3. Should be able to convert an MPEG-4 to an MPEG-3.
4. Should be able to trim MPEG-3 and MPEG-4.
5. The documentation for the library and every Atom in it should function as a replacement for the ISO/IEC specs. As such, the documentation from the official specs should be included in doc comments for each box, along with additional information gleaned over time. This will remove part of the entry barrier for working with these data structures. 

## Motivation
The motivation for creating this library is to make it easier for new programs to have the functionality of `ffmpeg` without making system calls. Most existing solutions [invoke an installed binary](https://crates.io/keywords/ffmpeg), which requires your VM/machine/docker to include that as a system dependency. Others [create C bindings](https://crates.io/crates/rusty_ffmpeg), which raises concerns around safety.

## Decoding/Encoding MPEG
An MPEG encoded file consists of a series of "boxes" or "atoms" and are encoded linearly (side-by-side) in the stream. Some boxes have no children and only carry data, others have only children and no data, and still others hold their own information along with children. The problem then is decoding and manipulating this stream in the most efficient way possible, into the most usable data structures. 

One possible solution to speed up decoding would be to distribute the reads across threads, reading by each box's offset. The first problem to solve them is the mutable borrow the reader. Encoding could work much the same way, but the same problem of mutable borrow would still exist, but with the writer. One solution may be a reader/writer thread that pulls from a queue and either reads out or writes in. Every other process will only borrow the communication structure.

The ISO/IEC specs outline the encoding in object-oriented terms, each box extending either the `Box` or the `FullBox` class. An initial pass would be to decode into the requisite structures from a stream and encode back from them.

## Unresolved Questions
Research still needs to be done on how to partition MPEG-4 to Apple's [HLS](https://developer.apple.com/streaming/) `.m3u8` format.
