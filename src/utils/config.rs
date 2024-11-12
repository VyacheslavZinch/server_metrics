pub mod config {
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufReader;

    #[derive(Deserialize, Debug)]
    pub struct ProcInfo {
        cpu_usage: f32,
        mem_usage: i32,
    }

    #[derive(Deserialize, Debug)]
    pub struct ConfigData {
        auth_type: String,
        fetch_data_interval: f32,
        warnings: ProcInfo,
        alerts: ProcInfo,
        exec_commands: bool,
        loglevel: String,
    }

    impl ConfigData {
        pub fn read_config() -> Result<ConfigData, Box<dyn std::error::Error>> {
            // Open the configuration file
            let file = File::open("config.json")?;
            let reader = BufReader::new(file);

            // Parse JSON into ConfigData
            let data: ConfigData = serde_json::from_reader(reader)?;

            println!("{:?}", data);
            Ok(data)
        }
    }
}
