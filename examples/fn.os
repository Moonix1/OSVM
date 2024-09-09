_start:
    call calculate
    push #2.3
    hlt
    
calculate:
    push #420
    push #20
    subs
    ret