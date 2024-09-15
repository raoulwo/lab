# Lazy loading a shared object

Instead of letting the loader load shared
objects *automatically*, we can do so *manually*
before using any of the symbols (functions) part
of the shared object in our code.

For our geometry example, we have to first create
our shared object (here we're statically linking
`libm.a` so that we don't have to manually link
any other dependencies apart from the dynamic
library), but first, we have to create the
relocatable object files:

```sh
gcc -c -fPIC trigonometry.c
gcc -c -fPIC geometry_2d.c
gcc -c -fPIC geometry_3d.c
```

```sh
gcc -shared trigonometry.o geometry_2d.o geometry_3d.o -lm -o libgeometry.so
```

Now we just need to compile the program from which
we want to manually load the shared object:

```sh
gcc main.c -ldl -o geometry
```
