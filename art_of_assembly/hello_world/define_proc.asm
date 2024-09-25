; A simple demonstration of a user-defined procedure.

        .code

; A sample user-defined procedure that this program can call.

myProc  proc
         ret     ; Immediately return to the caller.
myProc  endp

; Here is the `main` procedure.

main    proc

; Call the user-defined procedure.

        call    myProc

        ret     ; Returns to caller.
main    endp
        end
