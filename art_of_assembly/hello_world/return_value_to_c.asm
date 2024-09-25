; An assembly language program that demonstrates returning
; a function result to a C++ program.

; NOTE: Whenever interfacing with external assembly level
; code, be it whether you're calling into their procedures
; or they're calling into yours, both parties need to
; agree upon which registers to use for passing arguments
; to functions and which registers to use for returning
; values from functions. This sort of *contract* is called
; ABI (Application Binary Interface). In this book, we'll
; interface with the Microsoft ABI.

; The Microsoft ABI specifies that the first four params
; to any C/C++ function must be passed to the registers
; RCX, RDX, R8, R9.

; It also specifies that integer or pointer return values
; should be passed into the RAX register.

        option  casemap:none

nl      =       10      ; ASCII code for newline
maxlen  =       256     ; Maximum string size + 1

         .data
titleStr byte   'return_value_to_c', 0
prompt   byte   'Enter a string: ', 0
fmtStr   byte   "User entered: '%s'", nl, 0

; `input` is a buffer having `maxlen` bytes. This program
; will read a user string into this buffer.

; The `maxlen dup (?)` operand tells MASM to make `maxlen`
; duplicate copies of a byte, each of which is uninitialized.

input    byte   maxlen dup (?)

        .code

        externdef       printf:proc
        externdef       readLine:proc

; The C++ function calling this assembly language module
; expects a function called `getTitle` that returns a pointer
; to a string as the function result. This is that function:

         public getTitle
getTitle proc

; Load address of `titleStr` into the RAX register (RAX holds
; the function return result) and return back to the caller:
        lea rax, titleStr
        ret
getTitle endp

; Here is the `asmMain` function.

        public asmMain
asmMain proc

; Done for adherence to the Microsoft ABI in regards to stack
; alignment. Will be explained in later chapters.
        sub rsp, 56

; Call the `readLine` function (written in C++) to read a line
; of text from the console.

; int readLine(char *dest, int maxlen);

; Pass a pointer to the destination buffer in the RCX register.
; Pass the maximum buffer size (max chars + 1) in RDX.
; This function ignores the `readLine` return result.
; Prompt the user to enter a string:

        lea     rcx, prompt
        call    printf

; Ensure the input string is null-terminated (in the event
; there is an error):

        mov     input, 0

; Read a line of text from the user:

        lea     rcx, input
        mov     rdx, maxlen
        call    readLine

; Print the string input by the user by calling `printf()`:

        lea     rcx, fmtStr
        lea     rdx, input
        call    printf

        add     rsp, 56
        ret     ; Returns to caller.

asmMain endp
        end


