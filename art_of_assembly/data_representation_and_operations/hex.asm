; Displays some numeric values on the console.

        option casemap:none

nl      =       10      ; ASCII newline

        .data
i       qword   1
j       qword   123
k       qword   456789

titleStr byte   'hex', 0

fmtStrI byte    "i=%d, converted to hex=%x", nl, 0
fmtStrJ byte    "j=%d, converted to hex=%x", nl, 0
fmtStrK byte    "k=%d, converted to hex=%x", nl, 0

        .code
        externdef       printf:proc

; Return program title to C++ program.

        public getTitle
getTitle proc

; Load address of `titleStr` into the RAX register (RAX holds
; the function return result) and return back to the caller.

        lea rax, titleStr
        ret
getTitle endp

        public asmMain
asmMain proc
        sub     rsp, 56

        ; Call `printf` three times to print `i`, `j`, `k`.

        ; `printf("i=%d, converted to hex=%x\n", i, i);`

        lea rcx, fmtStrI
        mov rdx, i
        mov r8, rdx
        call printf

        ; `printf("j=%d, converted to hex=%x\n", j, j);`

        lea rcx, fmtStrJ
        mov rdx, j
        mov r8, rdx
        call printf

        ; `printf("k=%d, converted to hex=%x\n", k, k);`

        lea rcx, fmtStrK
        mov rdx, k
        mov r8, rdx
        call printf

        add     rsp, 56
        ret
asmMain endp
        end
