// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	weights, AccountId, Assets, Balance, Balances, Runtime, RuntimeEvent, RuntimeOrigin, AJUN,
	MILLI_AJUN,
};
use frame_support::{
	pallet_prelude::ConstU32,
	traits::{ConstU128, EnsureOriginWithArg},
};
use frame_system::EnsureRoot;
use parachains_common::AssetIdForTrustBackedAssets;

pub type AssetBalance = Balance;

/// always denies creation of assets
pub struct NoAssetCreators;
impl EnsureOriginWithArg<RuntimeOrigin, AssetIdForTrustBackedAssets> for NoAssetCreators {
	type Success = AccountId;

	fn try_origin(
		o: RuntimeOrigin,
		_a: &AssetIdForTrustBackedAssets,
	) -> Result<Self::Success, RuntimeOrigin> {
		Err(o)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(_a: &AssetIdForTrustBackedAssets) -> Result<RuntimeOrigin, ()> {
		Err(())
	}
}

pub type MainAssetsInstance = pallet_assets::Instance1;
impl pallet_assets::Config<MainAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = AssetBalance;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = AssetIdForTrustBackedAssets;
	type AssetIdParameter = parity_scale_codec::Compact<AssetIdForTrustBackedAssets>;
	type Currency = Balances;
	type CreateOrigin = NoAssetCreators; //assets can only be created by root
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ConstU128<{ AJUN }>;
	type AssetAccountDeposit = ConstU128<{ AJUN }>;
	type MetadataDepositBase = ConstU128<{ AJUN }>;
	type MetadataDepositPerByte = ConstU128<{ 10 * MILLI_AJUN }>;
	type ApprovalDeposit = ConstU128<{ 10 * MILLI_AJUN }>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = weights::pallet_assets::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

#[cfg(feature = "runtime-benchmarks")]
pub struct AssetRegistryBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_asset_registry::BenchmarkHelper<AssetIdForTrustBackedAssets>
	for AssetRegistryBenchmarkHelper
{
	fn get_registered_asset() -> AssetIdForTrustBackedAssets {
		use sp_runtime::traits::StaticLookup;

		let root = frame_system::RawOrigin::Root.into();
		let asset_id = 1;
		let caller = frame_benchmarking::whitelisted_caller();
		let caller_lookup = <Runtime as frame_system::Config>::Lookup::unlookup(caller);
		Assets::force_create(root, asset_id.into(), caller_lookup, true, 1)
			.expect("Should have been able to force create asset");
		asset_id
	}
}

impl pallet_asset_conversion_tx_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Fungibles = LocalAndForeignAssets;
	type OnChargeAssetTransaction =
		impls::tx_payment::SwapCreditAdapter<DotLocationV3, AssetConversion>;
}

impl pallet_asset_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ReserveAssetModifierOrigin = EnsureRoot<Self::AccountId>;
	type Assets = Assets;
	type WeightInfo = weights::pallet_asset_registry::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRegistryBenchmarkHelper;
}

pub type PoolAssetsInstance = pallet_assets::Instance3;
impl pallet_assets::Config<PoolAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = NeverEnsureOrigin<AccountId>;
	type ForceOrigin = AssetsForceOrigin;
	type AssetDeposit = ConstU128<0>;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type ApprovalDeposit = ExistentialDeposit;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = weights::pallet_assets_pool::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

/// Union fungibles implementation for `Assets`` and `ForeignAssets`.
pub type LocalAndForeignAssets = fungibles::UnionOf<
	Assets,
	ForeignAssets,
	LocalFromLeft<
		AssetIdForTrustBackedAssetsConvert<TrustBackedAssetsPalletLocationV3, xcm::v3::Location>,
		AssetIdForTrustBackedAssets,
		xcm::v3::Location,
	>,
	xcm::v3::Location,
	AccountId,
>;

/// Union fungibles implementation for [`LocalAndForeignAssets`] and `Balances`.
pub type NativeAndAssets = fungible::UnionOf<
	Balances,
	LocalAndForeignAssets,
	TargetFromLeft<DotLocationV3, xcm::v3::Location>,
	xcm::v3::Location,
	AccountId,
>;

parameter_types! {
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
	pub const LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
	// Storage deposit for pool setup within asset conversion pallet
	// and pool's lp token creation within assets pallet.
	pub const PoolSetupFee: Balance = system_para_deposit(1, 4) + AssetDeposit::get();
}

pub type PoolIdToAccountId = pallet_asset_conversion::AccountIdConverter<
	AssetConversionPalletId,
	(xcm::v3::Location, xcm::v3::Location),
>;

impl pallet_asset_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type HigherPrecisionBalance = sp_core::U256;
	type AssetKind = xcm::v3::Location;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = pallet_asset_conversion::WithFirstAsset<
		DotLocationV3,
		AccountId,
		Self::AssetKind,
		PoolIdToAccountId,
	>;
	type PoolAssetId = u32;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = PoolSetupFee;
	type PoolSetupFeeAsset = DotLocationV3;
	type PoolSetupFeeTarget = ResolveAssetTo<xcm_config::RelayTreasuryPalletAccount, Self::Assets>;
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type LPFee = ConstU32<3>;
	type PalletId = AssetConversionPalletId;
	type MaxSwapPathLength = ConstU32<3>;
	type MintMinLiquidity = ConstU128<100>;
	type WeightInfo = weights::pallet_asset_conversion::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = assets_common::benchmarks::AssetPairFactory<
		DotLocationV3,
		parachain_info::Pallet<Runtime>,
		xcm_config::TrustBackedAssetsPalletIndex,
		Self::AssetKind,
	>;
}
