pub use crate::application::use_cases::array::controller::*;
pub use crate::application::use_cases::basic::controller::*;
pub use crate::application::use_cases::counter::controller::*;
pub use crate::application::use_cases::hash::controller::*;
pub use crate::application::use_cases::list::controller::*;
pub use crate::application::use_cases::multi_key::controller::*;
pub use crate::application::use_cases::set::controller::*;
pub use crate::application::use_cases::single_key::*;
pub use crate::application::use_cases::sorted_set::controller::*;
pub use crate::application::use_cases::ttl::controller::*;

pub use crate::application::use_cases::array::builder::ArrayUseCases;
pub use crate::application::use_cases::basic::builder::BasicUseCases;
pub use crate::application::use_cases::counter::builder::CounterUseCases;
pub use crate::application::use_cases::hash::builder::HashUseCases;
pub use crate::application::use_cases::list::builder::ListUseCases;
pub use crate::application::use_cases::multi_key::builder::MultiKeyUseCases;
pub use crate::application::use_cases::set::builder::SetUseCases;
pub use crate::application::use_cases::sorted_set::builder::SortedSetUseCases;
pub use crate::application::use_cases::ttl::builder::TtlUseCases;

use crate::application::use_cases::array::operations::{
    array_append_use_case::ArrayAppendUseCase, array_get_use_case::ArrayGetUseCase,
    array_length_use_case::ArrayLengthUseCase, array_set_use_case::ArraySetUseCase,
    array_slice_use_case::ArraySliceUseCase, array_update_use_case::ArrayUpdateUseCase,
};
use crate::application::use_cases::basic::operations::delete_key_use_case::DeleteKeyUseCase;
use crate::application::use_cases::basic::operations::get_key_use_case::GetKeyUseCase;
use crate::application::use_cases::basic::operations::set_key_use_case::SetKeyUseCase;
use crate::application::use_cases::counter::operations::{
    decr_counter_use_case::DecrCounterUseCase, incr_counter_use_case::IncrCounterUseCase,
    reset_counter_use_case::ResetCounterUseCase,
};
use crate::application::use_cases::hash::operations::{
    hdel_use_case::HDelUseCase, hexists_use_case::HExistsUseCase, hget_use_case::HGetUseCase,
    hkeys_use_case::HKeysUseCase, hlen_use_case::HLenUseCase, hset_use_case::HSetUseCase,
    hvals_use_case::HValsUseCase,
};
use crate::application::use_cases::list::operations::{
    lpop_use_case::LPopUseCase, lpush_use_case::LPushUseCase, lrange_use_case::LRangeUseCase,
    rpop_use_case::RPopUseCase, rpush_use_case::RPushUseCase,
};
use crate::application::use_cases::multi_key::operations::{
    mdel_use_case::MDelUseCase, mget_use_case::MGetUseCase, mset_use_case::MSetUseCase,
};
use crate::application::use_cases::set::operations::{
    sadd_use_case::SAddUseCase, scard_use_case::SCardUseCase, sismember_use_case::SIsMemberUseCase,
    smembers_use_case::SMembersUseCase, srem_use_case::SRemUseCase,
};
use crate::application::use_cases::sorted_set::operations::{
    zadd_use_case::ZAddUseCase, zcard_use_case::ZCardUseCase, zrange_use_case::ZRangeUseCase,
    zrem_use_case::ZRemUseCase, zscore_use_case::ZScoreUseCase,
};
use crate::application::use_cases::ttl::operations::{
    expire_use_case::ExpireUseCase, persist_use_case::PersistUseCase,
    set_with_ttl_use_case::SetWithTtlUseCase, ttl_use_case::TtlUseCase,
};
use crate::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;

#[derive(Clone)]
pub struct Controllers {
    pub basic: Arc<BasicController>,
    pub ttl: Arc<TtlController>,
    pub array: Arc<ArrayController>,
    pub counter: Arc<CounterController>,
    pub hash: Arc<HashController>,
    pub list: Arc<ListController>,
    pub multi_key: Arc<MultiKeyController>,
    pub set: Arc<SetController>,
    pub single_key: Arc<SingleKeyController>,
    pub sorted_set: Arc<SortedSetController>,
}

impl Controllers {
    pub fn new(storage: &Arc<InMemoryStorage>) -> Self {
        let basic_use_cases = BasicUseCases::new(
            Arc::new(GetKeyUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(SetKeyUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(DeleteKeyUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let ttl_use_cases = TtlUseCases::new(
            Arc::new(SetWithTtlUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(TtlUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(PersistUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ExpireUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let array_use_cases = ArrayUseCases::new(
            Arc::new(ArraySetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ArrayGetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ArrayAppendUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ArraySliceUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ArrayUpdateUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ArrayLengthUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let counter_use_cases = CounterUseCases::new(
            Arc::new(IncrCounterUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(DecrCounterUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ResetCounterUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let hash_use_cases = HashUseCases::new(
            Arc::new(HSetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HGetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HDelUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HExistsUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HKeysUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HValsUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(HLenUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let list_use_cases = ListUseCases::new(
            Arc::new(LPushUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(RPushUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(LPopUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(RPopUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(LRangeUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let multi_key_use_cases = MultiKeyUseCases::new(
            Arc::new(MSetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(MGetUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(MDelUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let set_use_cases = SetUseCases::new(
            Arc::new(SAddUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(SMembersUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(SRemUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(SIsMemberUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(SCardUseCase::<InMemoryStorage>::new(storage.clone())),
        );
        let sorted_set_use_cases = SortedSetUseCases::new(
            Arc::new(ZAddUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ZRangeUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ZRemUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ZScoreUseCase::<InMemoryStorage>::new(storage.clone())),
            Arc::new(ZCardUseCase::<InMemoryStorage>::new(storage.clone())),
        );

        let basic = Arc::new(BasicController::new(basic_use_cases));
        let ttl = Arc::new(TtlController::new(ttl_use_cases));
        let array = Arc::new(ArrayController::new(array_use_cases));

        Self {
            basic: basic.clone(),
            ttl: ttl.clone(),
            array: array.clone(),
            counter: Arc::new(CounterController::new(counter_use_cases)),
            hash: Arc::new(HashController::new(hash_use_cases)),
            list: Arc::new(ListController::new(list_use_cases)),
            multi_key: Arc::new(MultiKeyController::new(multi_key_use_cases)),
            set: Arc::new(SetController::new(set_use_cases)),
            single_key: Arc::new(SingleKeyController::new(basic, ttl, array)),
            sorted_set: Arc::new(SortedSetController::new(sorted_set_use_cases)),
        }
    }
}
