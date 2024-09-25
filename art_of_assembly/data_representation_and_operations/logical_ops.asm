; Demonstrate AND, OR, XOR and NOT logical instructions.

        option casemap:none

nl      =       10      ; ASCII newline

        .data

leftOp   dword  0f0f0f0fh
rightOp1 dword  0f0f0f0f0h
rightOp2 dword  12345678h

titleStr byte   'logical_ops', 0

fmtStr1  byte   "%lx AND %lx = %lx", nl, 0
fmtStr2  byte   "%lx OR %lx = %lx", nl, 0
fmtStr3  byte   "%lx XOR %lx = %lx", nl, 0
fmtStr4  byte   "NOT %lx = %lx", nl, 0

        .code
        externdef       printf:proc

         public getTitle
getTitle proc

         lea rax, titleStr
         ret
getTitle endp

        public asmMain
asmMain proc
        sub     rsp, 56

; Demonstrate the AND instruction.

        lea     rcx, fmtStr1
        mov     edx, leftOp
        mov     r8d, rightOp1
        mov     r9d, edx        ; Compute `leftOp`
        and     r9d, r8d        ; AND `rightOp1`
        call    printf

        lea     rcx, fmtStr1
        mov     edx, leftOp
        mov     r8d, rightOp2
        mov     r9d, edx
        and     r9d, r8d
        call    printf

; Demonstrate the OR instruction.

        lea     rcx, fmtStr2
        mov     edx, leftOp
        mov     r8d, rightOp1
        mov     r9d, edx        ; Compute `leftOp`
        or      r9d, r8d        ; OR `rightOp1`
        call    printf

        lea     rcx, fmtStr2
        mov     edx, leftOp
        mov     r8d, rightOp2
        mov     r9d, edx
        or      r9d, r8d
        call    printf

; Demonstrate the XOR instruction.

        lea     rcx, fmtStr3
        mov     edx, leftOp
        mov     r8d, rightOp1
        mov     r9d, edx        ; Compute `leftOp`
        xor     r9d, r8d        ; XOR `rightOp1`
        call    printf

        lea     rcx, fmtStr3
        mov     edx, leftOp
        mov     r8d, rightOp2
        mov     r9d, edx
        xor     r9d, r8d
        call    printf

; Demonstrate the NOT instruction.

        lea     rcx, fmtStr4
        mov     edx, leftOp
        mov     r8d, edx        ; Compute NOT `leftOp`
        not     r8d
        call    printf

        lea     rcx, fmtStr4
        mov     edx, rightOp1
        mov     r8d, edx
        not     r8d
        call    printf

        lea     rcx, fmtStr4
        mov     edx, rightOp2
        mov     r8d, edx
        not     r8d
        call    printf

        add     rsp, 56
        ret

asmMain endp
        end
