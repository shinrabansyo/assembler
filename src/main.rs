use assembler::{assemble, assembly, Inst};
use compiler::compile;

#[rustfmt::skip]
fn main() {
    // let program = "
    //     hello, shirabansyo!
    // ";
    // let result = assemble(compile(program));
    // println!("{}", result);

    let result = assemble(vec![
        /*
            | 0x00000001    | SPI (Data)
            | 0x00000002    | SPI (Mode)
            | 0x00000003    | SPI (Clockshamt)
        */

        /*
           # mode
            out r0[2] = r0
           # clkshamt 
            addi r1 = r0, 4
            out r0[3] = r1
           
           # A: 97, Z: 122
            addi r2 = r0, 97
            addi r3 = r0, 123
            out r0[1] = r2
            addi r2 = r2, 1
            beq r0, (r2, r3) -> -24
            beq r0, (r0, r0) -> -18

        */
        assembly!(addi r1 = r0, 0),
        assembly!(out r0[2] = r1),

        assembly!(addi r1 = r0, 4),
        assembly!(out r0[3] = r1),

        assembly!(addi r2 = r0, 97),
        assembly!(addi r3 = r0, 123),
        assembly!(out r0[1] = r2),
        assembly!(addi r2 = r2, 1),
        assembly!(beq r0, (r2, r3) -> -24),
        assembly!(beq r0, (r0, r0) -> -18),

        // assembly!(addi r1 = r0, 1),                // 00 (00)
        // assembly!(bne r0, (r0, r1) -> 0),          // 06 (06)
        // assembly!(addi r1 = r1, 1),                // 12 (0C)

        // assembly!(addi r2 = r0, 97),                // 00 (00)
        // assembly!(out r0[0] = r2),                  // 06 (06)

        // `out(x[rs1] + imm, rs2)`
        // assembly!(out rs1[imm] = rs2),
        // `rd = in(x[rs1] + imm)`
        // assembly!(in rd = rs1[imm]),

        // assembly!(lw r6 = r0[0]),               // 00 (00)
        // assembly!(lw r7 = r0[4]),               // 06 (06)

        // assembly!(sw r0[0] = r7),               // 12 (0C)
        // assembly!(sw r0[4] = r6),               // 18 (12)

        // assembly!(addi r5 = r5, 1),             // 24 (18)
        // assembly!(addi r0 = r0, 0),             // 30 (1E)
        // assembly!(addi r1 = r1, 1),             // 36 (24)

        // assembly!(add  r2 = r0, r1),            // 42 (2A)
        // assembly!(addi r0 = r1, 0),             // 48 (30)
        // assembly!(addi r1 = r2, 0),             // 54 (36)
        // assembly!(addi r5 = r5, 1),             // 60 (3C)
        // assembly!(beq  r4, (r3, r3) -> -24),    // 66 (42)
        //                                         // 72 (48)
    ]);
    println!("{}", result);
}
