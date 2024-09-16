# Linker

The linker can combine relocatable object files,
in addition to static libraries, to create a final
executable object file. The question is, how does
it manage to take these separate object files and
produce a working binary?

## Symbols

A relocatable object file contains the equivalent
machine-level instructions for a single translation
unit. These instructions are grouped under sections
which are called **symbols**.

You can use the `nm` utility to inspect all symbols
of a given relocatable object file:

(First we need to create the object file)

```sh
gcc -c average.c
```

```sh
nm average.o
```

You can also use the `readelf` utility to inspect
the *symbol table* existing in an object file:

```sh
readelf -s average.o
```

Using `objdump` you can view the disassembly of
the machine-level instructions:

```sh
objdump -d average.o
```

The disassembly shows that when you need to link
multiple relocatable object files together to create
a final executable, each file only contains a portion
of the whole required function symbols needed.

During the linking process, the linker gathers all
symbols from various relocatable object files before
putting them together in a bigger object file to form
a complete executable binary.

Here's another example demonstrating how the linker
works. First we need to compile relocatable object files:

```sh
gcc -c add.c
gcc -c mult.c
gcc -c main.c
```

When listing the symbols of `main.o` using `nm` we
can see that the symbols `add` and `mult` are marked
with the letter `U`. This means that those 2 symbols
are *unresolved*. We now need to give the linker all
necessary symbols found in the other object files so
that it can resolve any unresolved symbols.

After having found all required symbols, the linker
can then proceed to create an executable object file.
Should it not be able to find the definition of an
unresolved symbol, it will fail with a *linkage error*.
You can often see this if you forget/fail to link
object files with a given static/dynamic library.

The following attempts to create an executable will
fail because of linkage errors:

```sh
gcc main.o add.o # undefined reference to `mult`
gcc main.o mult.o # undefined reference to `add`
gcc add.o mult.o # undefined reference to `main`
```
