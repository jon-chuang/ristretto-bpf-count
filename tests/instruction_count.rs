// #![cfg(feature = "test-bpf")]

use {
    ec_math::{
        edwards::CompressedEdwardsY, field::FieldElement, id, instruction,
        processor::process_instruction, scalar::Scalar,
    },
    // curve25519_bpf_test::field::FieldElement,
    solana_program::pubkey::Pubkey,
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

const NUM_ITER: u64 = 100_000;
const NUM_ITER_SMALL_OP: u64 = NUM_ITER * 1000;
const USE_JIT: bool = false;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

#[tokio::test]
async fn test_field_add() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(350_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::field_add(
            FieldElement::minus_one(),
            FieldElement::minus_one(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER_SMALL_OP as u128
    );
}

#[tokio::test]
async fn test_field_mul() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let two = &FieldElement::one() + &FieldElement::one();
    let minus_one = FieldElement::minus_one();

    let mut transaction = Transaction::new_with_payer(
        &[instruction::field_mul(two, minus_one)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER_SMALL_OP as u128
    );
}

#[tokio::test]
async fn test_field_invsqrt() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::field_invsqrt(FieldElement::one())],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER_SMALL_OP as u128
    );
}

#[tokio::test]
async fn test_scalar_add() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let zero = Scalar::default();
    let one = Scalar::one();

    let mut transaction =
        Transaction::new_with_payer(&[instruction::scalar_add(zero, one)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER_SMALL_OP as u128
    );
}

#[tokio::test]
async fn test_scalar_mul() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let zero = Scalar::default();
    let one = Scalar::one();

    let mut transaction =
        Transaction::new_with_payer(&[instruction::scalar_mul(zero, one)], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER_SMALL_OP as u128
    );
}

#[tokio::test]
async fn test_edwards_decompress() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::edwards_decompress(
            CompressedEdwardsY::default(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER as u128
    );
}

#[tokio::test]
async fn test_edwards_add() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::edwards_add(
            CompressedEdwardsY::default(),
            CompressedEdwardsY::default(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER as u128
    );
}

#[tokio::test]
async fn test_edwards_mul() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[instruction::edwards_mul(
            CompressedEdwardsY::default(),
            Scalar::one(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER as u128
    );
}

#[tokio::test]
async fn test_edwards_multiscalar_mul() {
    let mut pc = ProgramTest::new("ec_math", id(), processor!(process_instruction));

    // Arbitrary number for now
    pc.set_bpf_compute_max_units(30_500_000 * NUM_ITER);
    pc.use_bpf_jit(USE_JIT);

    let (mut banks_client, payer, recent_blockhash) = pc.start().await;

    let scalars = [Scalar::one(); 8];
    let points = [CompressedEdwardsY::default(); 8];

    let mut transaction = Transaction::new_with_payer(
        &[instruction::edwards_multiscalar_mul(points, scalars)],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    let now = std::time::Instant::now();
    banks_client.process_transaction(transaction).await.unwrap();
    println!(
        "{:?} time_taken: {:?}ns / op",
        function!(),
        now.elapsed().as_nanos() / NUM_ITER as u128
    );
}
