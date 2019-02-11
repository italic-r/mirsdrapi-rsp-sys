# mirsdrapi-rsp-sys 2.13

This Rust crate provides a thin wrapper for the official SDRplay
libraries for controlling SDRplay software-defined radios, namely
RSP1, RSP1a, RSP2, RSP2pro, RSPduo.
These bindings are created using `bindgen` and the provided header.

#### Dependencies
`bindgen ^0.47.1`  
`pkg-config ^0.3.14`  
`libmirsdrapi-rsp == 2.13`  

To ease linking, a custom `pkg-config` configuration is provided
at `pkgconfig/mirsdrapi-rsp.pc`. This config assumes the library
and header are installed in their default location at:  
`/usr/local/include/mirsdrapi-rsp.h`  
`/usr/local/lib/libmirsdrapi-rsp.so.2.13`  

#### Documentation
For documentation, see the included header.

#### License
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
