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
    D=A
    D=D+32
    A=D      // addr = addr + 32

    @i
    D=M

(END)
    @END
    0;JMP