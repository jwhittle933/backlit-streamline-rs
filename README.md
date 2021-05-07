# Backlit Streamline Rust
Streamline is part of the Backlit platform.

### Streamline for Rust
The streamline library for Rust (and for Go) operate directly on the media bytestream, chunking the stream into the boxes or atoms of the media then parsing the contents.

#### Small Header
```
       | size | |      name        |
binary [0 0 0 4 125 102 116 121 112]
```

#### Large Header
```
       |      size     | |      name        |
binary [0 0 0 0 0 0 0 25 125 102 116 121 112]
```
