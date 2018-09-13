/// crypt.rs
/// 
/// Public Key Cypher implementation.
/// 

use math;

use ramp::Int;

const DEFAULT_EXP: u32 = 65537;         // standard
const DEFAULT_BLOCK_SIZE: u32 = 128;    // bytes

