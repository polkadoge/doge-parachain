pub use primitive_types::{H160, H256, U256};
use crate::script::Script;

/// Represents a bitcoin 32 bytes hash digest encoded in little-endian
#[derive(Encode, Decode, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub struct H256Le {
    content: [u8; 32],
}


/// https://github.com/dogecoin/dogecoin/blob/master/src/primitives/pureheader.h#L34
pub struct PureBlockHeader {
    pub version: i32,
    pub hash_pre_block: H256Le,
    pub hash_merkle_root: H256Le,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

/// https://github.com/dogecoin/dogecoin/blob/master/src/primitives/block.h
pub struct BlockHeader {
    pub pure_block_header: PureBlockHeader,
    // todo
}

pub struct Block {

}

pub struct Transaction {
    pub version: i32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub locktime: u32,
}

/// An outpoint - a combination of a transaction hash and an index n into its vout
pub struct OutPoint {
    pub hash: H256Le,
    pub n: u32,
}

/// https://github.com/polkadoge/dogecoin/blob/master/src/primitives/transaction.h#L62
/// An input of a transaction. It contains the location of the previous transaction's
/// output that it claims and a signature that matches the output's public key
pub struct TransactionInput {
    pub pre_vout: OutPoint,
    /// https://github.com/polkadoge/dogecoin/blob/master/src/script/script.h#L373
    pub script_sig: Vec<u8>,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

/// Dogecoin transaction output
pub struct TransactionOutput {
    pub value: i64,
    pub script_pub_key: Script,
}

/// Dogecoin Script OpCodes
/// https://github.com/dogecoin/dogecoin/blob/master/src/script/script.h#L44
#[derive(Copy, Clone)]
pub enum OpCode {
    // push value
    Op0 = 0x00,
    // OpFalse = Op0
    OpPushData1 = 0x4c,
    OpPushData2 = 0x4d,
    OpPushData4 = 0x4e,
    Op1Negate = 0x4f,
    OpReserved = 0x50,
    Op1 = 0x51,
    // OpTrue = Op1
    Op2 = 0x52,
    Op3 = 0x53,
    Op4 = 0x54,
    Op5 = 0x55,
    Op6 = 0x56,
    Op7 = 0x57,
    Op8 = 0x58,
    Op9 = 0x59,
    Op10 = 0x5a,
    Op11 = 0x5b,
    Op12 = 0x5c,
    Op13 = 0x5d,
    Op14 = 0x5e,
    Op15 = 0x5f,
    Op16 = 0x60,

    // control
    OpNop = 0x61,
    OpVer = 0x62,
    OpIf = 0x63,
    OpNotIf = 0x64,
    OpVerIf = 0x65,
    OpVerNotIf = 0x66,
    OpElse = 0x67,
    OpEndIf = 0x68,
    OpVerify = 0x69,
    OpReturn = 0x6a,

    // stack ops
    OpToaltStack = 0x6b,
    OpFromAltStack = 0x6c,
    Op2Drop = 0x6d,
    Op2Dup = 0x6e,
    Op3Dup = 0x6f,
    Op2Over = 0x70,
    Op2Rot = 0x71,
    Op2Swap = 0x72,
    OpIfdup = 0x73,
    OpDepth = 0x74,
    OpDrop = 0x75,
    OpDup = 0x76,
    OpNip = 0x77,
    OpOver = 0x78,
    OpPick = 0x79,
    OpRoll = 0x7a,
    OpRot = 0x7b,
    OpSwap = 0x7c,
    OpTuck = 0x7d,

    // splice ops
    OpCat = 0x7e,
    OpSubstr = 0x7f,
    OpLeft = 0x80,
    OpRight = 0x81,
    OpSize = 0x82,

    // bit logic
    OpInvert = 0x83,
    OpAnd = 0x84,
    OpOr = 0x85,
    OpXor = 0x86,
    OpEqual = 0x87,
    OpEqualVerify = 0x88,
    OpReserved1 = 0x89,
    OpReserved2 = 0x8a,

    // numeric
    Op1Add = 0x8b,
    Op1Sub = 0x8c,
    Op2Mul = 0x8d,
    Op2Div = 0x8e,
    OpNegate = 0x8f,
    OpAbs = 0x90,
    OpNot = 0x91,
    Op0NotEqual = 0x92,

    OpAdd = 0x93,
    OpSub = 0x94,
    OpMul = 0x95,
    OpDiv = 0x96,
    OpMod = 0x97,
    OpLshift = 0x98,
    OpRshift = 0x99,

    OpBooland = 0x9a,
    OpBoolor = 0x9b,
    OpNumEqual = 0x9c,
    OpNumEqualVerify = 0x9d,
    OpNumNotEqual = 0x9e,
    OpLessThan = 0x9f,
    OpGreaterThan = 0xa0,
    OpLessThanOrEqual = 0xa1,
    OpGreaterThanOrEqual = 0xa2,
    OpMin = 0xa3,
    OpMax = 0xa4,

    OpWithin = 0xa5,

    // crypto
    OpRipemd160 = 0xa6,
    OpSha1 = 0xa7,
    OpSha256 = 0xa8,
    OpHash160 = 0xa9,
    OpHash256 = 0xaa,
    OpCodeSeparator = 0xab,
    OpCheckSig = 0xac,
    OpCheckSigverify = 0xad,
    OpCheckMultisig = 0xae,
    OpCheckMultisigVerify = 0xaf,

    // expansion
    OpNop1 = 0xb0,
    OpCheckLocktimeVerify = 0xb1,
    OpCheckSequenceVerify = 0xb2,
    OpNop4 = 0xb3,
    OpNop5 = 0xb4,
    OpNop6 = 0xb5,
    OpNop7 = 0xb6,
    OpNop8 = 0xb7,
    OpNop9 = 0xb8,
    OpNop10 = 0xb9,

    OpInvalidOpcode,
}

