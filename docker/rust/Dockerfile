FROM rust:1.83

WORKDIR /search

# Copy the contents of your Rust project to the container
COPY ./search .

# Build the Rust project
#RUN cargo build --release

# Command to run the compiled Rust program
#CMD ["/search/target/debug/search"]
CMD /search/target/release/search > /var/log/searchengine_stdout.log 2> /var/log/searchengine_stderr.log
