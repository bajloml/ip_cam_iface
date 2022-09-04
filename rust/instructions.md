### instruction to build Rust application:
-   install dependency thirtyfour, run ```cargo install thirtyfour```
-   in the project folder (rust) execute ```cargo new interface --bin``` to create new binary package
-   in the configuration file(Cargo.toml) add info about package like name, version, author...
-   add dependencies at the end of the Cargo.toml:
    ```[dependencies]```
    ```tokio = {version = "1", features = ["full"]}```
    ```thirtyfour = "0.30.0"```
-   run ```sudo sysctl dev.i915.perf_stream_paranoid=0```
-   run ```sudo apt-get install llvm-dev libopencv-dev```
-   run ```sudo apt-get install libclang-dev```
-   run ```sudo apt-get install clang-12 --install-suggests```
-   run ```cargo build```to build application/package
-   run ```cargo run``` to run the application

## note:
-   tensorflow shared libs (.so) are installed manually in ```/lib/x86_64-linux-gnu```

## alternative to cargo packaging is using an compiler:
-   ```rustc src/interface.rs``` --> the output will be in the folder from where the compiler has been called

## before run make sure chromedriver is running