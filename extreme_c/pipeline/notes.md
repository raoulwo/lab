# Notes

## Platform

A platform is the combination of an *operating system*
running on specific hardware (architecture), where we
care most about its CPU's *instruction set*.

## Cross-platform vs. Portable

Software that is *cross-platform* can be run on different
*platforms* (see above). The term cross-platform usually
describes software that can be *cross-compiled* to various
platforms, resulting in different binaries (final object
files) whereas *portable* software produces one set of
binaries that can be installed and run on all platforms.

For example, the C compilers `gcc` and `clang` are cross
platform. This means, we can compile final artifacts for
varying platforms. Java bytecode (result of compiling Java
code) is completely portable however.
