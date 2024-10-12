from rust

workdir /build

copy src Cargo.toml .

run apt update && cargo build --release --bin cripto 
workdir /app
run mv /build/target/release/cripto .
run rm -r /build
cmd sh ./cripto
