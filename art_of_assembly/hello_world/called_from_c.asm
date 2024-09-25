; A simple MASM module that contains an empty function to be
; called by C/C++ code.

        .CODE

; By default symbols in MASM are case-insensitive and all
; identifiers would be transformed to uppercase. This option
; tells MASM to respect the case of identifiers.

; NOTE: MASM identifiers can start contain `$` characters.

        option casemap:none

; Here's the `asmFunc` function. The `public` statement
; declares the `asmFunc()` identifier to be visible outside
; of the MASM source/object file.

; NOTE: The C/C++ linker would be confused if we'd try to
; call a `main` function, that's why we changed the name.

        public asmFunc
asmFunc PROC

; Empty function just returns to C/C++ code.

        ret     ; Returns to caller.

asmFunc ENDP
        END
