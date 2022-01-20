pub mod usdc_token {
    use solana_program::declare_id;

    #[cfg(feature = "devnet")]
    declare_id!("8FRFC6MoGGkMFQwngccyu69VnYbzykGeez7ignHVAFSN");
    #[cfg(not(feature = "devnet"))]
    declare_id!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
}
