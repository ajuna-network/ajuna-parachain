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

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `DESKTOP-0F6V7QQ`, CPU: `Intel(R) Core(TM) i7-10875H CPU @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/ajuna-node
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=pallet_session
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/ajuna/src/weights/pallet_session.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:1 w:1)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `3763`
		// Minimum execution time: 24_100_000 picoseconds.
		Weight::from_parts(24_700_000, 0)
			.saturating_add(Weight::from_parts(0, 3763))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:0 w:1)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn purge_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3745`
		// Minimum execution time: 18_200_000 picoseconds.
		Weight::from_parts(18_500_000, 0)
			.saturating_add(Weight::from_parts(0, 3745))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
