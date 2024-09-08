_start:
    mov r0, #500
    mov r1, #80
    sub r2, r0, r1
    
    mov r3, #1
    mov r4, #1
    
loop_test:
    add r3, r3, r4

jump: jmp loop_test