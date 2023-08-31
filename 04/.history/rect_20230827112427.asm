    @SCREEN
    D=A
    @addr
    A=D      // addr = SCREEN

    @R0
    D=M
    @n
    M=D      // n = RAM[0]

    @i
    D=0
    M=D      // i = 0

(LOOP)
    @i
    D=M
    @n
    D=D-M
    @END
    D;JGT    // if i > n goto END

    @addr
    M=-1     // RAM[addr] = -1
    A=A+32   // addr = addr + 32

    @i
    D=M
    D=D+1    // i = i + 1

    @LOOP
    0;JMP

(END)
    @END
    0;JMP