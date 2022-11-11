#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Param<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type SetFlag<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	// pub enum Event<T: Config> {
	// 	setParamBiggerThan100 {
	// 		who: T::AccountId,
	// 		claim: T::Hash,
	// 	},
	// }
	pub enum Event<T: Config> {
		SetParam(u32),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			log::info!("on_initialize: {:?}", n);
			0
		}

		fn on_finalize(n: T::BlockNumber) {
			log::info!(target: "use-hooks", "------------ on_finalize, block number is {:?}", n);
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		ParamInvalid,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::weight(0)]
		fn set_param_bigger_than_100(orogin: OriginFor<T>, param: u32) -> DispatchResult {
			// 1. 判断调用者权限
			ensure_root(orogin)?;

			// 2. 业务逻辑
			// 2.1 将标志位设置为 true
			SetFlag::<T>::put(true);

			// 2.2. 如果参数 > 100，则写入到 storage param 中
			if param <= 100i32 {
				return Err(Error::<T>::ParamInvalid.into());
			}

			// 3. 发出事件
			Self::deposit_event(Event::SetParam(param));
			log::info!(target: "use-hooks", "set param bigger then 100");
			Ok(().into())
		}
	}
}