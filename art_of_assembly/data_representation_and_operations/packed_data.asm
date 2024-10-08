; Demonstrate packed data types.

        option casemap:none

NULL    =       0
nl      =       10      ; ASCII newline
maxLen  =       256

; The `.const` section holds data values for read-only constants.

        .const

titleStr        byte    'packed_data', 0
moPrompt        byte    'Enter current month: ', 0
dayPrompt       byte    'Enter current day: ', 0
yearPrompt      byte    'Enter current year '
                byte    '(last 2 digits only): ', 0

packed          byte    'Packed date is %04x', nl, 0
theDate         byte    'The date is %02d/%02d/%02d'
                byte    nl, 0

badDayStr       byte    'Bad day value was entered '
                byte    '(expected 1-31)', nl, 0
badMonthStr     byte    'Bad month value was entered '
                byte    '(expected 1-12)', nl, 0
badYearStr      byte    'Bad year value was entered '
                byte    '(expected 00-99)', nl, 0

        .data

month   byte    ?
day     byte    ?
year    byte    ?
date    word    ?

input   byte    maxLen dup (?)

        .code
        externdef printf:proc
        externdef readLine:proc
        externdef atoi:proc

        public getTitle
getTitle proc
        lea rax, titleStr
        ret
getTitle endp

; Here's a user-written function that reads a numeric value from the
; user:

; `int readNum(char *prompt);`

; A pointer to a string containing a prompt message is passed in the
; RCX register.

; This procedure prints the prompt, reads an input string from the
; user, then converts the input string to an integer and returns the
; integer value in RAX.

readNum proc

; Must align stack before we can call any C/C++ functions.

        sub     rsp, 56

; Print the prompt message. Note that the prompt message was passed to
; this procedure in RCX, we're just passing it on to `printf`.

        call    printf

; Set up args for `readLine` and read a line of text from the user.
; Note that `readLine` returns `NULL` (0) in RAX if there was an error.

        lea     rcx, input
        mov     rdx, maxLen
        call    readLine

; Test for bad input string:

        cmp     rax, NULL
        je      badInput

; Okay, good input at this point, try converting the string to an
; integer by calling `atoi`. The `atoi` function returns zero if there was
; an error, but zero is a perfectly fine return result, so we ignore
; errors.

        lea     rcx, input      ; Pointer to string.
        call    atoi            ; Convert to integer.

badInput:
        add     rsp, 56         ; Undo the stack setup.
        ret
readNum endp

        public asmMain
asmMain proc
        sub     rsp, 56

; Read the date from the user. Begin by reading the month:

        lea     rcx, moPrompt
        call    readNum

; Verify the month is in the range 1..12:

        cmp     rax, 1
        jl      badMonth
        cmp     rax, 12
        jg      badMonth

; Good month, save it for now:

        mov     month, al       ; 1..12 fits in a byte

; Read the day:

        lea     rcx, dayPrompt
        call    readNum

; We'll be lazy here and verify only that the day is in the range
; 1..31.

        cmp     rax, 1
        jl      badDay
        cmp     rax, 31
        jg      badDay

; Good day, save it for now:

        mov     day, al         ; 1..31 fits in a byte

; Read the year:

        lea     rcx, yearPrompt
        call    readNum

; Verify that the year is in the range 0..99.

        cmp     rax, 0
        jl      badYear
        cmp     rax, 99
        jg      badYear

; Good year, save it for now:

        mov     year, al        ; 0..99 fits in a byte

; Pack the data into the following bits:

; 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
;  m  m  m  m  d  d  d  d  d  y  y  y  y  y  y  y

        movzx   ax, month       ; `movzx` zero-extended move
        shl     ax, 5
        or      al, day
        shl     ax, 7
        or      al, year
        mov     date, ax

; Print the packed date:

        lea     rcx, packed
        movzx   rdx, date
        call    printf

; Unpack the date and print it:

        lea     rcx, theDate
        movzx   rdx, date
        mov     r9, rdx
        and     r9, 7fh         ; Keep LO 7 bits (year)
        shr     rdx, 7          ; Get day in position
        mov     r8, rdx
        and     r8, 1fh         ; Keep LO 5 bits (day)
        shr     rdx, 5          ; Get month in position
        call    printf

        jmp     allDone

; Come down here if a bad day was entered:

badDay:
        lea     rcx, badDayStr
        call    printf
        jmp     allDone

; Come down here if a bad month was entered:

badMonth:
        lea     rcx, badMonthStr
        call    printf
        jmp     allDone

; Come down here if a bad year was entered:

badYear:
        lea     rcx, badYearStr
        call    printf
        jmp     allDone

allDone:
        add     rsp, 56
        ret
asmMain endp
        end



