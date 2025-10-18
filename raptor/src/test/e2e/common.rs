#[cfg(not(miri))]
use once_cell::sync::Lazy;
#[cfg(not(miri))]
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

#[cfg(not(miri))]
static BUILD: Lazy<()> = Lazy::new(|| {
    Command::new("cargo")
        .arg("build")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to build the application");
});

#[cfg(not(miri))]
pub struct TestApp {
    pub address: String,
    process: Child,
}

#[cfg(not(miri))]
impl TestApp {
    pub async fn spawn() -> Self {
        Lazy::force(&BUILD);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let process = Command::new("../target/debug/raptor")
            .env("PORT", port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start the application");

        let address = format!("http://127.0.0.1:{}", port);

        sleep(Duration::from_millis(100)).await;

        Self { address, process }
    }

    pub async fn spawn_with_env(env_vars: &[(&str, &str)]) -> Self {
        Lazy::force(&BUILD);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let mut command = Command::new("../target/debug/raptor");
        command
            .env("PORT", port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for (key, value) in env_vars {
            command.env(*key, *value);
        }

        let process = command.spawn().expect("Failed to start the application");

        let address = format!("http://127.0.0.1:{}", port);

        sleep(Duration::from_millis(100)).await;

        Self { address, process }
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.process.kill().expect("Failed to kill the app process");
    }
}
