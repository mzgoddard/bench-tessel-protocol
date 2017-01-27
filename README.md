# bench-tessel-protocol

A series of small tessel apps and other binaries written in c, rust, and js to determine ways to improve communication bandwidth between apps and the co-processor.

At this time only rust source is in this repo. They can be run on a tessel with the t2-cli.

1. `npm install -g t2-cli`
2. `rustup toolchain install 1.12.0` (at the moment, tessel's rust toolchain supports 1.11.0 and 1.12.0)
3. `rustup default 1.12.0`
4. `cargo tessel install sdk`
5. `t2 run process_unix_streamed` run one of the rust binaries on the tessel
