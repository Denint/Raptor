use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::decr_counter_use_case::DecrCounterUseCase;
use super::operations::incr_counter_use_case::IncrCounterUseCase;
use super::operations::reset_counter_use_case::ResetCounterUseCase;

pub struct CounterUseCases {
    pub incr_counter: Arc<IncrCounterUseCase<InMemoryStorage>>,
    pub decr_counter: Arc<DecrCounterUseCase<InMemoryStorage>>,
    pub reset_counter: Arc<ResetCounterUseCase<InMemoryStorage>>,
}

impl CounterUseCases {
    pub fn new(
        incr_counter: Arc<IncrCounterUseCase<InMemoryStorage>>,
        decr_counter: Arc<DecrCounterUseCase<InMemoryStorage>>,
        reset_counter: Arc<ResetCounterUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            incr_counter,
            decr_counter,
            reset_counter,
        }
    }
}
