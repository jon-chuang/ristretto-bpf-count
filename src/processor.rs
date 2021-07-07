//! Program state processor

use crate::edwards::EdwardsPoint;
use crate::traits::MultiscalarMul;

use {
    crate::instruction::ECInstruction,
    borsh::BorshDeserialize,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

const NUM_ITER: usize = 100_000;
const NUM_ITER_SMALL_OP: usize = NUM_ITER * 1000;

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = ECInstruction::try_from_slice(input).unwrap();
    match instruction {
        ECInstruction::FieldAdd { element1, element2 } => {
            msg!("Adding two field elements");
            for _ in 0..NUM_ITER_SMALL_OP {
                let _result = &element1 + &element2;
            }
            msg!("FieldAdd complete");
            Ok(())
        }
        ECInstruction::FieldMul { element1, element2 } => {
            msg!("Multiplying two field elements");
            for _ in 0..NUM_ITER_SMALL_OP {
                let _result = &element1 * &element2;
            }
            msg!("FieldMul complete");
            Ok(())
        }
        ECInstruction::FieldInvSqrt { element } => {
            msg!("Computing the inverse square root of an element");
            for _ in 0..NUM_ITER_SMALL_OP {
                let _result = element.invsqrt();
            }
            msg!("FieldInvSqrt complete");
            Ok(())
        }
        ECInstruction::ScalarAdd { scalar1, scalar2 } => {
            msg!("Adding two scalar elements");
            for _ in 0..NUM_ITER_SMALL_OP {
                let _result = scalar1 + scalar2;
            }
            msg!("ScalarAdd complete");
            Ok(())
        }
        ECInstruction::ScalarMul { scalar1, scalar2 } => {
            msg!("Multiplying two scalar elements");

            for _ in 0..NUM_ITER_SMALL_OP {
                let _result = scalar1 * scalar2;
            }
            msg!("ScalarMul complete");
            Ok(())
        }
        ECInstruction::EdwardsDecompress { element } => {
            msg!("Decompressing a compressed Edwards curve element");

            for _ in 0..NUM_ITER {
                let _result = element.decompress().unwrap();
            }
            msg!("EdwardsDecompress complete");
            Ok(())
        }
        ECInstruction::EdwardsAdd { element1, element2 } => {
            msg!("Adding two Edwards curve elements");

            for _ in 0..NUM_ITER {
                let element1_decompressed = element1.decompress().unwrap();
                let element2_decompressed = element2.decompress().unwrap();
                let _result = element1_decompressed + element2_decompressed;
            }
            msg!("EdwardsAdd complete");
            Ok(())
        }
        ECInstruction::EdwardsMul { element, scalar } => {
            msg!("Multiplying an Edwards curve element with a scalar");

            let mut element_decompressed = element.decompress().unwrap();
            for _ in 0..NUM_ITER {
                element_decompressed *= scalar;
            }
            msg!("EdwardsMul complete {:?}", element_decompressed);
            Ok(())
        }
        ECInstruction::EdwardsMultiScalarMul { elements, scalars } => {
            msg!("Multiplying an Edwards curve element with a scalar");

            for _ in 0..NUM_ITER {
                let elements_iter = elements.iter().map(|elem| elem.decompress().unwrap());
                let scalars_iter = scalars.iter();

                EdwardsPoint::multiscalar_mul(scalars_iter, elements_iter);
            }
            msg!("EdwardsMultiScalarMul complete");
            Ok(())
        }
    }
}
