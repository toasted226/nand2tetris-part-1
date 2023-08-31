// for (i = 0; i < n; i++) {
//     Draw 16 pixels to the screen and move down
// }

    @SCREEN
    D=A
    @addr
    M=D      // addr = SCREEN

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
    A=M
    M=-1     // RAM[addr] = -1

    D=A
    @32
    D=D+A   
    A=D      // addr = addr + 32

    @i
    D=M
    D=D+1    // i = i + 1

    @LOOP
    0;JMP

(END)
    @END
    0;JMP