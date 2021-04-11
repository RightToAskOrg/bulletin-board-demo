//! Define the hash algorithm and result used in this board.
//! This is all boilerplate.

use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use serde::de::Visitor;
use std::fmt;
use sha2::{Sha256, Digest};
use std::hash::Hasher;
use merkletree::hash::Algorithm;
use std::fmt::{Display, Formatter, Debug};

/// Define the hash type used in the Merkle trees. This is a wrapper around the Sha256 hash
pub struct MerkleHash(Sha256);

// The implementations below are to enable MerkleHash to be used as a hash method.

impl MerkleHash {
    pub fn new() -> MerkleHash {
        MerkleHash(Sha256::new())
    }
}

impl Default for MerkleHash {
    fn default() -> MerkleHash {
        MerkleHash::new()
    }
}

impl Hasher for MerkleHash {
    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!() // not needed for Merkle tree.
    }

    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.update(msg)
    }
}

impl Algorithm<[u8; 32]> for MerkleHash {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let output = self.0.finalize_reset();
        <[u8; 32]>::from(output)
//        let mut h = [0u8; 32];
//        self.0.result(&mut h);
//        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}



/// # Hash result
/// This is really just a fixed length array of bytes, but this can be annoying to serialize to JSON as an array of numbers.
/// So the main purpose of this wrapper is to allow serialization as a hex string, like that used by
/// the program "sha256sum" or its ilk.
pub struct HashValue(pub [u8;32]);

impl Display for HashValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",&hex::encode(&self.0))
    }
}

impl Debug for HashValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",&hex::encode(&self.0))
    }
}


/// Serialize an array of bytes as a string of the hexadecimal representation, as used in the "sha256sum" program.
impl Serialize for HashValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

/// Serialize an array of bytes as a string of the hexadecimal representation, as used in the "sha256sum" program.
impl <'de> Deserialize<'de> for HashValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(HashValueVisitor)
    }
}


/// Utility to do the work of deserialization.
struct HashValueVisitor;
impl<'de> Visitor<'de> for HashValueVisitor {
    type Value = HashValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64 character hexadecimal string")
    }

    /// called when a hex string is encountered.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error, {
        if v.len()==64 {
            let mut res = [0;32];
            match hex::decode_to_slice(v,&mut res) {
                Ok(_) => Ok(HashValue(res)),
                Err(_) => Err(E::custom("invalid hex string"))
            }
        } else {
            Err(E::custom("hex string should be 64 characters"))
        }
    }
}

