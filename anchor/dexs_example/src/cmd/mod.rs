pub mod simulate_dflow;
pub mod simulate_jup;
pub mod simulate_lifi;
pub mod simulate_solfi;


pub use simulate_dflow::simulate_dflow;
pub use simulate_jup::simulate_jup;
pub use simulate_lifi::simulate_lifi;
pub use simulate_solfi::simulate_solfi;


#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Dex {
    SplTokenSwap,
    StableSwap,
    Whirlpool,
    MeteoraDynamicpool,
    RaydiumSwap,
    RaydiumStableSwap,
    RaydiumClmmSwap,
    AldrinExchangeV1,
    AldrinExchangeV2,
    LifinityV1,
    LifinityV2,
    RaydiumClmmSwapV2,
    FluxBeam,
    MeteoraDlmm,
    RaydiumCpmmSwap,
    OpenBookV2,
    WhirlpoolV2,
    Phoenix,
    ObricV2,
    SanctumAddLiq,
    SanctumRemoveLiq,
    SanctumNonWsolSwap,
    SanctumWsolSwap,
    PumpfunBuy,
    PumpfunSell,
    StabbleSwap,
    SanctumRouter,
    MeteoraVaultDeposit,
    MeteoraVaultWithdraw,
    Saros,
    MeteoraLst,
    Solfi,
    QualiaSwap,
    Zerofi,
    PumpfunammBuy,
    PumpfunammSell,
    Virtuals,
    VertigoBuy,
    VertigoSell,
    PerpetualsAddLiq,
    PerpetualsRemoveLiq,
    PerpetualsSwap,
    RaydiumLaunchpad,
    LetsBonkFun,
    Woofi,
    MeteoraDbc,
    MeteoraDlmmSwap2,
    MeteoraDAMMV2,
    Gavel,
    BoopfunBuy,
    BoopfunSell,
    MeteoraDbc2,
    GooseFX,
    Dooar,
    Numeraire,
    SaberDecimalWrapperDeposit,
    SaberDecimalWrapperWithdraw,
    SarosDlmm,
    OneDexSwap,
    Manifest,
    ByrealClmm,
    PancakeSwapV3Swap,
    PancakeSwapV3SwapV2,
    Tessera,
    SolRfq,
    PumpfunBuy2,
    PumpfunammBuy2,
}

