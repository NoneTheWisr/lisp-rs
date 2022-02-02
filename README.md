# lisp-rs
An attempt to create a lisp-like language in Rust

If you want to play with this thing, you can hopefully build it and run as an executable. You can either run it as a REPL (don't provide any arguments) or evaluate the contents of a file (provide a single argument - the path to that file).

You can see the list of supported types in `src/frontend/token.rs` (String, Integer, Boolean) and builtins in `src/eval/builtins.rs` (+, *, =, def).