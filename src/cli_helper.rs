use std::{
    io::{Write, stdout},
    time::Duration,
};

use tokio::{runtime::Runtime, task, time};

pub fn a<F, Fut, T>(task_fn: F) -> T
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(spinner(task_fn))
}

pub async fn spinner<F, Fut, T>(task_fn: F) -> T
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let spinner_handle = task::spawn(async {
        let icons = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇⠏"];
        let mut i = 0;
        loop {
            print!("\r{}", icons[i % icons.len()]);
            stdout().flush().unwrap();
            i += 1;
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    let res = task_fn().await;
    spinner_handle.abort();
    res
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{runtime::Runtime, time};

    use super::spinner;

    pub fn a<F, Fut, T>(task_fn: F) -> T
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let rt = Runtime::new().unwrap();
        rt.block_on(spinner(task_fn))
    }

    async fn fetch_data_from_server() -> String {
        time::sleep(Duration::from_secs(3)).await;
        "サーバーデータ".to_string()
    }

    #[test]
    fn aa() {
        let r = a(fetch_data_from_server);
        println!("{r}");
    }
}
