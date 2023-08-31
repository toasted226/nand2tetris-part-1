@0   // A = 0
D=A
@R0  
M=D // RAM[0] = 0
@R2
M=D // RAM[2] = 0
@1
D=A
@R1
M=D // RAM[1] = 1
@R0
D=M 
@R1
D=D+M // D = RAM[0] + RAM[1]
@R2
M=D // RAM[2] = RAM[0] + RAM[1]
@R1
D=M
@R0
M=D // RAM[0] = RAM[1]
@R2
D=M // D = RAM[2]
@R1
M=D // RAM[1] = RAM[2]
