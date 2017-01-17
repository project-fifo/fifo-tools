docs:
* https://serde.rs/string-or-struct.html
* https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
* https://github.com/phsym/prettytable-rs
* https://github.com/kbknapp/clap-rs / https://clap.rs/


Test commands:
`cargo run -- snapshots create "this is a fancy comment"`
`cargo run -- snapshots lust`


Modules:

```
main.rs.       - main file
cmd            - generic command executor
cli            - CLI commands
 `- snapshots  - snapshot related.subcommands
```