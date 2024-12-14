# Pyszczek

Show your face. Rust library compiled to C for use with other languages.

## Example Ruby usage

```ruby
require 'ffi'

# Connect to the Rust library
module Pyszczek
  extend FFI::Library
  ffi_lib File.join(File.dirname(__FILE__), '../../pyszczek/target/release/libpyszczek.dylib')

  attach_function :nutek, [], :string
  attach_function :free_string, [:pointer], :void
end

puts Pyszczek.nutek
```

## Example Go usage

pyszczek_header.h

```c
#ifndef LIBPYSZCZEK_H
#define LIBPYSZCZEK_H

#ifdef __cplusplus
extern "C" {
#endif

const char* nutek();
void free_string(char* x);

#ifdef __cplusplus
}
#endif

#endif
```

```go
package main

import (
	/*
		#cgo CFLAGS: -I${SRCDIR}/../../../pyszczek/target/release/
		#cgo LDFLAGS: -L${SRCDIR}/../../../pyszczek/target/release/ -lpyszczek
		#include <pyszczek_header.h>
	*/
	"C"
	"fmt"
)

// function nutek that returns a string from C.nutek()
func nutek() string {
	return C.GoString(C.nutek())
}

func main() {
  fmt.Printf("%s\n", nutek())
}
```

## Installation
 To use in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
pyszczek = "0.1.11"
```

```rust
use pyszczek::nutek;
use std::ffi::CStr;
use std::os::raw::c_char;

fn main() {
    unsafe {
        let c_str: *const c_char = nutek();
        let r_str: &CStr = CStr::from_ptr(c_str);
        println!("{}", r_str.to_str().unwrap());
    }
}
```

## External usage

To use in your Ruby or Go project, use above example - top of the file with library
downloaded from [releases](https://github.com/nuteksecurity/pyszczek/releases).
