_start: 
    push #0
    push #1
loop:
    dupl 1
    dupl 1
    adds
    jmp loop
    hlt