# MPC

MultiPlayer CPU. Big endian.

## Overview

MPC is a virtual machine implementation that supports various opcodes for computation, memory operations, and stack management. The VM operates in a client-server architecture where clients can connect to execute instructions on the virtual machine.

## Building and Running

### Build the project
```bash
cargo build --release
```

### Start the server
```bash
cargo run -- server --ip 127.0.0.1 --port 3000
```

### Connect a client
```bash
cargo run -- client --ip 127.0.0.1 --port 3000
```

## Instruction Set

All instructions are 2 bytes (16 bits) encoded as nibbles. The format is:
`[OPCODE][ARG1][ARG2][ARG3]` where each part is a 4-bit nibble.

### 0x0 - HALT
**Description**: Stops the virtual machine execution
**Format**: `0x0000`
**Example**: `0x0000`
**Effect**: Sets the halted flag to true, stopping further instruction execution

### 0x1 - LOAD_IMMEDIATE
**Description**: Load an immediate 8-bit value into a register
**Format**: `0x1[REG][VAL_HIGH][VAL_LOW]`
**Parameters**:
- REG: Target register (0-F)
- VAL_HIGH: High nibble of the 8-bit value
- VAL_LOW: Low nibble of the 8-bit value
**Example**: `0x12AB` loads 0xAB into register 2
**Effect**: `REG[REG] = (VAL_HIGH << 4) | VAL_LOW`

### 0x3 - DUMP
**Description**: Store a register value to memory
**Format**: `0x3[REG][ADDR_HIGH][ADDR_LOW]`
**Parameters**:
- REG: Source register (0-F)
- ADDR_HIGH: High nibble of memory address
- ADDR_LOW: Low nibble of memory address
**Example**: `0x31AB` stores register 1 to memory address 0xAB
**Effect**: `MEM[ADDR] = REG[REG]` (16-bit value stored as 2 bytes)

### 0x4 - ADD
**Description**: Add two register values and store result in accumulator
**Format**: `0x4[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: First operand register (0-F)
- REG2: Second operand register (0-F)
- UNUSED: Ignored
**Example**: `0x4120` adds register 1 and register 2
**Effect**: `ACCUM = REG[REG1] + REG[REG2]`

### 0x5 - SUB
**Description**: Subtract second register from first register, store result in accumulator
**Format**: `0x5[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: Minuend register (0-F)
- REG2: Subtrahend register (0-F)
- UNUSED: Ignored
**Example**: `0x5120` subtracts register 2 from register 1
**Effect**: `ACCUM = REG[REG1] - REG[REG2]`

### 0x6 - MULT
**Description**: Multiply two register values and store result in accumulator
**Format**: `0x6[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: First operand register (0-F)
- REG2: Second operand register (0-F)
- UNUSED: Ignored
**Example**: `0x6120` multiplies register 1 and register 2
**Effect**: `ACCUM = REG[REG1] * REG[REG2]`

### 0x7 - DIV
**Description**: Divide first register by second register, store result in accumulator
**Format**: `0x7[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: Dividend register (0-F)
- REG2: Divisor register (0-F)
- UNUSED: Ignored
**Example**: `0x7120` divides register 1 by register 2
**Effect**: `ACCUM = REG[REG1] / REG[REG2]`
**Note**: Division by zero will cause a panic

### 0x8 - AND
**Description**: Perform bitwise AND on two register values, store result in accumulator
**Format**: `0x8[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: First operand register (0-F)
- REG2: Second operand register (0-F)
- UNUSED: Ignored
**Example**: `0x8120` performs AND on register 1 and register 2
**Effect**: `ACCUM = REG[REG1] & REG[REG2]`

### 0x9 - OR
**Description**: Perform bitwise OR on two register values, store result in accumulator
**Format**: `0x9[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: First operand register (0-F)
- REG2: Second operand register (0-F)
- UNUSED: Ignored
**Example**: `0x9120` performs OR on register 1 and register 2
**Effect**: `ACCUM = REG[REG1] | REG[REG2]`

### 0xA - NOT
**Description**: Perform bitwise NOT on a register value, store result in accumulator
**Format**: `0xA[REG][UNUSED][UNUSED]`
**Parameters**:
- REG: Source register (0-F)
- UNUSED: Ignored
**Example**: `0xA100` performs NOT on register 1
**Effect**: `ACCUM = !REG[REG]`

### 0xB - XOR
**Description**: Perform bitwise XOR on two register values, store result in accumulator
**Format**: `0xB[REG1][REG2][UNUSED]`
**Parameters**:
- REG1: First operand register (0-F)
- REG2: Second operand register (0-F)
- UNUSED: Ignored
**Example**: `0xB120` performs XOR on register 1 and register 2
**Effect**: `ACCUM = REG[REG1] ^ REG[REG2]`

### 0xC - LSHIFT
**Description**: Left shift a register value by specified bits, store result in accumulator
**Format**: `0xC[REG][BITS][UNUSED]`
**Parameters**:
- REG: Source register (0-F)
- BITS: Number of bits to shift (0-F)
- UNUSED: Ignored
**Example**: `0xC130` left shifts register 1 by 3 bits
**Effect**: `ACCUM = REG[REG] << BITS`

### 0xD - RSHIFT
**Description**: Right shift a register value by specified bits, store result in accumulator
**Format**: `0xD[REG][BITS][UNUSED]`
**Parameters**:
- REG: Source register (0-F)
- BITS: Number of bits to shift (0-F)
- UNUSED: Ignored
**Example**: `0xD130` right shifts register 1 by 3 bits
**Effect**: `ACCUM = REG[REG] >> BITS`

### 0xE - PUSH
**Description**: Push a register value onto the stack
**Format**: `0xE[REG][UNUSED][UNUSED]`
**Parameters**:
- REG: Source register (0-F)
- UNUSED: Ignored
**Example**: `0xE100` pushes register 1 onto the stack
**Effect**: Stack pointer decreases by 2, register value stored at new stack position

### 0xF - POP
**Description**: Pop a value from the stack into a register
**Format**: `0xF[REG][UNUSED][UNUSED]`
**Parameters**:
- REG: Target register (0-F)
- UNUSED: Ignored
**Example**: `0xF100` pops stack value into register 1
**Effect**: Value loaded from stack into register, stack pointer increases by 2

### 0x10 - DUMP_ACCUM
**Description**: Store accumulator value to memory
**Format**: `0x10[UNUSED][ADDR_HIGH][ADDR_LOW]`
**Parameters**:
- UNUSED: Ignored
- ADDR_HIGH: High nibble of memory address
- ADDR_LOW: Low nibble of memory address
**Example**: `0x10AB` stores accumulator to memory address 0xAB
**Effect**: `MEM[ADDR] = ACCUM` (16-bit value stored as 2 bytes)

### 0x11 - PUSH_ACCUM
**Description**: Push accumulator value onto the stack
**Format**: `0x1100`
**Example**: `0x1100` pushes accumulator onto the stack
**Effect**: Stack pointer decreases by 2, accumulator value stored at new stack position