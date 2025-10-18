use crate::{
    application::use_cases::counter::operations::{
        decr_counter_use_case::{DecrCounterInput, DecrCounterUseCase},
        incr_counter_use_case::{IncrCounterInput, IncrCounterUseCase},
        reset_counter_use_case::{ResetCounterInput, ResetCounterUseCase},
    },
    domain::{errors::DomainError, value_objects::key::Key},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

pub struct CounterController {
    incr_counter_use_case: Arc<IncrCounterUseCase<InMemoryStorage>>,
    decr_counter_use_case: Arc<DecrCounterUseCase<InMemoryStorage>>,
    reset_counter_use_case: Arc<ResetCounterUseCase<InMemoryStorage>>,
}

impl CounterController {
    pub fn new(
        incr_counter_use_case: Arc<IncrCounterUseCase<InMemoryStorage>>,
        decr_counter_use_case: Arc<DecrCounterUseCase<InMemoryStorage>>,
        reset_counter_use_case: Arc<ResetCounterUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            incr_counter_use_case,
            decr_counter_use_case,
            reset_counter_use_case,
        }
    }

    pub async fn incr_counter(&self, key: Key) -> Result<i64, DomainError> {
        self.incr_counter_use_case
            .execute(IncrCounterInput::new(key))
            .await
    }

    pub async fn decr_counter(&self, key: Key) -> Result<i64, DomainError> {
        self.decr_counter_use_case
            .execute(DecrCounterInput::new(key))
            .await
    }

    pub async fn reset_counter(&self, key: Key) -> Result<Option<i64>, DomainError> {
        self.reset_counter_use_case
            .execute(ResetCounterInput::new(key))
            .await
    }
}
