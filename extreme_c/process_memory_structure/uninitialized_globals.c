// This is a minimal C program with uninitialized
// global variables and global variables initialized
// to 0. All those variables are part of the BSS
// segment.

// Since the BSS segment is part of the static memory
// layout, these global variables that aren't initialized
// (or are initialized to 0) are also part of the static
// memory layout and are preallocated when a process is
// loading. They also never get deallocated during the
// lifetime of the process. This means they have *static*
// lifetime.

// WARNING: Because of design reasons, having too many
// global variables is frowned upon. This is because:
//
// * Increased binary size
// * Security concerns
// * Concurrency issues (data races)
// * Namespace pollution
// * Ownership ambiguity
// * ...

int global_var1;
int global_var2;
int global_var3 = 0;
int global_var4 = 0;

int main(int argc, char **argv) { return 0; }
