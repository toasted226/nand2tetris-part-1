// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
@SCREEN
D=A
@addr
M=D      // addr = 16384

@i
M=0      // i = 0

(LOOP)
    @KBD
    D=M
    @WHITE
    D;JEQ    // if RAM[KBD] == 0 goto WHITE
    @BLACK
    0;JMP    // goto BLACK

(WHITE)
    @addr
    D=M
    @KBD
    D=D-A
    @LOOP
    D;JEQ   // if addr == KBD goto LOOP

    @i
    D=M
    @addr
    D=D+M
    A=D
    M=0