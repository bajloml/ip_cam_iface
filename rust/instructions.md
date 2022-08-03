### instruction to build Rust application:
-   in the project folder (rust) execute ```cargo new interface --bin``` to create new binary package
-   in the configuration file(Cargo.toml) add info about package like name, version, author...
-   run ```cargo build```to build application/package
-   run ```cargo run``` to run the application 

## alternative to cargo packaging is using an compiler:
-   ```rustc src/interface.rs``` --> the output will be in the folder from where the compiler has been called