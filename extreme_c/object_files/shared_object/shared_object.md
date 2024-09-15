# Shared object

## Creating a shared object

First, we need to compile the sources into
relocatable object files. We need to pass a
special option `-fPIC` however:

```sh
gcc -c -fPIC trigonometry.c
gcc -c -fPIC geometry_2d.c
gcc -c -fPIC geometry_3d.c
```

*PIC* stands for *position independent code*
and is **mandatory** when creating a shared
object file.

We then need to invoke the compiler using
the `-shared` option to create a shared object:

```sh
gcc -shared trigonometry.o geometry_2d.o geometry_3d.o -o libgeometry.so
```

## Using a shared object

First, we compile our source that should
be linked against the shared object:

```sh
gcc -c main.c
```

Like with static libraries, we link against the
shared object:

```sh
gcc main.o -L${PWD} -lgeometry -lm -o geometry
```

NOTE: This isn't enough however, we still need to
link against the shared object at runtime since it
didn't become part of the executable at linking time.

In order to link against an executable at runtime
the *loader* (or *dynamic linker*) looks for the
shared object in its load path defined by the
environment variable `LD_LIBRARY_PATH`. We just
need to point this environment variable to the
directory our dynamic library resides in and then
execute our binary:

```sh
LD_LIBRARY_PATH=${PWD} ./geometry
```
