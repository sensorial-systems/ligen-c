# ligen-c
Rust Language Interface Generator for C

### Example

To run the example just follow these steps:

1. Clone this repository: `$ git clone https://github.com/sensorial-systems/ligen-c`
2. Install cargo-ligen: `$ cargo install cargo-ligen`
3. Generate the bindings: `$ cargo ligen`
4. Create the build folder in `examples/adder-c/build`
5. With `examples/adder-c/build` as your current directory, generate the CMake project files: `$ cmake ..`
6. Build `adder-c`: `$ cmake --build .`
7. Execute the program: `$ ./Debug/adder-c`