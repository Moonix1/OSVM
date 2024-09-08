_start:
    ; Moves the intial values to the registers
    mov r0, #42
    mov r1, #53
    
    ; Pushes the registers values onto the stack
    push r0
    push r1
    
    phsr r16 ; Copies top of the stack to register
    hlt