# Process memory structure

When you run an executable file, the operating system creates
a new *process*. A process is a running instance of a program
that is loaded into memory and has a unique *process identifier
(PID)*.

When creating a process, operating systems allocate portions
of memory dedicated to the process and then apply a predefined
*memory layout* (similar between different OSs).

This memory layout is divided into different *segments*, each
segment is a region of memory with a special task that is
supposed to store a specific type of data:

* *Block started by symbol (BSS)* (uninitialized data) segment
* Data segment
* Text/Code segment
* Stack segment
* Heap segment

## Inspecting memory structure

NOTE: Executable program != Running process

Processes are running programs spawned by executing an executable
object file. It consumes main memory and causes the CPU to fetch
and executes its instructions. Unlike the program itself, a process
is a *living* entity. In the memory layout of processes, some
segments stem from the base executable while others are built
dynamically at runtime:

* **Static memory layout** (stems from the executable object file)
* **Dynamic memory layout** (built at process runtime)

The *static memory layout* is prewritten into the executable file,
whereas the *dynamic memory layout* is unique to a running process
since it can differ between different runs of the same executable.

NOTE: OSs always alocate memory for processes in terms of *pages*.
*Pages* (also *memory pages* or *virtual pages*) are the smallest
unit of *fixed-length **virtual** memory*.

## Probing the static memory layout

The `size` command can be used to print the static memory layout of
and executable object file:

```sh
gcc minimal.c -o minimal
```

```sh
size minimal
```

The output shows that the following segments are part of the static
memory layout:

* Text/Code
* Data
* BSS

NOTE: `size` only allows you to view the *size* of segments part of
the static memory layout. To view the contents you can use commands
like `readelf` and `objdump` (in Linux).

You can view the contents of a specific segment using the following
command:

```sh
objdump -s -j .data minimal
```

* `-s` show the full contents of chosen section
* `-j .data` show the data section

NOTE: The output contains five columns, the first represents the
memory address the next four represent 4 bytes of data each (in
hexadecimal). The data output can differ between *little endian*
and *big endian* representation.

NOTE: You can also read the binary content of object files using
tools like `hexdump`.

### Block started by symbol (BSS) segment

The BSS segment holds reserved regions for uninitialized words. This
includes *uninitialized global variables* and *global variables that
are initialized to 0*.

### Data segment

The data segment holds global variables that are initialized to *non-
zero* values.

NOTE: Depending on the platform, either the BSS or data segment will
hold *local static variables*, no matter if initialized or not. This
is why they're called *static* by the way, it's because they're part
of the static memory layout.

### Text/Code segment

The text (code) segment holds all machine-level instructions of an
executable object file. They are part of the static layout and are
fetched from the CPU at runtime when the process is running.

You can view the text segment like this:

```sh
objdump -S minimal
```

* `-S` shows the disassembly intermixed with the source code.

## Probing the dynamic memory layout

The dynamic memory layout is the runtime memory of a process, it
exists as long as the process is running. You can only inspect it
at runtime of a process.

When an executable is executed the *loader* spawns a new process
and creates the initial memory layout for it. To do so, it copies
the segments part of the static memory layout and adds *two
dynamic memory segments* (for a total of 5):

* The stack
* The heap

NOTE: The dynamic memory layout consists of a total of 5 segments,
the most interesting are usually the stack and the heap however.

### Memory Mappings

The memory of a process consists of *memory mappings*. Each mapping
represents a region of memory mapped to a specific file or segment.
Both stack and heap segments have their own memory mappings.

You can view the memory mappings in the `maps` file of a process:

```sh
cat /proc/<PID>/maps
```

The output of the command consists of a number of rows. Each row
represents a memory mapping. Read more about the output format in
the book.

NOTE: `/proc` is the root directory of a special filesystem called
`procfs`.
