# keysmash
Configurable pseudorandom string generator
Use `-i (-/string)` to specify input string, out of which chars will be randomly picked. Same characters increase probability. '-' allows for the string to be piped in or inputted manually
`-l (--length) number` - specify output length, '128' if not specified
`-f (--file) path` - override -i with file contents
`-h (--help)` - displays help message
For building, run `cargo install`
