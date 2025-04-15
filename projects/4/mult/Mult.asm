// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
// The algorithm is based on repetitive addition.

// Initialize R2 to 0
@R2
M=0

// Check if R0 or R1 is 0
@R0
D=M
@END
D;JEQ    // If R0=0, goto END

@R1
D=M
@END
D;JEQ    // If R1=0, goto END

// Initialize counter (using R1 as counter)
@R1
D=M
@n
M=D     // n = R1

(LOOP)
    // Add R0 to R2
    @R0
    D=M
    @R2
    M=D+M    // R2 = R2 + R0

    // Decrement counter
    @n
    M=M-1
    D=M

    // If counter > 0, continue loop
    @LOOP
    D;JGT

(END)
    @END
    0;JMP
