// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed,
// the screen should be cleared.

(MAIN_LOOP)
    // Check if a key is pressed
    @KBD            // Keyboard register address (24576)
    D=M             // D = value from keyboard register

    // If no key is pressed (D=0), fill screen with white (0)
    // Otherwise, fill screen with black (-1, which is 1111111111111111 in binary)
    @FILL_VALUE
    M=0             // Default, fill with 0 (white)

    @KEY_RELEASED
    D;JEQ           // If D=0, go to KEY_RELEASED

    @FILL_VALUE
    M=-1            // Otherwise, fill with -1 (black)

(KEY_RELEASED)
    // Initialize counter for fill loop
    @8192           // Number of 16-bit words in the screen (512 * 256 / 16)
    D=A
    @counter
    M=D

    // Point to the beginning of the screen
    @SCREEN         // Screen's base address (16384)
    D=A
    @screen_ptr
    M=D

(FILL_LOOP)
    // Check if counter is 0
    @counter
    D=M
    @MAIN_LOOP
    D;JLE           // If counter ≤ 0, return to main loop

    // Fill current pixel
    @FILL_VALUE
    D=M             // D = fill value (0 or -1)

    @screen_ptr
    A=M             // A = memory address pointed by screen_ptr
    M=D             // Put fill value at this address

    // Increment screen pointer
    @screen_ptr
    M=M+1

    // Decrement counter
    @counter
    M=M-1

    // Continue fill loop
    @FILL_LOOP
    0;JMP
