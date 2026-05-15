    let config = ::server_config::get();
    if config.standalone && !config.telemetry_enabled {
        return Ok(());
    }
