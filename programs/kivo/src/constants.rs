use anchor_lang::prelude::*;

#[constant]
pub const USER: &[u8] = b"user";
#[constant]
pub const OUTGOING: &[u8] = b"outgoing_tx";
#[constant] 
pub const INCOMING: &[u8] = b"incoming_tx";
#[constant]
pub const UNWRAP: &[u8] = b"unwrap";