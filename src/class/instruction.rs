#![allow(non_camel_case_types)]
/*
  @see: https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html
*/

type u1 = u8;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub args: Vec<u1>,
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    // Auto generated.
    // Load reference from array
    Aaload = 0x32,
    // Store into reference array
    Aastore = 0x53,
    // Push null
    Aconst_null = 0x1,
    // Load reference from local variable
    Aload = 0x19,
    // Load reference from local variable
    Aload_0 = 0x2a,
    Aload_1 = 0x2b,
    Aload_2 = 0x2c,
    Aload_3 = 0x2d,
    // Create new array of reference
    Anewarray = 0xbd,
    // Return reference from method
    Areturn = 0xb0,
    // Get length of array
    Arraylength = 0xbe,
    // Store reference into local variable
    Astore = 0x3a,
    // Store reference into local variable
    Astore_0 = 0x4b,
    Astore_1 = 0x4c,
    Astore_2 = 0x4d,
    Astore_3 = 0x4e,
    // Throw exception or error
    Athrow = 0xbf,
    // Load byte or boolean from array
    Baload = 0x33,
    // Store into byte or boolean array
    Bastore = 0x54,
    // Push byte
    Bipush = 0x10,
    // Load char from array
    Caload = 0x34,
    // Store into char array
    Castore = 0x55,
    // Check whether object is of given type
    Checkcast = 0xc0,
    // Convert double to float
    D2f = 0x90,
    // Convert double to int
    D2i = 0x8e,
    // Convert double to long
    D2l = 0x8f,
    // Add double
    Dadd = 0x63,
    // Load double from array
    Daload = 0x31,
    // Store into double array
    Dastore = 0x52,
    // Compare double
    Dcmpg = 0x98,
    Dcmpl = 0x97,
    // Push double
    Dconst_0 = 0xe,
    Dconst_1 = 0xf,
    // Divide double
    Ddiv = 0x6f,
    // Load double from local variable
    Dload = 0x18,
    // Load double from local variable
    Dload_0 = 0x26,
    Dload_1 = 0x27,
    Dload_2 = 0x28,
    Dload_3 = 0x29,
    // Multiply double
    Dmul = 0x6b,
    // Negate double
    Dneg = 0x77,
    // Remainder double
    Drem = 0x73,
    // Return double from method
    Dreturn = 0xaf,
    // Store double into local variable
    Dstore = 0x39,
    // Store double into local variable
    Dstore_0 = 0x47,
    Dstore_1 = 0x48,
    Dstore_2 = 0x49,
    Dstore_3 = 0x4a,
    // Subtract double
    Dsub = 0x67,
    // Duplicate the top operand stack value
    Dup = 0x59,
    // Duplicate the top operand stack value and insert two values down
    Dup_x1 = 0x5a,
    // Duplicate the top operand stack value and insert two or three values down
    Dup_x2 = 0x5b,
    // Duplicate the top one or two operand stack values
    Dup2 = 0x5c,
    // Duplicate the top one or two operand stack values and insert two or three values down
    Dup2_x1 = 0x5d,
    // Duplicate the top one or two operand stack values and insert two, three, or four values down
    Dup2_x2 = 0x5e,
    // Convert float to double
    F2d = 0x8d,
    // Convert float to int
    F2i = 0x8b,
    // Convert float to long
    F2l = 0x8c,
    // Add float
    Fadd = 0x62,
    // Load float from array
    Faload = 0x30,
    // Store into float array
    Fastore = 0x51,
    // Compare float
    Fcmpg = 0x96,
    Fcmpl = 0x95,
    // Push float
    Fconst_0 = 0xb,
    Fconst_1 = 0xc,
    Fconst_2 = 0xd,
    // Divide float
    Fdiv = 0x6e,
    // Load float from local variable
    Fload = 0x17,
    // Load float from local variable
    Fload_0 = 0x22,
    Fload_1 = 0x23,
    Fload_2 = 0x24,
    Fload_3 = 0x25,
    // Multiply float
    Fmul = 0x6a,
    // Negate float
    Fneg = 0x76,
    // Remainder float
    Frem = 0x72,
    // Return float from method
    Freturn = 0xae,
    // Store float into local variable
    Fstore = 0x38,
    // Store float into local variable
    Fstore_0 = 0x43,
    Fstore_1 = 0x44,
    Fstore_2 = 0x45,
    Fstore_3 = 0x46,
    // Subtract float
    Fsub = 0x66,
    // Fetch field from object
    Getfield = 0xb4,
    // Get static field from class
    Getstatic = 0xb2,
    // Branch always
    Goto = 0xa7,
    // Branch always (wide index)
    Goto_w = 0xc8,
    // Convert int to byte
    I2b = 0x91,
    // Convert int to char
    I2c = 0x92,
    // Convert int to double
    I2d = 0x87,
    // Convert int to float
    I2f = 0x86,
    // Convert int to long
    I2l = 0x85,
    // Convert int to short
    I2s = 0x93,
    // Add int
    Iadd = 0x60,
    // Load int from array
    Iaload = 0x2e,
    // Boolean AND int
    Iand = 0x7e,
    // Store into int array
    Iastore = 0x4f,
    // Push int constant
    Iconst_m1 = 0x2,
    Iconst_0 = 0x3,
    Iconst_1 = 0x4,
    Iconst_2 = 0x5,
    Iconst_3 = 0x6,
    Iconst_4 = 0x7,
    Iconst_5 = 0x8,
    // Divide int
    Idiv = 0x6c,
    // Branch if reference comparison succeeds
    If_acmpeq = 0xa5,
    If_acmpne = 0xa6,
    // Branch if int comparison succeeds
    If_icmpeq = 0x9f,
    If_icmpne = 0xa0,
    If_icmplt = 0xa1,
    If_icmpge = 0xa2,
    If_icmpgt = 0xa3,
    If_icmple = 0xa4,
    // Branch if int comparison with zero succeeds
    Ifeq = 0x99,
    Ifne = 0x9a,
    Iflt = 0x9b,
    Ifge = 0x9c,
    Ifgt = 0x9d,
    Ifle = 0x9e,
    // Branch if reference not null
    Ifnonnull = 0xc7,
    // Branch if reference is null
    Ifnull = 0xc6,
    // Increment local variable by constant
    Iinc = 0x84,
    // Load int from local variable
    Iload = 0x15,
    // Load int from local variable
    Iload_0 = 0x1a,
    Iload_1 = 0x1b,
    Iload_2 = 0x1c,
    Iload_3 = 0x1d,
    // Multiply int
    Imul = 0x68,
    // Negate int
    Ineg = 0x74,
    // Determine if object is of given type
    Instanceof = 0xc1,
    // Invoke dynamic method
    Invokedynamic = 0xba,
    // Invoke interface method
    Invokeinterface = 0xb9,
    // Invoke instance method; special handling for superclass, private, and instance initialization method invocations
    Invokespecial = 0xb7,
    // Invoke a class (static) method
    Invokestatic = 0xb8,
    // Invoke instance method; dispatch based on class
    Invokevirtual = 0xb6,
    // Boolean OR int
    Ior = 0x80,
    // Remainder int
    Irem = 0x70,
    // Return int from method
    Ireturn = 0xac,
    // Shift left int
    Ishl = 0x78,
    // Arithmetic shift right int
    Ishr = 0x7a,
    // Store int into local variable
    Istore = 0x36,
    // Store int into local variable
    Istore_0 = 0x3b,
    Istore_1 = 0x3c,
    Istore_2 = 0x3d,
    Istore_3 = 0x3e,
    // Subtract int
    Isub = 0x64,
    // Logical shift right int
    Iushr = 0x7c,
    // Boolean XOR int
    Ixor = 0x82,
    // Jump subroutine
    Jsr = 0xa8,
    // Jump subroutine (wide index)
    Jsr_w = 0xc9,
    // Convert long to double
    L2d = 0x8a,
    // Convert long to float
    L2f = 0x89,
    // Convert long to int
    L2i = 0x88,
    // Add long
    Ladd = 0x61,
    // Load long from array
    Laload = 0x2f,
    // Boolean AND long
    Land = 0x7f,
    // Store into long array
    Lastore = 0x50,
    // Compare long
    Lcmp = 0x94,
    // Push long constant
    Lconst_0 = 0x9,
    Lconst_1 = 0xa,
    // Push item from run-time constant pool
    Ldc = 0x12,
    // Push item from run-time constant pool (wide index)
    Ldc_w = 0x13,
    // Push long or double from run-time constant pool (wide index)
    Ldc2_w = 0x14,
    // Divide long
    Ldiv = 0x6d,
    // Load long from local variable
    Lload = 0x16,
    // Load long from local variable
    Lload_0 = 0x1e,
    Lload_1 = 0x1f,
    Lload_2 = 0x20,
    Lload_3 = 0x21,
    // Multiply long
    Lmul = 0x69,
    // Negate long
    Lneg = 0x75,
    // Access jump table by key match and jump
    Lookupswitch = 0xab,
    // Boolean OR long
    Lor = 0x81,
    // Remainder long
    Lrem = 0x71,
    // Return long from method
    Lreturn = 0xad,
    // Shift left long
    Lshl = 0x79,
    // Arithmetic shift right long
    Lshr = 0x7b,
    // Store long into local variable
    Lstore = 0x37,
    // Store long into local variable
    Lstore_0 = 0x3f,
    Lstore_1 = 0x40,
    Lstore_2 = 0x41,
    Lstore_3 = 0x42,
    // Subtract long
    Lsub = 0x65,
    // Logical shift right long
    Lushr = 0x7d,
    // Boolean XOR long
    Lxor = 0x83,
    // Enter monitor for object
    Monitorenter = 0xc2,
    // Exit monitor for object
    Monitorexit = 0xc3,
    // Create new multidimensional array
    Multianewarray = 0xc5,
    // Create new object
    New = 0xbb,
    // Create new array
    Newarray = 0xbc,
    // Do nothing
    Nop = 0x0,
    // Pop the top operand stack value
    Pop = 0x57,
    // Pop the top one or two operand stack values
    Pop2 = 0x58,
    // Set field in object
    Putfield = 0xb5,
    // Set static field in class
    Putstatic = 0xb3,
    // Return from subroutine
    Ret = 0xa9,
    // Return void from method
    Return = 0xb1,
    // Load short from array
    Saload = 0x35,
    // Store into short array
    Sastore = 0x56,
    // Push short
    Sipush = 0x11,
    // Swap the top two operand stack values
    Swap = 0x5f,
    // Access jump table by index and jump
    Tableswitch = 0xaa,
}

impl From<u1> for InstructionKind {
    fn from(n: u1) -> Self {
        use InstructionKind::*;
        match n {
            0x32 => Aaload,
            0x53 => Aastore,
            0x1 => Aconst_null,
            0x19 => Aload,
            0x2a => Aload_0,
            0x2b => Aload_1,
            0x2c => Aload_2,
            0x2d => Aload_3,
            0xbd => Anewarray,
            0xb0 => Areturn,
            0xbe => Arraylength,
            0x3a => Astore,
            0x4b => Astore_0,
            0x4c => Astore_1,
            0x4d => Astore_2,
            0x4e => Astore_3,
            0xbf => Athrow,
            0x33 => Baload,
            0x54 => Bastore,
            0x10 => Bipush,
            0x34 => Caload,
            0x55 => Castore,
            0xc0 => Checkcast,
            0x90 => D2f,
            0x8e => D2i,
            0x8f => D2l,
            0x63 => Dadd,
            0x31 => Daload,
            0x52 => Dastore,
            0x98 => Dcmpg,
            0x97 => Dcmpl,
            0xe => Dconst_0,
            0xf => Dconst_1,
            0x6f => Ddiv,
            0x18 => Dload,
            0x26 => Dload_0,
            0x27 => Dload_1,
            0x28 => Dload_2,
            0x29 => Dload_3,
            0x6b => Dmul,
            0x77 => Dneg,
            0x73 => Drem,
            0xaf => Dreturn,
            0x39 => Dstore,
            0x47 => Dstore_0,
            0x48 => Dstore_1,
            0x49 => Dstore_2,
            0x4a => Dstore_3,
            0x67 => Dsub,
            0x59 => Dup,
            0x5a => Dup_x1,
            0x5b => Dup_x2,
            0x5c => Dup2,
            0x5d => Dup2_x1,
            0x5e => Dup2_x2,
            0x8d => F2d,
            0x8b => F2i,
            0x8c => F2l,
            0x62 => Fadd,
            0x30 => Faload,
            0x51 => Fastore,
            0x96 => Fcmpg,
            0x95 => Fcmpl,
            0xb => Fconst_0,
            0xc => Fconst_1,
            0xd => Fconst_2,
            0x6e => Fdiv,
            0x17 => Fload,
            0x22 => Fload_0,
            0x23 => Fload_1,
            0x24 => Fload_2,
            0x25 => Fload_3,
            0x6a => Fmul,
            0x76 => Fneg,
            0x72 => Frem,
            0xae => Freturn,
            0x38 => Fstore,
            0x43 => Fstore_0,
            0x44 => Fstore_1,
            0x45 => Fstore_2,
            0x46 => Fstore_3,
            0x66 => Fsub,
            0xb4 => Getfield,
            0xb2 => Getstatic,
            0xa7 => Goto,
            0xc8 => Goto_w,
            0x91 => I2b,
            0x92 => I2c,
            0x87 => I2d,
            0x86 => I2f,
            0x85 => I2l,
            0x93 => I2s,
            0x60 => Iadd,
            0x2e => Iaload,
            0x7e => Iand,
            0x4f => Iastore,
            0x2 => Iconst_m1,
            0x3 => Iconst_0,
            0x4 => Iconst_1,
            0x5 => Iconst_2,
            0x6 => Iconst_3,
            0x7 => Iconst_4,
            0x8 => Iconst_5,
            0x6c => Idiv,
            0xa5 => If_acmpeq,
            0xa6 => If_acmpne,
            0x9f => If_icmpeq,
            0xa0 => If_icmpne,
            0xa1 => If_icmplt,
            0xa2 => If_icmpge,
            0xa3 => If_icmpgt,
            0xa4 => If_icmple,
            0x99 => Ifeq,
            0x9a => Ifne,
            0x9b => Iflt,
            0x9c => Ifge,
            0x9d => Ifgt,
            0x9e => Ifle,
            0xc7 => Ifnonnull,
            0xc6 => Ifnull,
            0x84 => Iinc,
            0x15 => Iload,
            0x1a => Iload_0,
            0x1b => Iload_1,
            0x1c => Iload_2,
            0x1d => Iload_3,
            0x68 => Imul,
            0x74 => Ineg,
            0xc1 => Instanceof,
            0xba => Invokedynamic,
            0xb9 => Invokeinterface,
            0xb7 => Invokespecial,
            0xb8 => Invokestatic,
            0xb6 => Invokevirtual,
            0x80 => Ior,
            0x70 => Irem,
            0xac => Ireturn,
            0x78 => Ishl,
            0x7a => Ishr,
            0x36 => Istore,
            0x3b => Istore_0,
            0x3c => Istore_1,
            0x3d => Istore_2,
            0x3e => Istore_3,
            0x64 => Isub,
            0x7c => Iushr,
            0x82 => Ixor,
            0xa8 => Jsr,
            0xc9 => Jsr_w,
            0x8a => L2d,
            0x89 => L2f,
            0x88 => L2i,
            0x61 => Ladd,
            0x2f => Laload,
            0x7f => Land,
            0x50 => Lastore,
            0x94 => Lcmp,
            0x9 => Lconst_0,
            0xa => Lconst_1,
            0x12 => Ldc,
            0x13 => Ldc_w,
            0x14 => Ldc2_w,
            0x6d => Ldiv,
            0x16 => Lload,
            0x1e => Lload_0,
            0x1f => Lload_1,
            0x20 => Lload_2,
            0x21 => Lload_3,
            0x69 => Lmul,
            0x75 => Lneg,
            0xab => Lookupswitch,
            0x81 => Lor,
            0x71 => Lrem,
            0xad => Lreturn,
            0x79 => Lshl,
            0x7b => Lshr,
            0x37 => Lstore,
            0x3f => Lstore_0,
            0x40 => Lstore_1,
            0x41 => Lstore_2,
            0x42 => Lstore_3,
            0x65 => Lsub,
            0x7d => Lushr,
            0x83 => Lxor,
            0xc2 => Monitorenter,
            0xc3 => Monitorexit,
            0xc5 => Multianewarray,
            0xbb => New,
            0xbc => Newarray,
            0x0 => Nop,
            0x57 => Pop,
            0x58 => Pop2,
            0xb5 => Putfield,
            0xb3 => Putstatic,
            0xa9 => Ret,
            0xb1 => Return,
            0x35 => Saload,
            0x56 => Sastore,
            0x11 => Sipush,
            0x5f => Swap,
            0xaa => Tableswitch,
            _ => {
                dbg!(n);
                unreachable!()
            }
        }
    }
}

impl InstructionKind {
    pub fn argc(&self) -> i8 {
        use InstructionKind::*;
        match self {
            Aaload => 0,
            Aastore => 0,
            Aconst_null => 0,
            Aload => 1,
            Aload_0 => 0,
            Aload_1 => 0,
            Aload_2 => 0,
            Aload_3 => 0,
            Anewarray => 2,
            Areturn => 0,
            Arraylength => 0,
            Astore => 1,
            Astore_0 => 0,
            Astore_1 => 0,
            Astore_2 => 0,
            Astore_3 => 0,
            Athrow => 0,
            Baload => 0,
            Bastore => 0,
            Bipush => 1,
            Caload => 0,
            Castore => 0,
            Checkcast => 2,
            D2f => 0,
            D2i => 0,
            D2l => 0,
            Dadd => 0,
            Daload => 0,
            Dastore => 0,
            Dcmpg => 0,
            Dcmpl => 0,
            Dconst_0 => 0,
            Dconst_1 => 0,
            Ddiv => 0,
            Dload => 1,
            Dload_0 => 0,
            Dload_1 => 0,
            Dload_2 => 0,
            Dload_3 => 0,
            Dmul => 0,
            Dneg => 0,
            Drem => 0,
            Dreturn => 0,
            Dstore => 1,
            Dstore_0 => 0,
            Dstore_1 => 0,
            Dstore_2 => 0,
            Dstore_3 => 0,
            Dsub => 0,
            Dup => 0,
            Dup_x1 => 0,
            Dup_x2 => 0,
            Dup2 => 0,
            Dup2_x1 => 0,
            Dup2_x2 => 0,
            F2d => 0,
            F2i => 0,
            F2l => 0,
            Fadd => 0,
            Faload => 0,
            Fastore => 0,
            Fcmpg => 0,
            Fcmpl => 0,
            Fconst_0 => 0,
            Fconst_1 => 0,
            Fconst_2 => 0,
            Fdiv => 0,
            Fload => 1,
            Fload_0 => 0,
            Fload_1 => 0,
            Fload_2 => 0,
            Fload_3 => 0,
            Fmul => 0,
            Fneg => 0,
            Frem => 0,
            Freturn => 0,
            Fstore => 1,
            Fstore_0 => 0,
            Fstore_1 => 0,
            Fstore_2 => 0,
            Fstore_3 => 0,
            Fsub => 0,
            Getfield => 2,
            Getstatic => 2,
            Goto => 2,
            Goto_w => 4,
            I2b => 0,
            I2c => 0,
            I2d => 0,
            I2f => 0,
            I2l => 0,
            I2s => 0,
            Iadd => 0,
            Iaload => 0,
            Iand => 0,
            Iastore => 0,
            Iconst_m1 => 0,
            Iconst_0 => 0,
            Iconst_1 => 0,
            Iconst_2 => 0,
            Iconst_3 => 0,
            Iconst_4 => 0,
            Iconst_5 => 0,
            Idiv => 0,
            If_acmpeq => 2,
            If_acmpne => 2,
            If_icmpeq => 2,
            If_icmpne => 2,
            If_icmplt => 2,
            If_icmpge => 2,
            If_icmpgt => 2,
            If_icmple => 2,
            Ifeq => 2,
            Ifne => 2,
            Iflt => 2,
            Ifge => 2,
            Ifgt => 2,
            Ifle => 2,
            Ifnonnull => 2,
            Ifnull => 2,
            Iinc => 2,
            Iload => 1,
            Iload_0 => 0,
            Iload_1 => 0,
            Iload_2 => 0,
            Iload_3 => 0,
            Imul => 0,
            Ineg => 0,
            Instanceof => 2,
            Invokedynamic => 4,
            Invokeinterface => 4,
            Invokespecial => 2,
            Invokestatic => 2,
            Invokevirtual => 2,
            Ior => 0,
            Irem => 0,
            Ireturn => 0,
            Ishl => 0,
            Ishr => 0,
            Istore => 1,
            Istore_0 => 0,
            Istore_1 => 0,
            Istore_2 => 0,
            Istore_3 => 0,
            Isub => 0,
            Iushr => 0,
            Ixor => 0,
            Jsr => 2,
            Jsr_w => 4,
            L2d => 0,
            L2f => 0,
            L2i => 0,
            Ladd => 0,
            Laload => 0,
            Land => 0,
            Lastore => 0,
            Lcmp => 0,
            Lconst_0 => 0,
            Lconst_1 => 0,
            Ldc => 1,
            Ldc_w => 2,
            Ldc2_w => 2,
            Ldiv => 0,
            Lload => 1,
            Lload_0 => 0,
            Lload_1 => 0,
            Lload_2 => 0,
            Lload_3 => 0,
            Lmul => 0,
            Lneg => 0,
            Lookupswitch => -1,
            Lor => 0,
            Lrem => 0,
            Lreturn => 0,
            Lshl => 0,
            Lshr => 0,
            Lstore => 1,
            Lstore_0 => 0,
            Lstore_1 => 0,
            Lstore_2 => 0,
            Lstore_3 => 0,
            Lsub => 0,
            Lushr => 0,
            Lxor => 0,
            Monitorenter => 0,
            Monitorexit => 0,
            Multianewarray => 3,
            New => 2,
            Newarray => 1,
            Nop => 0,
            Pop => 0,
            Pop2 => 0,
            Putfield => 2,
            Putstatic => 2,
            Ret => 1,
            Return => 0,
            Saload => 0,
            Sastore => 0,
            Sipush => 2,
            Swap => 0,
            Tableswitch => -1,
        }
    }
}
