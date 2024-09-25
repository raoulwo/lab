; Demonstrate two's complement operation and input of numeric values.

        option casemap:none

nl      =       10      ; ASCII newline
maxLen  =       256

        .data

titleStr byte    'twos_complement', 0

prompt1 byte    "Enter an integer between 0 and 127:", 0
fmtStr1 byte    "Value in hexadecimal: %x", nl, 0
fmtStr2 byte    "Invert all the bits (hexadecimal): %x", nl, 0
fmtStr3 byte    "Add 1 (hexadecimal): %x", nl, 0
fmtStr4 byte    "Output as signed integer: %d", nl, 0
fmtStr5 byte    "Using `neg` instruction: %d", nl, 0

intVal  sqword  ?
input   byte    maxLen dup (?)

        .code

        externdef printf:proc
        externdef atoi:proc
        externdef readLine:proc

        public getTitle
getTitle proc

        lea     rax, titleStr
        ret

getTitle endp

        public asmMain
asmMain proc
        sub     rsp, 56

; Read an unsigned integer from the user: This code will blindly
; assume that the user's input was correct. The `atoi` function returns
; zero if there was some sort of error on the user input.

        lea     rcx, prompt1
        call    printf

        lea     rcx, input
        mov     rdx, maxLen
        call    readLine

; Call C stdlib `atoi` function.

; `i = atoi(str);`

        lea     rcx, input
        call    atoi
        and     rax, 0ffh       ; Only keep LO 8 bits.
        mov     intVal, rax

; Print the input value (in decimal) as a hexadecimal number.

        lea     rcx, fmtStr1
        mov     rdx, rax
        call    printf

; Perform two's complement operation on the input number.
; Begin by inverting all the bits (just work with a byte here).

        lea     rcx, fmtStr2
        mov     rdx, intVal
        not     dl              ; Only work with 8-bit values.
        call    printf

; Invert all the bits and add 1 (still working with just a byte).

        lea     rcx, fmtStr3
        mov     rdx, intVal
        not     rdx
        add     rdx, 1
        and     rdx, 0ffh       ; Only keep LO 8 bits.
        call    printf

; Negate the value and print as a signed integer (work with a full
; integer here, because C++ `%d` format specifier expects a 32-bit
; integer). HO 32 bits of RDX get ignored by C++.

        lea     rcx, fmtStr4
        mov     rdx, intVal
        not     rdx
        add     rdx, 1
        call    printf

; Negate the value using the `neg` instruction.

        lea     rcx, fmtStr5
        mov     rdx, intVal
        neg     rdx
        call    printf

        add     rsp, 56
        ret
asmMain endp
        end
