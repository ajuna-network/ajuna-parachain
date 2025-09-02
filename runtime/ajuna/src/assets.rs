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
	AJUN, AccountId, AssetConversion, Assets, Balance, Balances, ExistentialDeposit, MILLI_AJUN,
	PoolAssets, Runtime, RuntimeEvent, RuntimeOrigin, TreasuryAccount, weights,
};
use frame_support::{
	PalletId, ord_parameter_types,
	pallet_prelude::{ConstU32, PalletInfoAccess},
	parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, EnsureOriginWithArg,
		fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
		tokens::imbalance::ResolveAssetTo,
	},
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use pallet_asset_conversion::{Ascending, Chain, WithFirstAsset};
use parachains_common::AssetIdForTrustBackedAssets;
use sp_runtime::{Permill, traits::AccountIdConversion};
use sp_std::vec;

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
	type Holder = ();
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
	type AssetId = NativeOrWithId<AssetIdForTrustBackedAssets>;
	type OnChargeAssetTransaction = pallet_asset_conversion_tx_payment::SwapAssetAdapter<
		Native,
		NativeAndAssets,
		AssetConversion,
		ResolveAssetTo<TreasuryAccount, NativeAndAssets>,
	>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetConversionTxHelper;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct AssetConversionTxHelper;

#[cfg(feature = "runtime-benchmarks")]
pub type AssetConversionAssetIdFor<T> = <T as pallet_asset_conversion_tx_payment::Config>::AssetId;

#[cfg(feature = "runtime-benchmarks")]
impl
	pallet_asset_conversion_tx_payment::BenchmarkHelperTrait<
		AccountId,
		AssetConversionAssetIdFor<Runtime>,
		AssetConversionAssetIdFor<Runtime>,
	> for AssetConversionTxHelper
{
	fn create_asset_id_parameter(
		_seed: u32,
	) -> (AssetConversionAssetIdFor<Runtime>, AssetConversionAssetIdFor<Runtime>) {
		todo!()
		// // Use a different parachain' foreign assets pallet so that the asset is indeed foreign.
		// let asset_id = Location::new(
		// 	1,
		// 	[
		// 		Junction::Parachain(3000),
		// 		Junction::PalletInstance(53),
		// 		Junction::GeneralIndex(seed.into()),
		// 	],
		// );
		// (asset_id.clone(), asset_id)
	}

	fn setup_balances_and_pool(_asset_id: AssetConversionAssetIdFor<Runtime>, _account: AccountId) {
		todo!()
		// use alloc::boxed::Box;
		// use frame_support::{assert_ok, traits::fungibles::Mutate};
		//
		// assert_ok!(ForeignAssets::force_create(
		// 	RuntimeOrigin::root(),
		// 	asset_id.clone(),
		// 	account.clone().into(), /* owner */
		// 	true,                   /* is_sufficient */
		// 	1,
		// ));
		//
		// let lp_provider = account.clone();
		// use frame_support::traits::Currency;
		// let _ = Balances::deposit_creating(&lp_provider, u64::MAX.into());
		// assert_ok!(ForeignAssets::mint_into(asset_id.clone(), &lp_provider, u64::MAX.into()));
		//
		// let token_native = Box::new(KsmLocation::get());
		// let token_second = Box::new(asset_id);
		//
		// assert_ok!(AssetConversion::create_pool(
		// 	RuntimeOrigin::signed(lp_provider.clone()),
		// 	token_native.clone(),
		// 	token_second.clone()
		// ));
		//
		// assert_ok!(AssetConversion::add_liquidity(
		// 	RuntimeOrigin::signed(lp_provider.clone()),
		// 	token_native,
		// 	token_second,
		// 	(u32::MAX / 8).into(), // 1 desired
		// 	u32::MAX.into(),       // 2 desired
		// 	1,                     // 1 min
		// 	1,                     // 2 min
		// 	lp_provider,
		// ));
	}
}

impl pallet_asset_registry::Config for Runtime {
	type ReserveAssetModifierOrigin = EnsureRoot<Self::AccountId>;
	type Assets = Assets;
	type WeightInfo = weights::pallet_asset_registry::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = AssetRegistryBenchmarkHelper;
}

pub type NativeAndAssets = UnionOf<
	Balances,
	Assets,
	NativeFromLeft,
	NativeOrWithId<AssetIdForTrustBackedAssets>,
	AccountId,
>;

pub type AscendingLocator =
	Ascending<AccountId, NativeOrWithId<AssetIdForTrustBackedAssets>, PoolIdToAccountId>;

pub type WithFirstAssetLocator = WithFirstAsset<
	Native,
	AccountId,
	NativeOrWithId<AssetIdForTrustBackedAssets>,
	PoolIdToAccountId,
>;

impl pallet_asset_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type HigherPrecisionBalance = sp_core::U256;
	type AssetKind = NativeOrWithId<AssetIdForTrustBackedAssets>;
	type Assets = NativeAndAssets;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = Chain<WithFirstAssetLocator, AscendingLocator>;
	type PoolAssetId = u32;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = ConstU128<0>; // Asset class deposit fees are sufficient to prevent spam
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type LPFee = ConstU32<3>; // 0.3% swap fee
	type PalletId = AssetConversionPalletId;
	type MaxSwapPathLength = ConstU32<3>;
	type MintMinLiquidity = ConstU128<100>;
	type WeightInfo = weights::pallet_asset_conversion::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub AssetsPalletIndex: u32 = <Assets as PalletInfoAccess>::index() as u32;
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
	// we charge no fee for liquidity withdrawal
	pub const LiquidityWithdrawalFee: Permill = Permill::from_perthousand(0);
}

ord_parameter_types! {
	pub const AssetConversionOrigin: sp_runtime::AccountId32 =
		AccountIdConversion::<sp_runtime::AccountId32>::into_account_truncating(&AssetConversionPalletId::get());
}

pub type PoolIdToAccountId = pallet_asset_conversion::AccountIdConverter<
	AssetConversionPalletId,
	(NativeOrWithId<AssetIdForTrustBackedAssets>, NativeOrWithId<AssetIdForTrustBackedAssets>),
>;

pub type PoolAssetsInstance = pallet_assets::Instance2;
impl pallet_assets::Config<PoolAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type RemoveItemsLimit = ConstU32<1000>;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin =
		AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, sp_runtime::AccountId32>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	// Deposits are zero because creation/admin is limited to Asset Conversion pallet.
	type AssetDeposit = ConstU128<0>;
	type AssetAccountDeposit = ConstU128<0>;
	type MetadataDepositBase = ConstU128<0>;
	type MetadataDepositPerByte = ConstU128<0>;
	type ApprovalDeposit = ExistentialDeposit;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = weights::pallet_assets::WeightInfo<Runtime>;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type Holder = ();
}
