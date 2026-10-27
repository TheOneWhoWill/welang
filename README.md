# Welang
A simple programming language designed to allow users to write simple script-like code that can be executed as native binaries without the need for an interpreter, virtual machine, or heavy processing such as a garbage collector or virtual environment. Welang is functional in nature and has support for object-oriented programming and type systems. To start Welang will compile down to C code and then use a C compiler to generate the final binary. We will be adding llvm support in the future to allow for more advanced optimizations and features.

Example code:
```welang
// This is a simple example of Welang code that prints "Hello, World!" to the console.
int main() {
	print("Hello, World!")
	// Main functions automatically return 0 if no return statement is provided.
}
```