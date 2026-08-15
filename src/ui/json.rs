use crate::sys::SystemInfo;

fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

pub fn render_json(info: &SystemInfo) {
    let mut gpus_json = Vec::new();
    for g in &info.gpu {
        gpus_json.push(format!("\"{}\"", escape_json(g)));
    }

    let mut disks_json = Vec::new();
    for d in &info.disks {
        disks_json.push(format!(
            r#"    {{
      "mount": "{}",
      "total_bytes": {},
      "used_bytes": {},
      "free_bytes": {},
      "usage_percent": {:.2},
      "fs_type": "{}"
    }}"#,
            escape_json(&d.mount),
            d.total_bytes,
            d.used_bytes,
            d.free_bytes,
            d.usage_percent,
            escape_json(&d.fs_type)
        ));
    }

    let battery_json = if let Some(b) = &info.battery {
        format!(
            r#"{{
    "percentage": {},
    "is_charging": {},
    "state": "{}"
  }}"#,
            b.percentage,
            b.is_charging,
            escape_json(&b.state)
        )
    } else {
        "null".to_string()
    };

    let mut displays_json = Vec::new();
    for disp in &info.displays {
        displays_json.push(format!(
            r#"    {{
      "resolution": "{}",
      "refresh_rate": {}
    }}"#,
            escape_json(&disp.resolution),
            disp.refresh_rate
        ));
    }

    println!(
        r#"{{
  "nourfetch_version": "1.0.0",
  "username": "{}",
  "hostname": "{}",
  "os": {{
    "name": "{}",
    "version": "{}",
    "build": "{}",
    "arch": "{}",
    "kernel": "{}"
  }},
  "host": "{}",
  "uptime_seconds": {},
  "shell": "{}",
  "terminal": "{}",
  "wm_de": "{}",
  "packages": {{
    "count": {},
    "managers": "{}"
  }},
  "cpu": {{
    "model": "{}",
    "cores": {},
    "threads": {},
    "freq_mhz": {}
  }},
  "gpu": [{}],
  "memory": {{
    "total_bytes": {},
    "used_bytes": {},
    "free_bytes": {},
    "usage_percent": {:.2}
  }},
  "disks": [
{}
  ],
  "battery": {},
  "displays": [
{}
  ]
}}"#,
        escape_json(&info.username),
        escape_json(&info.hostname),
        escape_json(&info.os_name),
        escape_json(&info.os_version),
        escape_json(&info.os_build),
        escape_json(&info.os_arch),
        escape_json(&info.kernel),
        escape_json(&info.host_model),
        info.uptime_seconds,
        escape_json(&info.shell),
        escape_json(&info.terminal),
        escape_json(&info.wm_de),
        info.packages_count,
        escape_json(&info.package_managers),
        escape_json(&info.cpu.model),
        info.cpu.cores,
        info.cpu.threads,
        info.cpu.freq_mhz,
        gpus_json.join(", "),
        info.memory.total_bytes,
        info.memory.used_bytes,
        info.memory.free_bytes,
        info.memory.usage_percent,
        disks_json.join(",\n"),
        battery_json,
        displays_json.join(",\n")
    );
}
