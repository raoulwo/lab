# Pipeline

Building a C/C++ project means that we compile all *sources*
to produce *relocatable object files* (or *intermediate object
files*). These relocatable object files are then *linked* to
produce final artifacts, those can be:

* Executable binary
* Static library (archive)
* Dynamic library (shared object)

When building C/C++ projects there are 2 rules to follow:

1. *Only* compile source files.
2. Compile each source file *separately*.

If a header contains C code that needs to be compiled, we *don't*
compile the header, instead we include it in a source file and
compile that source (this is how you'd use header only libraries).

## 1. Preprocessing

During the preprocessing phase the preprocessor resolves all
*preprocessor directives* (such as includes and macro definitions).
The result of this phase is a single piece of C code that doesn't
contain any preprocessor directives - a *translation unit* (or
*compilation unit*).

(These translation units can become quite big if they, prior to
preprocessing, included lots of headers.)

You can use a compiler to dump a translation unit without compiling
it using the `-E` option:

```sh
gcc -E main.c
gcc -E average.c
```

On many Unix-like systems you can find the `cpp` C preprocessor which
is used under the hood by `gcc` for preprocessing C files.

## 2. Compilation

The compilation step transforms a translation unit into *assembly
code*. This assembly code is specific to the *target architecture*.
The target architecture (or *host architecture*) is the hardware or
CPU the program is compiled for. This *target architecture* can be
different from the *build architecture*, the architecture used to
compile the source.

You can use a compiler to generate the assembly code using the `-S`
option:

```sh
gcc -S main.c
gcc -S average.c
```

## 3. Assembly

The *assembly* step produces the actual machine-level instructions
(machine code) out of assembly code. This is done by the *assembler*
which is specific to each architecture. The resulting files containing
the machine code are called *object files*. In this case, we care
about *relocatable* or *intermediate* object files which are temporary
products used to create the final build artifacts.

Most Unix-like systems have an assembler-tool called `as` which you
can use to create relocatable object files:

```sh
as main.s -o main.o
as average.s -o average.o
```

You can combine the first three steps using a single compiler command:

```sh
gcc -c main.c
gcc -c average.c
```

The assembly step is the last step of compiling a single source file
into relocatable object files. These object files aren't executable
since they only the machine instructions represented by a single
translation unit and not the whole program.

NOTE: Each operating system defines its own specific *object file
format* when it comes to storing machine-level instructions in files.
Object file formats are *platform-dependent*. Linux uses the
*Executable and Linking Format (ELF)*.

## 4. Linking

The last step of the build process is the *linking phase*. In this
phase we combine any number of relocatable (intermediate) object
files into a build artifact.

The default linker in Unix-like systems is `ld`. Using it directly
can be quite complex for beginners, instead we can invoke the
compiler which calls `ld` for us, passing all necessary options so
that it runs:

```sh
gcc main.o average.o
```
