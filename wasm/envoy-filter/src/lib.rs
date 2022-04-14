use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use getrandom::getrandom;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HelloWorld) });
}}

struct HelloWorld;

impl Context for HelloWorld {}

impl RootContext for HelloWorld {
    fn on_vm_start(&mut self, _: usize) -> bool {
        info!("Hello, World!");
        self.set_tick_period(Duration::from_secs(5));
        true
    }

    fn on_tick(&mut self) {
        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                let now: DateTime<Utc> = self.get_current_time().into();
                info!("It's {}, there is no lucky number.", now);

            } else {
                let now: DateTime<Utc> = Utc::now();
                let mut buf = [0u8; 1];
                getrandom(&mut buf).unwrap();
                info!("It's {}, your lucky number is {}.", now, buf[0]);
            }
        }
    }
}