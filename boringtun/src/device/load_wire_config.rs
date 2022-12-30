// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;

impl super::DeviceHandle {
    pub fn load_config(&mut self) {
        const ENV_WG_CONFIG_FILE: &str = "WG_CONFIG_FILE";
        match env::var(ENV_WG_CONFIG_FILE) {
            Ok(c) => {
                if c.is_empty() {
                    tracing::info!("{}: set to empty", ENV_WG_CONFIG_FILE)
                } else {
                    let cc = fs::read_to_string(c).expect("Failed to read WG_CONFIG_FILE content");
                    let namec = self.device.read().iface.name().unwrap();
                    thread::spawn(move || {
                        let mut socket =
                            UnixStream::connect(format!("/var/run/wireguard/{}.sock", namec))
                                .unwrap();
                        write!(socket, "{}\n\n", cc).unwrap();
                        let mut resp = String::new();
                        socket.read_to_string(&mut resp).unwrap();
                        resp = resp.trim().to_owned();
                        if resp != "errno=0" {
                            tracing::error!("error on config apply: {}", resp);
                        }
                    });
                }
            }
            Err(_) => tracing::info!("{}: is not set", ENV_WG_CONFIG_FILE),
        }
    }
}
