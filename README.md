# Backlit Streamline Rust
Streamline is part of the Backlit platform.

### Streamline for Rust
The streamline library for Rust operates directly on the media bytestream, chunking the stream into the boxes or atoms of the media then parsing the contents.

### RFC (How to do the thing)
The boxes in mp4 format (and other media playback types) are encoded linearly (side-by-side) in the stream. Box hierarchies are not dynamically deducible from the stream itself, but must be known from the ISO/IEC spec. Some boxes have no children, others have only children, and still other hold their own information along with children. The problem then is decoding this stream in the most efficient way possible, into the most usable data structures. The spec outlines the encoding in very object-oriented terms, each box extending either the `Box` or the `FullBox` class. Accomplishing this with the Rust `trait` is complicated at best. 

An alternative to creating a `struct` for each box type would be a single structure that tracks and records the offset and size of each box, without parsing its content (currently the `Info` struct). This allows for indexed reads of the binary without having to handle lifetimes and dropping. Of course parsing the content into human-readable 


Research still needs to be done on how to partition mp4 to HLS `.m3u8` format.
