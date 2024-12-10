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

## Installation
 To use in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
pyszczek = "0.1"
```

```rust
use pyszczek::nutek;
let face = nuetk();
println!("{}", face);
```

To use in your Ruby project, use above example - top of the file with library
downloaded from [releases](https://github.com/neosb/pyszczek/releases).
