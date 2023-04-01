
pub enum Segment {
    Constant,
    Local,
    Argument,
    Static,
    This,
    That,
    Temp,
    Pointer, // supplies pointer -> 0 This and pointer -> 1 That
}

pub enum ArithmeticExpr {
    Add,
    Sub,
}

pub enum VirtualMachineExpr {
    Push(Box<Segment>, Box<i16>),
    Pop(Box<Segment>, Box<i16>),
    Arithmetic(Box<ArithmeticExpr>),
    LogicalOperation(Box<LogicalExpr>, Box<i16>),
    Goto(Box<String>),
    ConditionalGoto(Box<LogicalExpr>, Box<String>),
    Call(Box<VirtualMachineExpr>),
    Function(Box<String>),
    Return(Box<i16>),
    Label(Box<i16>, Box<String>),
}

pub enum LogicalExpr {
    Not(i16),
    Negative(),
    Equal(i16),
    And(i16, i16),
    Or(i16, i16),
    LessThan(),
    GreaterThan(),
}

const CMD_ARR: [&str; 5] = [ 
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D+M\n", // ADD
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n", // SUB
    "@SP\nA=M\nM=D\n@SP\nM=M+1\n", // PUSH
    "@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", // POP
    "@LCL\nD=M\n@5\nA=D-A\nD=M\n@R13\nM=D\n@SP\nA=M-1\n 
    D=M\n@ARG\nA=M\nM=D\n
    D=A+1\n@SP\nM=D\n
    @LCL\nA=M-1\nD=M\n@THAT\nM=D\n
    @LCL\nA=M-1\nD=M\n@THIS\nM=D\n
    @LCL\nA=M-1\nD=M\n@ARG\nM=D\n
    @LCL\nA=M-1\nD=M\n@LCL\nM=D\n
    @R13\nA=M\n0; JMP\n", // RET 
];

const LOGICAL_ARR: [&str; 4] = [
    "@SP\nA=M-1\nM=!M\n", // NOT
    "@SP\nA=M-1\nM=-M\n", // NEG
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n", // AND
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n", // OR
];
