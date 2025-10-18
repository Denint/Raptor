use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::CounterUseCases;
use super::operations::decr_counter_use_case::DecrCounterInput;
use super::operations::incr_counter_use_case::IncrCounterInput;
use super::operations::reset_counter_use_case::ResetCounterInput;

pub struct CounterController {
    use_cases: CounterUseCases,
}

impl CounterController {
    pub fn new(use_cases: CounterUseCases) -> Self {
        Self { use_cases }
    }

    pub fn incr_counter_use_case(&self) -> &Arc<crate::application::use_cases::counter::operations::incr_counter_use_case::IncrCounterUseCase<InMemoryStorage>>{
        &self.use_cases.incr_counter
    }

    pub fn decr_counter_use_case(&self) -> &Arc<crate::application::use_cases::counter::operations::decr_counter_use_case::DecrCounterUseCase<InMemoryStorage>>{
        &self.use_cases.decr_counter
    }

    pub fn reset_counter_use_case(&self) -> &Arc<crate::application::use_cases::counter::operations::reset_counter_use_case::ResetCounterUseCase<InMemoryStorage>>{
        &self.use_cases.reset_counter
    }

    pub async fn incr_counter(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<i64, crate::domain::errors::DomainError> {
        let input = IncrCounterInput::new(key);
        self.incr_counter_use_case().execute(input).await
    }

    pub async fn decr_counter(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<i64, crate::domain::errors::DomainError> {
        let input = DecrCounterInput::new(key);
        self.decr_counter_use_case().execute(input).await
    }

    pub async fn reset_counter(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<Option<i64>, crate::domain::errors::DomainError> {
        let input = ResetCounterInput::new(key);
        self.reset_counter_use_case().execute(input).await
    }
}
