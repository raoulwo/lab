# Static Library

## Creating a static library

Here's how to create a static library out of
the sources in this folder:

First, we have to compile the sources:

```sh
gcc -c trigonometry.c
gcc -c geometry_2d.c
gcc -c geometry_3d.c
```

Then we can create the archive:

```sh
ar crs libgeometry.a trigonometry.o geometry_2d.o geometry_3d.o
```

To view the contents of an archive you can do:

```sh
ar t libgeometry.a
```

## Using a static library

When utilizing code of a static library, we're
only concerned with the *public interface* of
the static library. This public API is usually
exposed through a header file.

When done programming, create the object file:

```sh
gcc -c main.c
```

Then we can link `main.o` against our archive
`libgeometry.a` (and `libm.a`).

```sh
gcc main.o -L${PWD} -lgeometry -lm -o geometry
```

The option `-L` specifies a directory which should
be part of the lookup path for the given static
or shared libraries. In this case, it's the current
working directory.

`-lgeometry` specifies that we want to link either
the file `libgeometry.a` or `libgeometry.so`. The
convention used is that when you pass the option
`-lfoo`, the compiler will look for either `libfoo.a`
or `libfoo.so`. The prefix and file-extension is
added implicitly.

`-lm` specifies that we want to link against either
`libm.a` or `libm.so`. This library keeps definitions
of mathematical functions in *glibc*.

NOTE: After the linking process, we don't have any
dependency on the static library anymore as it becomes
part of the executable. This can lead to larger binary
sizes however!
