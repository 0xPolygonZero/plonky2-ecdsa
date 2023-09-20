use alloc::vec::Vec;
use plonky2::util::serialization::{Buffer, IoResult, Write};
use plonky2_u32::serialization::{ReadU32, WriteU32};

use crate::gadgets::biguint::BigUintTarget;

pub trait WriteBigUintTarget {
    fn write_biguint_target(&mut self, biguint_target: BigUintTarget) -> IoResult<()>;
}

impl WriteBigUintTarget for Vec<u8> {
    fn write_biguint_target(&mut self, biguint_target: BigUintTarget) -> IoResult<()> {
        let num_limbs = biguint_target.num_limbs();
        let num_limbs_be = num_limbs.to_be_bytes();
        for byte in &num_limbs_be {
            self.write_u8(*byte)?
        }
        for limb in &biguint_target.limbs {
            self.write_target_u32(*limb)?;
        }
        Ok(())
    }
}

pub trait ReadBigUintTarget {
    fn read_biguint_target(&mut self) -> IoResult<BigUintTarget>;
}

impl ReadBigUintTarget for Buffer<'_> {
    fn read_biguint_target(&mut self) -> IoResult<BigUintTarget> {
        let mut num_limbs_be = [0_u8; 8];
        num_limbs_be.copy_from_slice(&self.bytes()[0..8]);

        let num_limbs = usize::from_be_bytes(num_limbs_be);
        let mut limbs = Vec::new();

        for _ in 0..num_limbs {
            let limb = self.read_target_u32()?;
            limbs.push(limb)
        }

        Ok(BigUintTarget { limbs })
    }
}
