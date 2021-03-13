# rusty-particle-hub
[![Rust](https://github.com/jzalger/rusty-particle-hub/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/jzalger/rusty-particle-hub/actions/workflows/rust.yml)

Rust implementation of particle hub, a data logging hub for the Particle IoT platform.

## Docker-compose reference
`docker-compose up` from project root directory.

## Docker Command reference
Build the image
`docker build -t rusty-particlehub:latest .`

Run the image:
`docker run -p7070:7070 rusty-particlehub:latest`