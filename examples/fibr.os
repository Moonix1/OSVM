_start:
    ; counter
    mov r16, #17
    
    mov r0, #0
    mov r1, #1
loop:
    add r0, r0, r1
    srg r0, r1
    
    ; Decrement counter
    dec r16
    jnz loop, r16
    hlt