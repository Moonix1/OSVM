_start: 
    push #0
    push #1
loop:
    dupl 1
    dupl 1
    adds
    
    ; Print fibbonacci number using print_num system function
    mov r7, #3
    sysf
    
    jmp loop
    hlt