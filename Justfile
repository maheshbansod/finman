
run *PARAMS:
    cargo run -- {{PARAMS}}

install:
    cargo build --release
    cp ./target/release/finman ~/opt/bin/

