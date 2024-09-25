; A "Hello, world!" program using the C/C++ `printf()` function to
; provide the output. The procedure `asmFunc` will be called from
; a C driver file.

        option  casemap:none
        .data

; NOTE: `10` is the line feed character `\n`

fmtStr  byte    "Hello, world!", 10, 0

        .code

; The `externdef` directive is used to define the external symbol
; `printf` of type `proc` (a procedure).

        externdef       printf:proc

        public asmFunc
asmFunc proc

; *Magic* instruction offered without explanation at this point:

        sub     rsp, 56

; Here's where we'll call the C `printf()` function to print
; "Hello, world!":
;
;   1. Pass the address of the format string as the first param
;      of `printf` to the RCX register. This is done using the
;      `lea` (load effective address) instruction.
;   2. Call the procedure using the `call` instruction.

        lea     rcx, fmtStr
        call    printf

; Another *magic* instruction that undoes the effect of the
; previous one before this procedure returns to its caller.

        add     rsp, 56

        ret     ; Returns to caller.
asmFunc endp
        end
