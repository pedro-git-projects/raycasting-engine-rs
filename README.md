# Raycasting Engine 

A simple raycasting engine inspired by Wolfenstein 3D, implemented in Rust using SDL2.

## Table of Contents

- [Overview](#overview)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
- [Controls](#controls)

## Overview

This project is a raycasting engine written in Rust, designed to recreate the visual style of Wolfenstein 3D. It utilizes the SDL2 library for graphics rendering.

## Dependencies

- [lazy_static](https://crates.io/crates/lazy_static) - 1.4.0
- [sdl2](https://crates.io/crates/sdl2) - 0.36.0

## Installation

### Prerequisites

Before running the raycasting engine, make sure you have SDL2 installed on your computer. You can download SDL2 from [sdl2's website](https://www.libsdl.org/download-2.0.php) or install it using your system's package manager.

### Building the Project

To run the raycasting engine, follow these steps:

1. **Clone the repository:**

    ```bash
    git clone https://github.com/pedro-git-projects/raycasting-engine-rs.git
    ```

2. **Change into the project directory:**

    ```bash
    cd raycasting-engine-rs 
    ```

3. **Build the project:**

    ```bash
    cargo build --release
    ```

    This command will download and build the required dependencies.

## Usage

Once the project is built, you can run the raycasting engine using:

```bash
cargo run --release
```

This command will launch the application, and you should see the raycasting engine window. 

## Controls

- Move forward: Up arrow 
- Move backward: Down arrow 
- Rotate left: Left arrow 
- Rotate right: Right Arrow 
- Quit the application: Esc

Adjust the controls as needed in the source code.
