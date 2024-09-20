_start:
    ; Moves the intial values to the registers
    mov r0, #42
    mov r1, #53
    
    ; Set system function to print_num
    mov r7, #3
    
    ; Pushes the registers values onto the stack
    push r0
    sysf
    push r1
    sysf
    
    mov r16, $0
    sysf r16
    inc r16
    sysf r16
    hlt