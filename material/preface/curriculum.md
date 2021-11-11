# Course overview

## Part 1: Overview of the Rust embedded ecosystem + writing a platform-agnostic driver

## Part 2: A case for Rust: how to make Rust work for you in an IoT project
*An idea of how working with Rust feels like in a larger project.*


Case: nrf52840 communicating sensor measurements to a Rust service on a host.

- RTIC
- Sharing code
  - Making the type system work for you
  - Serde with postcard
- Rust tooling
  - Cargo projects
  - Testing
  - Debugging tools
- The not-so-nice parts
 - Crate stability
 - Availability of SDK's
 - Compile duration
 
- Async Rust on embedded