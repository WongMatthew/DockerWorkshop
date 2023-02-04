# Base Image
FROM rust:alpine

EXPOSE 8001

WORKDIR /usr/src/rust-api

# Create a user called rust and the group for it
RUN addgroup -S rustgroup && adduser -S rust -G rustgroup
# Change permissions of the workdir for the new user
RUN chown rust /usr/src/rust-api
# Change to the rust user
USER rust

ADD . .

# Compile the rust source to an executable
RUN rustc server.rs

# Run the executable from above
CMD ["./server"]