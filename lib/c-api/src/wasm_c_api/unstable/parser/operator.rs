use wasmer::wasmparser::Operator;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum wasmer_parser_operator_t {
    Unreachable,
    Nop,
    Block,
    Loop,
    If,
    Else,
    Try,
    Catch,
    Throw,
    Rethrow,
    Unwind,
    End,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,
    ReturnCall,
    ReturnCallIndirect,
    Drop,
    Select,
    TypedSelect,
    LocalGet,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    I32Load,
    I64Load,
    F32Load,
    F64Load,
    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,
    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
    I32Store,
    I64Store,
    F32Store,
    F64Store,
    I32Store8,
    I32Store16,
    I64Store8,
    I64Store16,
    I64Store32,
    MemorySize,
    MemoryGrow,
    I32Const,
    I64Const,
    F32Const,
    F64Const,
    RefNull,
    RefIsNull,
    RefFunc,
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    MemoryInit,
    DataDrop,
    MemoryCopy,
    MemoryFill,
    TableInit,
    ElemDrop,
    TableCopy,
    TableFill,
    TableGet,
    TableSet,
    TableGrow,
    TableSize,
    MemoryAtomicNotify,
    MemoryAtomicWait32,
    MemoryAtomicWait64,
    AtomicFence,
    I32AtomicLoad,
    I64AtomicLoad,
    I32AtomicLoad8U,
    I32AtomicLoad16U,
    I64AtomicLoad8U,
    I64AtomicLoad16U,
    I64AtomicLoad32U,
    I32AtomicStore,
    I64AtomicStore,
    I32AtomicStore8,
    I32AtomicStore16,
    I64AtomicStore8,
    I64AtomicStore16,
    I64AtomicStore32,
    I32AtomicRmwAdd,
    I64AtomicRmwAdd,
    I32AtomicRmw8AddU,
    I32AtomicRmw16AddU,
    I64AtomicRmw8AddU,
    I64AtomicRmw16AddU,
    I64AtomicRmw32AddU,
    I32AtomicRmwSub,
    I64AtomicRmwSub,
    I32AtomicRmw8SubU,
    I32AtomicRmw16SubU,
    I64AtomicRmw8SubU,
    I64AtomicRmw16SubU,
    I64AtomicRmw32SubU,
    I32AtomicRmwAnd,
    I64AtomicRmwAnd,
    I32AtomicRmw8AndU,
    I32AtomicRmw16AndU,
    I64AtomicRmw8AndU,
    I64AtomicRmw16AndU,
    I64AtomicRmw32AndU,
    I32AtomicRmwOr,
    I64AtomicRmwOr,
    I32AtomicRmw8OrU,
    I32AtomicRmw16OrU,
    I64AtomicRmw8OrU,
    I64AtomicRmw16OrU,
    I64AtomicRmw32OrU,
    I32AtomicRmwXor,
    I64AtomicRmwXor,
    I32AtomicRmw8XorU,
    I32AtomicRmw16XorU,
    I64AtomicRmw8XorU,
    I64AtomicRmw16XorU,
    I64AtomicRmw32XorU,
    I32AtomicRmwXchg,
    I64AtomicRmwXchg,
    I32AtomicRmw8XchgU,
    I32AtomicRmw16XchgU,
    I64AtomicRmw8XchgU,
    I64AtomicRmw16XchgU,
    I64AtomicRmw32XchgU,
    I32AtomicRmwCmpxchg,
    I64AtomicRmwCmpxchg,
    I32AtomicRmw8CmpxchgU,
    I32AtomicRmw16CmpxchgU,
    I64AtomicRmw8CmpxchgU,
    I64AtomicRmw16CmpxchgU,
    I64AtomicRmw32CmpxchgU,
    V128Load,
    V128Store,
    V128Const,
    I8x16Splat,
    I8x16ExtractLaneS,
    I8x16ExtractLaneU,
    I8x16ReplaceLane,
    I16x8Splat,
    I16x8ExtractLaneS,
    I16x8ExtractLaneU,
    I16x8ReplaceLane,
    I32x4Splat,
    I32x4ExtractLane,
    I32x4ReplaceLane,
    I64x2Splat,
    I64x2ExtractLane,
    I64x2ReplaceLane,
    F32x4Splat,
    F32x4ExtractLane,
    F32x4ReplaceLane,
    F64x2Splat,
    F64x2ExtractLane,
    F64x2ReplaceLane,
    I8x16Eq,
    I8x16Ne,
    I8x16LtS,
    I8x16LtU,
    I8x16GtS,
    I8x16GtU,
    I8x16LeS,
    I8x16LeU,
    I8x16GeS,
    I8x16GeU,
    I16x8Eq,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,
    I32x4Eq,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,
    I64x2Eq,
    I64x2Ne,
    F32x4Eq,
    F32x4Ne,
    F32x4Lt,
    F32x4Gt,
    F32x4Le,
    F32x4Ge,
    F64x2Eq,
    F64x2Ne,
    F64x2Lt,
    F64x2Gt,
    F64x2Le,
    F64x2Ge,
    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,
    I8x16Abs,
    I8x16Neg,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I16x8Abs,
    I16x8Neg,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16x8Mul,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I32x4Abs,
    I32x4Neg,
    I32x4AllTrue,
    I32x4Bitmask,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Sub,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4DotI16x8S,
    I64x2Neg,
    I64x2AllTrue,
    I64x2Bitmask,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub,
    I64x2Mul,
    F32x4Ceil,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F64x2Ceil,
    F64x2Floor,
    F64x2Trunc,
    F64x2Nearest,
    F32x4Abs,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4PMin,
    F32x4PMax,
    F64x2Abs,
    F64x2Neg,
    F64x2Sqrt,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2PMin,
    F64x2PMax,
    I32x4TruncSatF32x4S,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I8x16Swizzle,
    I8x16Shuffle,
    V128Load8Splat,
    V128Load16Splat,
    V128Load32Splat,
    V128Load32Zero,
    V128Load64Splat,
    V128Load64Zero,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8WidenLowI8x16S,
    I16x8WidenHighI8x16S,
    I16x8WidenLowI8x16U,
    I16x8WidenHighI8x16U,
    I32x4WidenLowI16x8S,
    I32x4WidenHighI16x8S,
    I32x4WidenLowI16x8U,
    I32x4WidenHighI16x8U,
    I64x2WidenLowI32x4S,
    I64x2WidenHighI32x4S,
    I64x2WidenLowI32x4U,
    I64x2WidenHighI32x4U,
    I16x8ExtMulLowI8x16S,
    I16x8ExtMulHighI8x16S,
    I16x8ExtMulLowI8x16U,
    I16x8ExtMulHighI8x16U,
    I32x4ExtMulLowI16x8S,
    I32x4ExtMulHighI16x8S,
    I32x4ExtMulLowI16x8U,
    I32x4ExtMulHighI16x8U,
    I64x2ExtMulLowI32x4S,
    I64x2ExtMulHighI32x4S,
    I64x2ExtMulLowI32x4U,
    I64x2ExtMulHighI32x4U,
    V128Load8x8S,
    V128Load8x8U,
    V128Load16x4S,
    V128Load16x4U,
    V128Load32x2S,
    V128Load32x2U,
    V128Load8Lane,
    V128Load16Lane,
    V128Load32Lane,
    V128Load64Lane,
    V128Store8Lane,
    V128Store16Lane,
    V128Store32Lane,
    V128Store64Lane,
    I8x16RoundingAverageU,
    I16x8RoundingAverageU,
    I16x8Q15MulrSatS,
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
}

impl<'a> From<&Operator<'a>> for wasmer_parser_operator_t {
    fn from(operator: &Operator<'a>) -> Self {
        use Operator::*;

        match operator {
            Unreachable => Self::Unreachable,
            Nop => Self::Nop,
            Block { .. } => Self::Block,
            Loop { .. } => Self::Loop,
            If { .. } => Self::If,
            Else => Self::Else,
            Try { .. } => Self::Try,
            Catch { .. } => Self::Catch,
            Throw { .. } => Self::Throw,
            Rethrow { .. } => Self::Rethrow,
            Unwind => Self::Unwind,
            End => Self::End,
            Br { .. } => Self::Br,
            BrIf { .. } => Self::BrIf,
            BrTable { .. } => Self::BrTable,
            Return => Self::Return,
            Call { .. } => Self::Call,
            CallIndirect { .. } => Self::CallIndirect,
            ReturnCall { .. } => Self::ReturnCall,
            ReturnCallIndirect { .. } => Self::ReturnCallIndirect,
            Drop => Self::Drop,
            Select => Self::Select,
            TypedSelect { .. } => Self::TypedSelect,
            LocalGet { .. } => Self::LocalGet,
            LocalSet { .. } => Self::LocalSet,
            LocalTee { .. } => Self::LocalTee,
            GlobalGet { .. } => Self::GlobalGet,
            GlobalSet { .. } => Self::GlobalSet,
            I32Load { .. } => Self::I32Load,
            I64Load { .. } => Self::I64Load,
            F32Load { .. } => Self::F32Load,
            F64Load { .. } => Self::F64Load,
            I32Load8S { .. } => Self::I32Load8S,
            I32Load8U { .. } => Self::I32Load8U,
            I32Load16S { .. } => Self::I32Load16S,
            I32Load16U { .. } => Self::I32Load16U,
            I64Load8S { .. } => Self::I64Load8S,
            I64Load8U { .. } => Self::I64Load8U,
            I64Load16S { .. } => Self::I64Load16S,
            I64Load16U { .. } => Self::I64Load16U,
            I64Load32S { .. } => Self::I64Load32S,
            I64Load32U { .. } => Self::I64Load32U,
            I32Store { .. } => Self::I32Store,
            I64Store { .. } => Self::I64Store,
            F32Store { .. } => Self::F32Store,
            F64Store { .. } => Self::F64Store,
            I32Store8 { .. } => Self::I32Store8,
            I32Store16 { .. } => Self::I32Store16,
            I64Store8 { .. } => Self::I64Store8,
            I64Store16 { .. } => Self::I64Store16,
            I64Store32 { .. } => Self::I64Store32,
            MemorySize { .. } => Self::MemorySize,
            MemoryGrow { .. } => Self::MemoryGrow,
            I32Const { .. } => Self::I32Const,
            I64Const { .. } => Self::I64Const,
            F32Const { .. } => Self::F32Const,
            F64Const { .. } => Self::F64Const,
            RefNull { .. } => Self::RefNull,
            RefIsNull => Self::RefIsNull,
            RefFunc { .. } => Self::RefFunc,
            I32Eqz => Self::I32Eqz,
            I32Eq => Self::I32Eq,
            I32Ne => Self::I32Ne,
            I32LtS => Self::I32LtS,
            I32LtU => Self::I32LtU,
            I32GtS => Self::I32GtS,
            I32GtU => Self::I32GtU,
            I32LeS => Self::I32LeS,
            I32LeU => Self::I32LeU,
            I32GeS => Self::I32GeS,
            I32GeU => Self::I32GeU,
            I64Eqz => Self::I64Eqz,
            I64Eq => Self::I64Eq,
            I64Ne => Self::I64Ne,
            I64LtS => Self::I64LtS,
            I64LtU => Self::I64LtU,
            I64GtS => Self::I64GtS,
            I64GtU => Self::I64GtU,
            I64LeS => Self::I64LeS,
            I64LeU => Self::I64LeU,
            I64GeS => Self::I64GeS,
            I64GeU => Self::I64GeU,
            F32Eq => Self::F32Eq,
            F32Ne => Self::F32Ne,
            F32Lt => Self::F32Lt,
            F32Gt => Self::F32Gt,
            F32Le => Self::F32Le,
            F32Ge => Self::F32Ge,
            F64Eq => Self::F64Eq,
            F64Ne => Self::F64Ne,
            F64Lt => Self::F64Lt,
            F64Gt => Self::F64Gt,
            F64Le => Self::F64Le,
            F64Ge => Self::F64Ge,
            I32Clz => Self::I32Clz,
            I32Ctz => Self::I32Ctz,
            I32Popcnt => Self::I32Popcnt,
            I32Add => Self::I32Add,
            I32Sub => Self::I32Sub,
            I32Mul => Self::I32Mul,
            I32DivS => Self::I32DivS,
            I32DivU => Self::I32DivU,
            I32RemS => Self::I32RemS,
            I32RemU => Self::I32RemU,
            I32And => Self::I32And,
            I32Or => Self::I32Or,
            I32Xor => Self::I32Xor,
            I32Shl => Self::I32Shl,
            I32ShrS => Self::I32ShrS,
            I32ShrU => Self::I32ShrU,
            I32Rotl => Self::I32Rotl,
            I32Rotr => Self::I32Rotr,
            I64Clz => Self::I64Clz,
            I64Ctz => Self::I64Ctz,
            I64Popcnt => Self::I64Popcnt,
            I64Add => Self::I64Add,
            I64Sub => Self::I64Sub,
            I64Mul => Self::I64Mul,
            I64DivS => Self::I64DivS,
            I64DivU => Self::I64DivU,
            I64RemS => Self::I64RemS,
            I64RemU => Self::I64RemU,
            I64And => Self::I64And,
            I64Or => Self::I64Or,
            I64Xor => Self::I64Xor,
            I64Shl => Self::I64Shl,
            I64ShrS => Self::I64ShrS,
            I64ShrU => Self::I64ShrU,
            I64Rotl => Self::I64Rotl,
            I64Rotr => Self::I64Rotr,
            F32Abs => Self::F32Abs,
            F32Neg => Self::F32Neg,
            F32Ceil => Self::F32Ceil,
            F32Floor => Self::F32Floor,
            F32Trunc => Self::F32Trunc,
            F32Nearest => Self::F32Nearest,
            F32Sqrt => Self::F32Sqrt,
            F32Add => Self::F32Add,
            F32Sub => Self::F32Sub,
            F32Mul => Self::F32Mul,
            F32Div => Self::F32Div,
            F32Min => Self::F32Min,
            F32Max => Self::F32Max,
            F32Copysign => Self::F32Copysign,
            F64Abs => Self::F64Abs,
            F64Neg => Self::F64Neg,
            F64Ceil => Self::F64Ceil,
            F64Floor => Self::F64Floor,
            F64Trunc => Self::F64Trunc,
            F64Nearest => Self::F64Nearest,
            F64Sqrt => Self::F64Sqrt,
            F64Add => Self::F64Add,
            F64Sub => Self::F64Sub,
            F64Mul => Self::F64Mul,
            F64Div => Self::F64Div,
            F64Min => Self::F64Min,
            F64Max => Self::F64Max,
            F64Copysign => Self::F64Copysign,
            I32WrapI64 => Self::I32WrapI64,
            I32TruncF32S => Self::I32TruncF32S,
            I32TruncF32U => Self::I32TruncF32U,
            I32TruncF64S => Self::I32TruncF64S,
            I32TruncF64U => Self::I32TruncF64U,
            I64ExtendI32S => Self::I64ExtendI32S,
            I64ExtendI32U => Self::I64ExtendI32U,
            I64TruncF32S => Self::I64TruncF32S,
            I64TruncF32U => Self::I64TruncF32U,
            I64TruncF64S => Self::I64TruncF64S,
            I64TruncF64U => Self::I64TruncF64U,
            F32ConvertI32S => Self::F32ConvertI32S,
            F32ConvertI32U => Self::F32ConvertI32U,
            F32ConvertI64S => Self::F32ConvertI64S,
            F32ConvertI64U => Self::F32ConvertI64U,
            F32DemoteF64 => Self::F32DemoteF64,
            F64ConvertI32S => Self::F64ConvertI32S,
            F64ConvertI32U => Self::F64ConvertI32U,
            F64ConvertI64S => Self::F64ConvertI64S,
            F64ConvertI64U => Self::F64ConvertI64U,
            F64PromoteF32 => Self::F64PromoteF32,
            I32ReinterpretF32 => Self::I32ReinterpretF32,
            I64ReinterpretF64 => Self::I64ReinterpretF64,
            F32ReinterpretI32 => Self::F32ReinterpretI32,
            F64ReinterpretI64 => Self::F64ReinterpretI64,
            I32Extend8S => Self::I32Extend8S,
            I32Extend16S => Self::I32Extend16S,
            I64Extend8S => Self::I64Extend8S,
            I64Extend16S => Self::I64Extend16S,
            I64Extend32S => Self::I64Extend32S,
            I32TruncSatF32S => Self::I32TruncSatF32S,
            I32TruncSatF32U => Self::I32TruncSatF32U,
            I32TruncSatF64S => Self::I32TruncSatF64S,
            I32TruncSatF64U => Self::I32TruncSatF64U,
            I64TruncSatF32S => Self::I64TruncSatF32S,
            I64TruncSatF32U => Self::I64TruncSatF32U,
            I64TruncSatF64S => Self::I64TruncSatF64S,
            I64TruncSatF64U => Self::I64TruncSatF64U,
            MemoryInit { .. } => Self::MemoryInit,
            DataDrop { .. } => Self::DataDrop,
            MemoryCopy { .. } => Self::MemoryCopy,
            MemoryFill { .. } => Self::MemoryFill,
            TableInit { .. } => Self::TableInit,
            ElemDrop { .. } => Self::ElemDrop,
            TableCopy { .. } => Self::TableCopy,
            TableFill { .. } => Self::TableFill,
            TableGet { .. } => Self::TableGet,
            TableSet { .. } => Self::TableSet,
            TableGrow { .. } => Self::TableGrow,
            TableSize { .. } => Self::TableSize,
            MemoryAtomicNotify { .. } => Self::MemoryAtomicNotify,
            MemoryAtomicWait32 { .. } => Self::MemoryAtomicWait32,
            MemoryAtomicWait64 { .. } => Self::MemoryAtomicWait64,
            AtomicFence { .. } => Self::AtomicFence,
            I32AtomicLoad { .. } => Self::I32AtomicLoad,
            I64AtomicLoad { .. } => Self::I64AtomicLoad,
            I32AtomicLoad8U { .. } => Self::I32AtomicLoad8U,
            I32AtomicLoad16U { .. } => Self::I32AtomicLoad16U,
            I64AtomicLoad8U { .. } => Self::I64AtomicLoad8U,
            I64AtomicLoad16U { .. } => Self::I64AtomicLoad16U,
            I64AtomicLoad32U { .. } => Self::I64AtomicLoad32U,
            I32AtomicStore { .. } => Self::I32AtomicStore,
            I64AtomicStore { .. } => Self::I64AtomicStore,
            I32AtomicStore8 { .. } => Self::I32AtomicStore8,
            I32AtomicStore16 { .. } => Self::I32AtomicStore16,
            I64AtomicStore8 { .. } => Self::I64AtomicStore8,
            I64AtomicStore16 { .. } => Self::I64AtomicStore16,
            I64AtomicStore32 { .. } => Self::I64AtomicStore32,
            I32AtomicRmwAdd { .. } => Self::I32AtomicRmwAdd,
            I64AtomicRmwAdd { .. } => Self::I64AtomicRmwAdd,
            I32AtomicRmw8AddU { .. } => Self::I32AtomicRmw8AddU,
            I32AtomicRmw16AddU { .. } => Self::I32AtomicRmw16AddU,
            I64AtomicRmw8AddU { .. } => Self::I64AtomicRmw8AddU,
            I64AtomicRmw16AddU { .. } => Self::I64AtomicRmw16AddU,
            I64AtomicRmw32AddU { .. } => Self::I64AtomicRmw32AddU,
            I32AtomicRmwSub { .. } => Self::I32AtomicRmwSub,
            I64AtomicRmwSub { .. } => Self::I64AtomicRmwSub,
            I32AtomicRmw8SubU { .. } => Self::I32AtomicRmw8SubU,
            I32AtomicRmw16SubU { .. } => Self::I32AtomicRmw16SubU,
            I64AtomicRmw8SubU { .. } => Self::I64AtomicRmw8SubU,
            I64AtomicRmw16SubU { .. } => Self::I64AtomicRmw16SubU,
            I64AtomicRmw32SubU { .. } => Self::I64AtomicRmw32SubU,
            I32AtomicRmwAnd { .. } => Self::I32AtomicRmwAnd,
            I64AtomicRmwAnd { .. } => Self::I64AtomicRmwAnd,
            I32AtomicRmw8AndU { .. } => Self::I32AtomicRmw8AndU,
            I32AtomicRmw16AndU { .. } => Self::I32AtomicRmw16AndU,
            I64AtomicRmw8AndU { .. } => Self::I64AtomicRmw8AndU,
            I64AtomicRmw16AndU { .. } => Self::I64AtomicRmw16AndU,
            I64AtomicRmw32AndU { .. } => Self::I64AtomicRmw32AndU,
            I32AtomicRmwOr { .. } => Self::I32AtomicRmwOr,
            I64AtomicRmwOr { .. } => Self::I64AtomicRmwOr,
            I32AtomicRmw8OrU { .. } => Self::I32AtomicRmw8OrU,
            I32AtomicRmw16OrU { .. } => Self::I32AtomicRmw16OrU,
            I64AtomicRmw8OrU { .. } => Self::I64AtomicRmw8OrU,
            I64AtomicRmw16OrU { .. } => Self::I64AtomicRmw16OrU,
            I64AtomicRmw32OrU { .. } => Self::I64AtomicRmw32OrU,
            I32AtomicRmwXor { .. } => Self::I32AtomicRmwXor,
            I64AtomicRmwXor { .. } => Self::I64AtomicRmwXor,
            I32AtomicRmw8XorU { .. } => Self::I32AtomicRmw8XorU,
            I32AtomicRmw16XorU { .. } => Self::I32AtomicRmw16XorU,
            I64AtomicRmw8XorU { .. } => Self::I64AtomicRmw8XorU,
            I64AtomicRmw16XorU { .. } => Self::I64AtomicRmw16XorU,
            I64AtomicRmw32XorU { .. } => Self::I64AtomicRmw32XorU,
            I32AtomicRmwXchg { .. } => Self::I32AtomicRmwXchg,
            I64AtomicRmwXchg { .. } => Self::I64AtomicRmwXchg,
            I32AtomicRmw8XchgU { .. } => Self::I32AtomicRmw8XchgU,
            I32AtomicRmw16XchgU { .. } => Self::I32AtomicRmw16XchgU,
            I64AtomicRmw8XchgU { .. } => Self::I64AtomicRmw8XchgU,
            I64AtomicRmw16XchgU { .. } => Self::I64AtomicRmw16XchgU,
            I64AtomicRmw32XchgU { .. } => Self::I64AtomicRmw32XchgU,
            I32AtomicRmwCmpxchg { .. } => Self::I32AtomicRmwCmpxchg,
            I64AtomicRmwCmpxchg { .. } => Self::I64AtomicRmwCmpxchg,
            I32AtomicRmw8CmpxchgU { .. } => Self::I32AtomicRmw8CmpxchgU,
            I32AtomicRmw16CmpxchgU { .. } => Self::I32AtomicRmw16CmpxchgU,
            I64AtomicRmw8CmpxchgU { .. } => Self::I64AtomicRmw8CmpxchgU,
            I64AtomicRmw16CmpxchgU { .. } => Self::I64AtomicRmw16CmpxchgU,
            I64AtomicRmw32CmpxchgU { .. } => Self::I64AtomicRmw32CmpxchgU,
            V128Load { .. } => Self::V128Load,
            V128Store { .. } => Self::V128Store,
            V128Const { .. } => Self::V128Const,
            I8x16Splat => Self::I8x16Splat,
            I8x16ExtractLaneS { .. } => Self::I8x16ExtractLaneS,
            I8x16ExtractLaneU { .. } => Self::I8x16ExtractLaneU,
            I8x16ReplaceLane { .. } => Self::I8x16ReplaceLane,
            I16x8Splat => Self::I16x8Splat,
            I16x8ExtractLaneS { .. } => Self::I16x8ExtractLaneS,
            I16x8ExtractLaneU { .. } => Self::I16x8ExtractLaneU,
            I16x8ReplaceLane { .. } => Self::I16x8ReplaceLane,
            I32x4Splat => Self::I32x4Splat,
            I32x4ExtractLane { .. } => Self::I32x4ExtractLane,
            I32x4ReplaceLane { .. } => Self::I32x4ReplaceLane,
            I64x2Splat => Self::I64x2Splat,
            I64x2ExtractLane { .. } => Self::I64x2ExtractLane,
            I64x2ReplaceLane { .. } => Self::I64x2ReplaceLane,
            F32x4Splat => Self::F32x4Splat,
            F32x4ExtractLane { .. } => Self::F32x4ExtractLane,
            F32x4ReplaceLane { .. } => Self::F32x4ReplaceLane,
            F64x2Splat => Self::F64x2Splat,
            F64x2ExtractLane { .. } => Self::F64x2ExtractLane,
            F64x2ReplaceLane { .. } => Self::F64x2ReplaceLane,
            I8x16Eq => Self::I8x16Eq,
            I8x16Ne => Self::I8x16Ne,
            I8x16LtS => Self::I8x16LtS,
            I8x16LtU => Self::I8x16LtU,
            I8x16GtS => Self::I8x16GtS,
            I8x16GtU => Self::I8x16GtU,
            I8x16LeS => Self::I8x16LeS,
            I8x16LeU => Self::I8x16LeU,
            I8x16GeS => Self::I8x16GeS,
            I8x16GeU => Self::I8x16GeU,
            I16x8Eq => Self::I16x8Eq,
            I16x8Ne => Self::I16x8Ne,
            I16x8LtS => Self::I16x8LtS,
            I16x8LtU => Self::I16x8LtU,
            I16x8GtS => Self::I16x8GtS,
            I16x8GtU => Self::I16x8GtU,
            I16x8LeS => Self::I16x8LeS,
            I16x8LeU => Self::I16x8LeU,
            I16x8GeS => Self::I16x8GeS,
            I16x8GeU => Self::I16x8GeU,
            I32x4Eq => Self::I32x4Eq,
            I32x4Ne => Self::I32x4Ne,
            I32x4LtS => Self::I32x4LtS,
            I32x4LtU => Self::I32x4LtU,
            I32x4GtS => Self::I32x4GtS,
            I32x4GtU => Self::I32x4GtU,
            I32x4LeS => Self::I32x4LeS,
            I32x4LeU => Self::I32x4LeU,
            I32x4GeS => Self::I32x4GeS,
            I32x4GeU => Self::I32x4GeU,
            I64x2Eq => Self::I64x2Eq,
            I64x2Ne => Self::I64x2Ne,
            F32x4Eq => Self::F32x4Eq,
            F32x4Ne => Self::F32x4Ne,
            F32x4Lt => Self::F32x4Lt,
            F32x4Gt => Self::F32x4Gt,
            F32x4Le => Self::F32x4Le,
            F32x4Ge => Self::F32x4Ge,
            F64x2Eq => Self::F64x2Eq,
            F64x2Ne => Self::F64x2Ne,
            F64x2Lt => Self::F64x2Lt,
            F64x2Gt => Self::F64x2Gt,
            F64x2Le => Self::F64x2Le,
            F64x2Ge => Self::F64x2Ge,
            V128Not => Self::V128Not,
            V128And => Self::V128And,
            V128AndNot => Self::V128AndNot,
            V128Or => Self::V128Or,
            V128Xor => Self::V128Xor,
            V128Bitselect => Self::V128Bitselect,
            V128AnyTrue => Self::V128AnyTrue,
            I8x16Abs => Self::I8x16Abs,
            I8x16Neg => Self::I8x16Neg,
            I8x16AllTrue => Self::I8x16AllTrue,
            I8x16Bitmask => Self::I8x16Bitmask,
            I8x16Shl => Self::I8x16Shl,
            I8x16ShrS => Self::I8x16ShrS,
            I8x16ShrU => Self::I8x16ShrU,
            I8x16Add => Self::I8x16Add,
            I8x16AddSatS => Self::I8x16AddSatS,
            I8x16AddSatU => Self::I8x16AddSatU,
            I8x16Sub => Self::I8x16Sub,
            I8x16SubSatS => Self::I8x16SubSatS,
            I8x16SubSatU => Self::I8x16SubSatU,
            I8x16MinS => Self::I8x16MinS,
            I8x16MinU => Self::I8x16MinU,
            I8x16MaxS => Self::I8x16MaxS,
            I8x16MaxU => Self::I8x16MaxU,
            I16x8Abs => Self::I16x8Abs,
            I16x8Neg => Self::I16x8Neg,
            I16x8AllTrue => Self::I16x8AllTrue,
            I16x8Bitmask => Self::I16x8Bitmask,
            I16x8Shl => Self::I16x8Shl,
            I16x8ShrS => Self::I16x8ShrS,
            I16x8ShrU => Self::I16x8ShrU,
            I16x8Add => Self::I16x8Add,
            I16x8AddSatS => Self::I16x8AddSatS,
            I16x8AddSatU => Self::I16x8AddSatU,
            I16x8Sub => Self::I16x8Sub,
            I16x8SubSatS => Self::I16x8SubSatS,
            I16x8SubSatU => Self::I16x8SubSatU,
            I16x8Mul => Self::I16x8Mul,
            I16x8MinS => Self::I16x8MinS,
            I16x8MinU => Self::I16x8MinU,
            I16x8MaxS => Self::I16x8MaxS,
            I16x8MaxU => Self::I16x8MaxU,
            I32x4Abs => Self::I32x4Abs,
            I32x4Neg => Self::I32x4Neg,
            I32x4AllTrue => Self::I32x4AllTrue,
            I32x4Bitmask => Self::I32x4Bitmask,
            I32x4Shl => Self::I32x4Shl,
            I32x4ShrS => Self::I32x4ShrS,
            I32x4ShrU => Self::I32x4ShrU,
            I32x4Add => Self::I32x4Add,
            I32x4Sub => Self::I32x4Sub,
            I32x4Mul => Self::I32x4Mul,
            I32x4MinS => Self::I32x4MinS,
            I32x4MinU => Self::I32x4MinU,
            I32x4MaxS => Self::I32x4MaxS,
            I32x4MaxU => Self::I32x4MaxU,
            I32x4DotI16x8S => Self::I32x4DotI16x8S,
            I64x2Neg => Self::I64x2Neg,
            I64x2AllTrue => Self::I64x2AllTrue,
            I64x2Bitmask => Self::I64x2Bitmask,
            I64x2Shl => Self::I64x2Shl,
            I64x2ShrS => Self::I64x2ShrS,
            I64x2ShrU => Self::I64x2ShrU,
            I64x2Add => Self::I64x2Add,
            I64x2Sub => Self::I64x2Sub,
            I64x2Mul => Self::I64x2Mul,
            F32x4Ceil => Self::F32x4Ceil,
            F32x4Floor => Self::F32x4Floor,
            F32x4Trunc => Self::F32x4Trunc,
            F32x4Nearest => Self::F32x4Nearest,
            F64x2Ceil => Self::F64x2Ceil,
            F64x2Floor => Self::F64x2Floor,
            F64x2Trunc => Self::F64x2Trunc,
            F64x2Nearest => Self::F64x2Nearest,
            F32x4Abs => Self::F32x4Abs,
            F32x4Neg => Self::F32x4Neg,
            F32x4Sqrt => Self::F32x4Sqrt,
            F32x4Add => Self::F32x4Add,
            F32x4Sub => Self::F32x4Sub,
            F32x4Mul => Self::F32x4Mul,
            F32x4Div => Self::F32x4Div,
            F32x4Min => Self::F32x4Min,
            F32x4Max => Self::F32x4Max,
            F32x4PMin => Self::F32x4PMin,
            F32x4PMax => Self::F32x4PMax,
            F64x2Abs => Self::F64x2Abs,
            F64x2Neg => Self::F64x2Neg,
            F64x2Sqrt => Self::F64x2Sqrt,
            F64x2Add => Self::F64x2Add,
            F64x2Sub => Self::F64x2Sub,
            F64x2Mul => Self::F64x2Mul,
            F64x2Div => Self::F64x2Div,
            F64x2Min => Self::F64x2Min,
            F64x2Max => Self::F64x2Max,
            F64x2PMin => Self::F64x2PMin,
            F64x2PMax => Self::F64x2PMax,
            I32x4TruncSatF32x4S => Self::I32x4TruncSatF32x4S,
            I32x4TruncSatF32x4U => Self::I32x4TruncSatF32x4U,
            F32x4ConvertI32x4S => Self::F32x4ConvertI32x4S,
            F32x4ConvertI32x4U => Self::F32x4ConvertI32x4U,
            I8x16Swizzle => Self::I8x16Swizzle,
            I8x16Shuffle { .. } => Self::I8x16Shuffle,
            V128Load8Splat { .. } => Self::V128Load8Splat,
            V128Load16Splat { .. } => Self::V128Load16Splat,
            V128Load32Splat { .. } => Self::V128Load32Splat,
            V128Load32Zero { .. } => Self::V128Load32Zero,
            V128Load64Splat { .. } => Self::V128Load64Splat,
            V128Load64Zero { .. } => Self::V128Load64Zero,
            I8x16NarrowI16x8S => Self::I8x16NarrowI16x8S,
            I8x16NarrowI16x8U => Self::I8x16NarrowI16x8U,
            I16x8NarrowI32x4S => Self::I16x8NarrowI32x4S,
            I16x8NarrowI32x4U => Self::I16x8NarrowI32x4U,
            I16x8WidenLowI8x16S => Self::I16x8WidenLowI8x16S,
            I16x8WidenHighI8x16S => Self::I16x8WidenHighI8x16S,
            I16x8WidenLowI8x16U => Self::I16x8WidenLowI8x16U,
            I16x8WidenHighI8x16U => Self::I16x8WidenHighI8x16U,
            I32x4WidenLowI16x8S => Self::I32x4WidenLowI16x8S,
            I32x4WidenHighI16x8S => Self::I32x4WidenHighI16x8S,
            I32x4WidenLowI16x8U => Self::I32x4WidenLowI16x8U,
            I32x4WidenHighI16x8U => Self::I32x4WidenHighI16x8U,
            I64x2WidenLowI32x4S => Self::I64x2WidenLowI32x4S,
            I64x2WidenHighI32x4S => Self::I64x2WidenHighI32x4S,
            I64x2WidenLowI32x4U => Self::I64x2WidenLowI32x4U,
            I64x2WidenHighI32x4U => Self::I64x2WidenHighI32x4U,
            I16x8ExtMulLowI8x16S => Self::I16x8ExtMulLowI8x16S,
            I16x8ExtMulHighI8x16S => Self::I16x8ExtMulHighI8x16S,
            I16x8ExtMulLowI8x16U => Self::I16x8ExtMulLowI8x16U,
            I16x8ExtMulHighI8x16U => Self::I16x8ExtMulHighI8x16U,
            I32x4ExtMulLowI16x8S => Self::I32x4ExtMulLowI16x8S,
            I32x4ExtMulHighI16x8S => Self::I32x4ExtMulHighI16x8S,
            I32x4ExtMulLowI16x8U => Self::I32x4ExtMulLowI16x8U,
            I32x4ExtMulHighI16x8U => Self::I32x4ExtMulHighI16x8U,
            I64x2ExtMulLowI32x4S => Self::I64x2ExtMulLowI32x4S,
            I64x2ExtMulHighI32x4S => Self::I64x2ExtMulHighI32x4S,
            I64x2ExtMulLowI32x4U => Self::I64x2ExtMulLowI32x4U,
            I64x2ExtMulHighI32x4U => Self::I64x2ExtMulHighI32x4U,
            V128Load8x8S { .. } => Self::V128Load8x8S,
            V128Load8x8U { .. } => Self::V128Load8x8U,
            V128Load16x4S { .. } => Self::V128Load16x4S,
            V128Load16x4U { .. } => Self::V128Load16x4U,
            V128Load32x2S { .. } => Self::V128Load32x2S,
            V128Load32x2U { .. } => Self::V128Load32x2U,
            V128Load8Lane { .. } => Self::V128Load8Lane,
            V128Load16Lane { .. } => Self::V128Load16Lane,
            V128Load32Lane { .. } => Self::V128Load32Lane,
            V128Load64Lane { .. } => Self::V128Load64Lane,
            V128Store8Lane { .. } => Self::V128Store8Lane,
            V128Store16Lane { .. } => Self::V128Store16Lane,
            V128Store32Lane { .. } => Self::V128Store32Lane,
            V128Store64Lane { .. } => Self::V128Store64Lane,
            I8x16RoundingAverageU => Self::I8x16RoundingAverageU,
            I16x8RoundingAverageU => Self::I16x8RoundingAverageU,
            I16x8Q15MulrSatS => Self::I16x8Q15MulrSatS,
            F32x4DemoteF64x2Zero => Self::F32x4DemoteF64x2Zero,
            F64x2PromoteLowF32x4 => Self::F64x2PromoteLowF32x4,
            F64x2ConvertLowI32x4S => Self::F64x2ConvertLowI32x4S,
            F64x2ConvertLowI32x4U => Self::F64x2ConvertLowI32x4U,
            I32x4TruncSatF64x2SZero => Self::I32x4TruncSatF64x2SZero,
            I32x4TruncSatF64x2UZero => Self::I32x4TruncSatF64x2UZero,
        }
    }
}
