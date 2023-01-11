use super::{Felt, ProcedureId, Vec};
use core::fmt;

// NODES
// ================================================================================================

/// A node in a AST that can represent a block, instruction or a control flow.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Instruction(Instruction),
    IfElse(Vec<Node>, Vec<Node>),
    Repeat(u16, Vec<Node>),
    While(Vec<Node>),
}

/// This holds the list of instructions supported in a Miden program.
/// This instruction list is used to hold reference to the instruction, and future be
/// used for MAST generation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Assert,
    AssertEq,
    Assertz,
    Add,
    AddImm(Felt),
    Sub,
    SubImm(Felt),
    Mul,
    MulImm(Felt),
    Div,
    DivImm(Felt),
    Neg,
    Inv,
    Incr,
    Pow2,
    Exp,
    ExpImm(Felt),
    ExpBitLength(u8),
    Not,
    And,
    Or,
    Xor,
    Eq,
    EqImm(Felt),
    Neq,
    NeqImm(Felt),
    Eqw,
    Lt,
    Lte,
    Gt,
    Gte,

    // ----- ext2 operations ----------------------------------------------------------------------
    Ext2Add,
    Ext2Sub,
    Ext2Mul,
    Ext2Div,
    Ext2Neg,
    Ext2Inv,

    // ----- u32 manipulation ---------------------------------------------------------------------
    U32Test,
    U32TestW,
    U32Assert,
    U32Assert2,
    U32AssertW,
    U32Split,
    U32Cast,
    U32CheckedAdd,
    U32CheckedAddImm(u32),
    U32WrappingAdd,
    U32WrappingAddImm(u32),
    U32OverflowingAdd,
    U32OverflowingAddImm(u32),
    U32OverflowingAdd3,
    U32WrappingAdd3,
    U32CheckedSub,
    U32CheckedSubImm(u32),
    U32WrappingSub,
    U32WrappingSubImm(u32),
    U32OverflowingSub,
    U32OverflowingSubImm(u32),
    U32CheckedMul,
    U32CheckedMulImm(u32),
    U32WrappingMul,
    U32WrappingMulImm(u32),
    U32OverflowingMul,
    U32OverflowingMulImm(u32),
    U32OverflowingMadd,
    U32WrappingMadd,
    U32CheckedDiv,
    U32CheckedDivImm(u32),
    U32UncheckedDiv,
    U32UncheckedDivImm(u32),
    U32CheckedMod,
    U32CheckedModImm(u32),
    U32UncheckedMod,
    U32UncheckedModImm(u32),
    U32CheckedDivMod,
    U32CheckedDivModImm(u32),
    U32UncheckedDivMod,
    U32UncheckedDivModImm(u32),
    U32CheckedAnd,
    U32CheckedOr,
    U32CheckedXor,
    U32CheckedNot,
    U32CheckedShr,
    U32CheckedShrImm(u8),
    U32UncheckedShr,
    U32UncheckedShrImm(u8),
    U32CheckedShl,
    U32CheckedShlImm(u8),
    U32UncheckedShl,
    U32UncheckedShlImm(u8),
    U32CheckedRotr,
    U32CheckedRotrImm(u8),
    U32UncheckedRotr,
    U32UncheckedRotrImm(u8),
    U32CheckedRotl,
    U32CheckedRotlImm(u8),
    U32UncheckedRotl,
    U32UncheckedRotlImm(u8),
    U32CheckedPopcnt,
    U32UncheckedPopcnt,
    U32CheckedEq,
    U32CheckedEqImm(u32),
    U32CheckedNeq,
    U32CheckedNeqImm(u32),
    U32CheckedLt,
    U32UncheckedLt,
    U32CheckedLte,
    U32UncheckedLte,
    U32CheckedGt,
    U32UncheckedGt,
    U32CheckedGte,
    U32UncheckedGte,
    U32CheckedMin,
    U32UncheckedMin,
    U32CheckedMax,
    U32UncheckedMax,

    // ----- stack manipulation -------------------------------------------------------------------
    Drop,
    DropW,
    PadW,
    Dup0,
    Dup1,
    Dup2,
    Dup3,
    Dup4,
    Dup5,
    Dup6,
    Dup7,
    Dup8,
    Dup9,
    Dup10,
    Dup11,
    Dup12,
    Dup13,
    Dup14,
    Dup15,
    DupW0,
    DupW1,
    DupW2,
    DupW3,
    Swap1,
    Swap2,
    Swap3,
    Swap4,
    Swap5,
    Swap6,
    Swap7,
    Swap8,
    Swap9,
    Swap10,
    Swap11,
    Swap12,
    Swap13,
    Swap14,
    Swap15,
    SwapW1,
    SwapW2,
    SwapW3,
    SwapDw,
    MovUp2,
    MovUp3,
    MovUp4,
    MovUp5,
    MovUp6,
    MovUp7,
    MovUp8,
    MovUp9,
    MovUp10,
    MovUp11,
    MovUp12,
    MovUp13,
    MovUp14,
    MovUp15,
    MovUpW2,
    MovUpW3,
    MovDn2,
    MovDn3,
    MovDn4,
    MovDn5,
    MovDn6,
    MovDn7,
    MovDn8,
    MovDn9,
    MovDn10,
    MovDn11,
    MovDn12,
    MovDn13,
    MovDn14,
    MovDn15,
    MovDnW2,
    MovDnW3,
    CSwap,
    CSwapW,
    CDrop,
    CDropW,

    // ----- input / output operations ------------------------------------------------------------
    PushU8(u8),
    PushU16(u16),
    PushU32(u32),
    PushFelt(Felt),
    PushWord([Felt; 4]),
    PushU8List(Vec<u8>),
    PushU16List(Vec<u16>),
    PushU32List(Vec<u32>),
    PushFeltList(Vec<Felt>),
    Locaddr(u16),
    Sdepth,
    Caller,

    MemLoad,
    MemLoadImm(u32),
    MemLoadW,
    MemLoadWImm(u32),
    LocLoad(u16),
    LocLoadW(u16),

    MemStore,
    MemStoreImm(u32),
    LocStore(u16),
    MemStoreW,
    MemStoreWImm(u32),
    LocStoreW(u16),

    MemStream,
    AdvPipe,

    AdvPush(u8),
    AdvLoadW,

    AdvU64Div,
    AdvKeyval,
    AdvMem(u32, u32),
    AdvExt2Inv,
    AdvExt2INTT,

    // ----- cryptographic operations -------------------------------------------------------------
    RpHash,
    RpPerm,
    MTreeGet,
    MTreeSet,
    MTreeCwm,

    // ----- exec / call --------------------------------------------------------------------------
    ExecLocal(u16),
    ExecImported(ProcedureId),
    CallLocal(u16),
    CallImported(ProcedureId),
    SysCall(ProcedureId),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assert => write!(f, "assert"),
            Self::AssertEq => write!(f, "assert_eq"),
            Self::Assertz => write!(f, "assertz"),
            Self::Add => write!(f, "add"),
            Self::AddImm(value) => write!(f, "add.{value}"),
            Self::Sub => write!(f, "sub"),
            Self::SubImm(value) => write!(f, "sub.{value}"),
            Self::Mul => write!(f, "mul"),
            Self::MulImm(value) => write!(f, "mul.{value}"),
            Self::Div => write!(f, "div"),
            Self::DivImm(value) => write!(f, "div.{value}"),
            Self::Neg => write!(f, "neg"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "add.1"),
            Self::Pow2 => write!(f, "pow2"),
            Self::Exp => write!(f, "exp"),
            Self::ExpImm(value) => write!(f, "exp.{value}"),
            Self::ExpBitLength(value) => write!(f, "exp.u{value}"),
            Self::Not => write!(f, "not"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Eq => write!(f, "eq"),
            Self::EqImm(value) => write!(f, "eq.{value}"),
            Self::Neq => write!(f, "neq"),
            Self::NeqImm(value) => write!(f, "neq.{value}"),
            Self::Eqw => write!(f, "eqw"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),

            // ----- ext2 operations --------------------------------------------------------------
            Self::Ext2Add => write!(f, "ext2add"),
            Self::Ext2Sub => write!(f, "ext2sub"),
            Self::Ext2Mul => write!(f, "ext2mul"),
            Self::Ext2Div => write!(f, "ext2div"),
            Self::Ext2Neg => write!(f, "ext2neg"),
            Self::Ext2Inv => write!(f, "ext2inv"),

            // ----- u32 manipulation ---------------------------------------------------------------
            Self::U32Test => write!(f, "u32test"),
            Self::U32TestW => write!(f, "u32testw"),
            Self::U32Assert => write!(f, "u32assert.1"),
            Self::U32Assert2 => write!(f, "u32assert.2"),
            Self::U32AssertW => write!(f, "u32assertw"),
            Self::U32Split => write!(f, "u32split"),
            Self::U32Cast => write!(f, "u32cast"),
            Self::U32CheckedAdd => write!(f, "u32checked_add"),
            Self::U32CheckedAddImm(value) => write!(f, "u32checked_add.{value}"),
            Self::U32WrappingAdd => write!(f, "u32wrapping_add"),
            Self::U32WrappingAddImm(value) => write!(f, "u32wrapping_add.{value}"),
            Self::U32OverflowingAdd => write!(f, "u32overflowing_add"),
            Self::U32OverflowingAddImm(value) => write!(f, "u32overflowing_add.{value}"),
            Self::U32OverflowingAdd3 => write!(f, "u32overflowing_add3"),
            Self::U32WrappingAdd3 => write!(f, "u32wrapping_add3"),
            Self::U32CheckedSub => write!(f, "u32checked_sub"),
            Self::U32CheckedSubImm(value) => write!(f, "u32checked_sub.{value}"),
            Self::U32WrappingSub => write!(f, "u32wrapping_sub"),
            Self::U32WrappingSubImm(value) => write!(f, "u32wrapping_sub.{value}"),
            Self::U32OverflowingSub => write!(f, "u32overflowing_sub"),
            Self::U32OverflowingSubImm(value) => write!(f, "u32overflowing_sub.{value}"),
            Self::U32CheckedMul => write!(f, "u32checked_mul"),
            Self::U32CheckedMulImm(value) => write!(f, "u32checked_mul.{value}"),
            Self::U32WrappingMul => write!(f, "u32wrapping_mul"),
            Self::U32WrappingMulImm(value) => write!(f, "u32wrapping_mul.{value}"),
            Self::U32OverflowingMul => write!(f, "u32overflowing_mul"),
            Self::U32OverflowingMulImm(value) => write!(f, "u32overflowing_mul.{value}"),
            Self::U32OverflowingMadd => write!(f, "u32overflowing_madd"),
            Self::U32WrappingMadd => write!(f, "u32wrapping_madd"),
            Self::U32CheckedDiv => write!(f, "u32checked_div"),
            Self::U32CheckedDivImm(value) => write!(f, "u32checked_div.{value}"),
            Self::U32UncheckedDiv => write!(f, "u32unchecked_div"),
            Self::U32UncheckedDivImm(value) => write!(f, "u32unchecked_div.{value}"),
            Self::U32CheckedMod => write!(f, "u32checked_mod"),
            Self::U32CheckedModImm(value) => write!(f, "u32checked_mod.{value}"),
            Self::U32UncheckedMod => write!(f, "u32unchecked_mod"),
            Self::U32UncheckedModImm(value) => write!(f, "u32unchecked_mod.{value}"),
            Self::U32CheckedDivMod => write!(f, "u32checked_divmod"),
            Self::U32CheckedDivModImm(value) => write!(f, "u32checked_divmod.{value}"),
            Self::U32UncheckedDivMod => write!(f, "u32unchecked_divmod"),
            Self::U32UncheckedDivModImm(value) => write!(f, "u32unchecked_divmod.{value}"),
            Self::U32CheckedAnd => write!(f, "u32checked_and"),
            Self::U32CheckedOr => write!(f, "u32checked_or"),
            Self::U32CheckedXor => write!(f, "u32checked_xor"),
            Self::U32CheckedNot => write!(f, "u32checked_not"),
            Self::U32CheckedShr => write!(f, "u32checked_shr"),
            Self::U32CheckedShrImm(value) => write!(f, "u32checked_shr.{value}"),
            Self::U32UncheckedShr => write!(f, "u32unchecked_shr"),
            Self::U32UncheckedShrImm(value) => write!(f, "u32unchecked_shr.{value}"),
            Self::U32CheckedShl => write!(f, "u32checked_shl"),
            Self::U32CheckedShlImm(value) => write!(f, "u32checked_shl.{value}"),
            Self::U32UncheckedShl => write!(f, "u32unchecked_shl"),
            Self::U32UncheckedShlImm(value) => write!(f, "u32unchecked_shl.{value}"),
            Self::U32CheckedRotr => write!(f, "u32checked_rotr"),
            Self::U32CheckedRotrImm(value) => write!(f, "u32checked_rotr.{value}"),
            Self::U32UncheckedRotr => write!(f, "u32unchecked_rotr"),
            Self::U32UncheckedRotrImm(value) => write!(f, "u32unchecked_rotr.{value}"),
            Self::U32CheckedRotl => write!(f, "u32checked_rotl"),
            Self::U32CheckedRotlImm(value) => write!(f, "u32checked_rotl.{value}"),
            Self::U32UncheckedRotl => write!(f, "u32unchecked_rotl"),
            Self::U32UncheckedRotlImm(value) => write!(f, "u32unchecked_rotl.{value}"),
            Self::U32CheckedPopcnt => write!(f, "u32checked_popcnt"),
            Self::U32UncheckedPopcnt => write!(f, "u32unchecked_popcnt"),
            Self::U32CheckedEq => write!(f, "u32checked_eq"),
            Self::U32CheckedEqImm(value) => write!(f, "u32checked_eq.{value}"),
            Self::U32CheckedNeq => write!(f, "u32checked_neq"),
            Self::U32CheckedNeqImm(value) => write!(f, "u32checked_neq.{value}"),
            Self::U32CheckedLt => write!(f, "u32checked_lt"),
            Self::U32UncheckedLt => write!(f, "u32unchecked_lt"),
            Self::U32CheckedLte => write!(f, "u32checked_lte"),
            Self::U32UncheckedLte => write!(f, "u32unchecked_lte"),
            Self::U32CheckedGt => write!(f, "u32checked_gt"),
            Self::U32UncheckedGt => write!(f, "u32unchecked_gt"),
            Self::U32CheckedGte => write!(f, "u32checked_gte"),
            Self::U32UncheckedGte => write!(f, "u32unchecked_gte"),
            Self::U32CheckedMin => write!(f, "u32checked_min"),
            Self::U32UncheckedMin => write!(f, "u32unchecked_min"),
            Self::U32CheckedMax => write!(f, "u32checked_max"),
            Self::U32UncheckedMax => write!(f, "u32unchecked_max"),

            // ----- stack manipulation ---------------------------------------------------------------
            Self::Drop => write!(f, "drop"),
            Self::DropW => write!(f, "dropw"),
            Self::PadW => write!(f, "padw"),
            Self::Dup0 => write!(f, "dup.0"),
            Self::Dup1 => write!(f, "dup.1"),
            Self::Dup2 => write!(f, "dup.2"),
            Self::Dup3 => write!(f, "dup.3"),
            Self::Dup4 => write!(f, "dup.4"),
            Self::Dup5 => write!(f, "dup.5"),
            Self::Dup6 => write!(f, "dup.6"),
            Self::Dup7 => write!(f, "dup.7"),
            Self::Dup8 => write!(f, "dup.8"),
            Self::Dup9 => write!(f, "dup.9"),
            Self::Dup10 => write!(f, "dup.10"),
            Self::Dup11 => write!(f, "dup.11"),
            Self::Dup12 => write!(f, "dup.12"),
            Self::Dup13 => write!(f, "dup.13"),
            Self::Dup14 => write!(f, "dup.14"),
            Self::Dup15 => write!(f, "dup.15"),
            Self::DupW0 => write!(f, "dupw.0"),
            Self::DupW1 => write!(f, "dupw.1"),
            Self::DupW2 => write!(f, "dupw.2"),
            Self::DupW3 => write!(f, "dupw.3"),
            Self::Swap1 => write!(f, "swap.1"),
            Self::Swap2 => write!(f, "swap.2"),
            Self::Swap3 => write!(f, "swap.3"),
            Self::Swap4 => write!(f, "swap.4"),
            Self::Swap5 => write!(f, "swap.5"),
            Self::Swap6 => write!(f, "swap.6"),
            Self::Swap7 => write!(f, "swap.7"),
            Self::Swap8 => write!(f, "swap.8"),
            Self::Swap9 => write!(f, "swap.9"),
            Self::Swap10 => write!(f, "swap.10"),
            Self::Swap11 => write!(f, "swap.11"),
            Self::Swap12 => write!(f, "swap.12"),
            Self::Swap13 => write!(f, "swap.13"),
            Self::Swap14 => write!(f, "swap.14"),
            Self::Swap15 => write!(f, "swap.15"),
            Self::SwapW1 => write!(f, "swapw.1"),
            Self::SwapW2 => write!(f, "swapw.2"),
            Self::SwapW3 => write!(f, "swapw.3"),
            Self::SwapDw => write!(f, "swapdw"),
            Self::MovUp2 => write!(f, "movup.2"),
            Self::MovUp3 => write!(f, "movup.3"),
            Self::MovUp4 => write!(f, "movup.4"),
            Self::MovUp5 => write!(f, "movup.5"),
            Self::MovUp6 => write!(f, "movup.6"),
            Self::MovUp7 => write!(f, "movup.7"),
            Self::MovUp8 => write!(f, "movup.8"),
            Self::MovUp9 => write!(f, "movup.9"),
            Self::MovUp10 => write!(f, "movup.10"),
            Self::MovUp11 => write!(f, "movup.11"),
            Self::MovUp12 => write!(f, "movup.12"),
            Self::MovUp13 => write!(f, "movup.13"),
            Self::MovUp14 => write!(f, "movup.14"),
            Self::MovUp15 => write!(f, "movup.15"),
            Self::MovUpW2 => write!(f, "movupw.2"),
            Self::MovUpW3 => write!(f, "movupw.3"),
            Self::MovDn2 => write!(f, "movdn.2"),
            Self::MovDn3 => write!(f, "movdn.3"),
            Self::MovDn4 => write!(f, "movdn.4"),
            Self::MovDn5 => write!(f, "movdn.5"),
            Self::MovDn6 => write!(f, "movdn.6"),
            Self::MovDn7 => write!(f, "movdn.7"),
            Self::MovDn8 => write!(f, "movdn.8"),
            Self::MovDn9 => write!(f, "movdn.9"),
            Self::MovDn10 => write!(f, "movdn.10"),
            Self::MovDn11 => write!(f, "movdn.11"),
            Self::MovDn12 => write!(f, "movdn.12"),
            Self::MovDn13 => write!(f, "movdn.13"),
            Self::MovDn14 => write!(f, "movdn.14"),
            Self::MovDn15 => write!(f, "movdn.15"),
            Self::MovDnW2 => write!(f, "movdnw.2"),
            Self::MovDnW3 => write!(f, "movdnw.3"),
            Self::CSwap => write!(f, "cswap"),
            Self::CSwapW => write!(f, "cswapw"),
            Self::CDrop => write!(f, "cdrop"),
            Self::CDropW => write!(f, "cdropw"),

            // ----- input / output operations ----------------------------------------------------
            Self::PushU8(value) => write!(f, "push.{value}"),
            Self::PushU16(value) => write!(f, "push.{value}"),
            Self::PushU32(value) => write!(f, "push.{value}"),
            Self::PushFelt(value) => write!(f, "push.{value}"),
            Self::PushWord(values) => display_push_vec(f, values),
            Self::PushU8List(values) => display_push_vec(f, values),
            Self::PushU16List(values) => display_push_vec(f, values),
            Self::PushU32List(values) => display_push_vec(f, values),
            Self::PushFeltList(values) => display_push_vec(f, values),

            Self::Locaddr(value) => write!(f, "locaddr.{value}"),
            Self::Sdepth => write!(f, "sdepth"),
            Self::Caller => write!(f, "caller"),

            Self::MemLoad => write!(f, "mem_load"),
            Self::MemLoadImm(value) => write!(f, "mem_load.{value}"),
            Self::MemLoadW => write!(f, "mem_loadw"),
            Self::MemLoadWImm(value) => write!(f, "mem_loadw.{value}"),
            Self::LocLoad(value) => write!(f, "loc_load.{value}"),
            Self::LocLoadW(value) => write!(f, "loc_loadw.{value}"),

            Self::MemStore => write!(f, "mem_store"),
            Self::MemStoreImm(value) => write!(f, "mem_store.{value}"),
            Self::LocStore(value) => write!(f, "loc_store.{value}"),
            Self::MemStoreW => write!(f, "mem_storew"),
            Self::MemStoreWImm(value) => write!(f, "mem_storew.{value}"),
            Self::LocStoreW(value) => write!(f, "loc_storew.{value}"),

            Self::MemStream => write!(f, "mem_stream"),
            Self::AdvPipe => write!(f, "adv_pipe"),

            Self::AdvPush(value) => write!(f, "adv_push.{value}"),
            Self::AdvLoadW => write!(f, "adv_loadw"),

            Self::AdvU64Div => write!(f, "adv.u64div"),
            Self::AdvKeyval => write!(f, "adv.keyval"),
            Self::AdvMem(start_addr, num_words) => write!(f, "adv.mem.{start_addr}.{num_words}"),
            Self::AdvExt2Inv => write!(f, "adv.ext2inv"),
            Self::AdvExt2INTT => write!(f, "adv.ext2intt"),

            // ----- cryptographic operations -----------------------------------------------------
            Self::RpHash => write!(f, "rphash"),
            Self::RpPerm => write!(f, "rpperm"),
            Self::MTreeGet => write!(f, "mtree_get"),
            Self::MTreeSet => write!(f, "mtree_set"),
            Self::MTreeCwm => write!(f, "mtree_cwm"),

            // ----- exec / call ------------------------------------------------------------------
            // TODO: print exec/call instructions with procedures names, not indexes or id's
            Self::ExecLocal(index) => write!(f, "exec.{index}"),
            Self::ExecImported(proc_id) => write!(f, "exec.{proc_id}"),
            Self::CallLocal(index) => write!(f, "call.{index}"),
            Self::CallImported(proc_id) => write!(f, "call.{proc_id}"),
            Self::SysCall(proc_id) => write!(f, "syscall.{proc_id}"),
        }
    }
}

// HELPER FUNCTIONS
// ================================================================================================

/// Builds a string from input vector to display push operation
fn display_push_vec<T: fmt::Display>(f: &mut fmt::Formatter<'_>, values: &[T]) -> fmt::Result {
    write!(f, "push")?;
    for elem in values {
        write!(f, ".{elem}")?;
    }
    Ok(())
}

// TESTS
// ================================================================================================

#[test]
fn test_instruction_display() {
    let instruction = format!("{}", Instruction::Assert);
    assert_eq!("assert", instruction);

    let instruction = format!("{}", Instruction::Add);
    assert_eq!("add", instruction);

    let instruction = format!("{}", Instruction::AddImm(Felt::new(5)));
    assert_eq!("add.5", instruction);

    let instruction = format!("{}", Instruction::ExpBitLength(32));
    assert_eq!("exp.u32", instruction);

    let instruction = format!(
        "{}",
        Instruction::PushFeltList(vec![Felt::new(3), Felt::new(4), Felt::new(8), Felt::new(9)])
    );
    assert_eq!("push.3.4.8.9", instruction);

    let hash = [7; 24];
    let proc_id = ProcedureId::from(hash);
    let instruction = format!("{}", Instruction::ExecImported(proc_id));
    assert_eq!("exec.0x070707070707070707070707070707070707070707070707", instruction);
}
