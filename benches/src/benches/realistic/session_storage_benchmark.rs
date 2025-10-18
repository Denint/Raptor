use criterion::{Criterion, criterion_group, criterion_main};
use raptor::{
    application::use_cases::{
        basic::operations::{
            get_key_use_case::{GetKeyInput, GetKeyUseCase},
            set_key_use_case::{SetKeyInput, SetKeyUseCase},
        },
        counter::operations::incr_counter_use_case::{IncrCounterInput, IncrCounterUseCase},
    },
    domain::value_objects::{key::Key, value::Value},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn session_storage_benchmark(c: &mut Criterion) {
    let storage = Arc::new(InMemoryStorage::new(Arc::new(
        raptor::infrastructure::config::Config::from_env().unwrap(),
    )));
    let set_use_case = Arc::new(SetKeyUseCase::new(Arc::clone(&storage)));
    let get_use_case = Arc::new(GetKeyUseCase::new(Arc::clone(&storage)));
    let incr_use_case = Arc::new(IncrCounterUseCase::new(Arc::clone(&storage)));

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("session_storage_pattern");

    group.bench_function("session_storage_pattern", |b| {
        b.iter(|| {
            rt.block_on(async {
                let session_id = format!("session_{}", fastrand::u64(..));

                let user_key = Key::new(format!("{}_user", session_id)).unwrap();
                let data_key = Key::new(format!("{}_data", session_id)).unwrap();
                let counter_key = Key::new(format!("{}_views", session_id)).unwrap();

                let set_user_input =
                    SetKeyInput::new(user_key, Value::String(b"user_data".to_vec()));
                set_use_case.execute(set_user_input).await.unwrap();

                let set_data_input =
                    SetKeyInput::new(data_key, Value::String(b"session_data".to_vec()));
                set_use_case.execute(set_data_input).await.unwrap();

                let incr_input = IncrCounterInput::new(counter_key);
                let _ = incr_use_case.execute(incr_input).await.unwrap();

                let get_user_input =
                    GetKeyInput::new(Key::new(format!("{}_user", session_id)).unwrap());
                let _ = get_use_case.execute(get_user_input).await.unwrap();

                let get_views_input =
                    GetKeyInput::new(Key::new(format!("{}_views", session_id)).unwrap());
                let _ = get_use_case.execute(get_views_input).await.unwrap();
            })
        })
    });

    group.finish();
}

criterion_group!(session_storage_benches, session_storage_benchmark);
criterion_main!(session_storage_benches);
