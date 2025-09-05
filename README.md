# Rust Speedtest

This repository contains everything you need to set up your very own (very basic) speedtesting infrastructure!
It contains of a simple TCP server, which is capable of running three different tests:

- **ping**: reads packets from the core client and sends identical packets back
- **download**: sends an unlimited stream of data for the core client
- **upload**: reads an unlimited stream of data from the core client

## Components

The project contains three Rust packages:

- **cli**: the Command Line Interface (CLI), which depends on the core
- **core**: client side functionality initiating all tests (this one is a library package)
- **server**: server side functionality

## Set up and run

1. make sure you have installed Rust on your system
2. run `make BUILD=release` in the project's root folder
3. the binaries are now in their respective folders
