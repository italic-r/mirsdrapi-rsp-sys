# mirsdrapi-rsp-sys 2.13

This Rust crate provides a thin wrapper for the official RSP
libraries for controlling SDRplay software-defined radios, namely
RSP1, RSP1a, RSP2, RSP2pro, RSPduo (single-tuner mode).
These bindings are created using `bindgen` and the provided header.

#### Status
This crate is being developed on Linux, and as such will be focusing on
peculiarities of Linux only (for now). Once the Linux build system is
finished, then Windows will come, then OSX if someone is willing to
contribute (I do not own an iOS device).

This wrapper is developed on version 2.13.1 of the RSP driver.
Version 3.01 will be released in the near future, but will be wrapped
in another crate.

#### Dependencies
`bindgen ^0.47.1`  
`pkg-config ^0.3.14`  
`libmirsdrapi-rsp == 2.13`  

Download the RSP driver from [SDRplay Limited](https://www.sdrplay.com/).

To ease linking on Linux, a custom `pkg-config` descriptor is provided at
`pkgconfig/mirsdrapi-rsp.pc`.
This descriptor assumes the library and header are installed in their default location at:  
`/usr/local/include/mirsdrapi-rsp.h`  
`/usr/local/lib/libmirsdrapi-rsp.so.2.13`  

#### Documentation
For full documentation, see the
[official API specification pdf](https://www.sdrplay.com/docs/SDRplay_SDR_API_Specification.pdf).

#### License
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
