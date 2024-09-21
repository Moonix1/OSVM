%include "./libs/sys_libs.os"

_start:
    ; counter
    mov r16, #17
    
    mov r0, #0
    mov r1, #1
loop:
    add r0, r0, r1
    mov r2, r0
    srg r0, r1
    
    ; Print fibbonacci number using print_num system function
    mov r7, print_num
    sysf r2
    
    ; Decrement counter
    dec r16
    jnz loop, r16
    hlt