#!/bin/bash 
docker run --rm -it -v "$(pwd)":/workspace -w /workspace mcr.microsoft.com/devcontainers/rust:1-1-bullseye cargo build