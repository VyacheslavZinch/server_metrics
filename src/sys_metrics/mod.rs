
pub mod sys_metrics {
    use std::fmt::{Debug, Display};
    use rocket::serde::{Serialize, json::Json};
    use std::time::{SystemTime, UNIX_EPOCH};
    use rocket::yansi::Paint;
    use sysinfo::{Components, Disks, Networks, System};

    #[derive(Debug, Serialize, Clone)]
    pub struct MemInfo{
        total_memory: u64,
        used_memory:  u64,
        total_swap:   u64,
        used_swap:    u64
    }
    #[derive(Debug, Serialize, Clone)]
    pub struct SysInfo {
        sys_name: String,
        kernel_version: String,
        sys_os_version: String,
        sys_host_name: String
    }
    #[derive(Debug, Serialize, Clone)]
    pub struct Process {
        pid: u32,
        name: String,
        cpu_usage: f32,
        lifetime: u64,
        virtual_memory: u64,
        read_bytes: u64,
        write_bytes: u64,
    }
    #[derive(Debug, Serialize, Clone)]
    pub struct Disk{
        name: String,
        available_space: u64,
        total_space: u64,
        file_system: String,
        mount_point: String,
    }
    #[derive(Debug, Serialize, Clone)]
    pub struct NetworkInterface{
        name: String,
        mac_addr: String,
        ip: String,
        prefix: u8,
        total_received: u64,
        total_transmitted: u64,

    }
    #[derive(Debug, Serialize, Clone)]
    pub struct Component{
        label: String,
        current_temperature: f32,
        max_temperature: f32,
    }


    pub trait OutputData<T>
        where
            T: Serialize + From<Self>,
            Self: Sized + Clone
    {
        fn json_data(&self) -> Json<T>{
            let self_clone = self.clone();
            Json(T::from(self.clone()))
        }
        fn new() -> T;
    }
    impl OutputData<MemInfo> for MemInfo {
        fn new() -> MemInfo {
            let mut sys = System::new_all();
            sys.refresh_all();

            MemInfo{
                total_memory: (sys.total_memory()/1024),
                used_memory:  (sys.used_memory()/1024),
                total_swap:   (sys.total_swap()/1024),
                used_swap:    (sys.used_swap()/1024)
            }
        }
    }
    impl OutputData<SysInfo> for SysInfo {
        fn new() -> SysInfo {
            let mut sys = System::new_all();
            sys.refresh_all();

            SysInfo {
                sys_name: System::name().unwrap(),
                kernel_version: System::kernel_version().unwrap(),
                sys_os_version: System::os_version().unwrap(),
                sys_host_name: System::host_name().unwrap()
            }
        }
    }
    impl Process {
        fn new(
            pid: u32, name: String, cpu_usage: f32, lifetime: u64,
            virtual_memory: u64, read_bytes: u64, write_bytes: u64) -> Process {
            Process {
                pid,
                name,
                cpu_usage,
                lifetime,
                virtual_memory,
                read_bytes,
                write_bytes,
            }
        }
        pub fn get_processes() -> Vec<Process> {
            let mut processes = Vec::<Process>::new();
            let mut sys = System::new_all();
            sys.refresh_all();

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            for (pid, process) in sys.processes() {
                let lifetime = now - process.start_time();

                let proc = Process::new(
                    pid.as_u32(),
                    process.name().to_str().unwrap().to_string(),
                    process.cpu_usage(),
                    lifetime,
                    process.virtual_memory()/1024,
                    process.disk_usage().read_bytes/1024,
                    process.disk_usage().written_bytes/1024
                );
                processes.push(proc);

            }
            processes
        }
        pub fn json_data(data: Vec<Process>) -> Json<Vec<Process>> {
            Json(data)
        }
    }
    impl Disk {
        fn new(
            name: String, available_space: u64,
            total_space: u64, file_system: String,
            mount_point: String) -> Disk
        {
            Disk{
                name,
                available_space,
                total_space,
                file_system,
                mount_point
            }

        }
        pub fn get_discs() -> Vec<Disk> {
            let mut disks = Vec::<Disk>::new();
            let mut sys = System::new_all();
            sys.refresh_all();
            let discs = Disks::new_with_refreshed_list();

            for _disk in &discs {
                let _disk = Disk::new(
                    _disk.name().to_str().unwrap().to_string(),
                    _disk.available_space()/1024/1024,
                    _disk.total_space()/1024/1024,
                    _disk.file_system().to_str().unwrap().to_string(),
                    _disk.mount_point().to_str().unwrap().to_string()
                );
                disks.push(_disk);
            }
            disks

        }
        pub fn json_data(data: Vec<Disk>) -> Json<Vec<Disk>> {
            Json(data)
        }
    }
    impl NetworkInterface{
        fn new(name: String, mac_addr: String,
               ip: String, prefix: u8, total_received: u64,
               total_transmitted: u64,) -> NetworkInterface
        {
            NetworkInterface{
                name,
                mac_addr,
                ip,
                prefix,
                total_received,
                total_transmitted
            }
        }
        pub fn get_network_interfaces() -> Vec<NetworkInterface> {
            let mut network_interfaces = Vec::<NetworkInterface>::new();
            let mut sys = System::new_all();
            sys.refresh_all();

            let networks = Networks::new_with_refreshed_list();
            for (interface_name, data) in &networks {
                let ip_data = data.ip_networks();
                if !ip_data.is_empty() {
                    let _interface = NetworkInterface::new(
                        interface_name.as_str().to_string(),
                        data.mac_address().to_string(),
                        ip_data[0].addr.to_string(),
                        ip_data[0].prefix,
                        data.total_received(),
                        data.total_transmitted()
                    );
                    network_interfaces.push(_interface);
                }
            }
            network_interfaces
        }

        pub fn json_data(data: Vec<NetworkInterface>) -> Json<Vec<NetworkInterface>> {
            Json(data)
        }
    }
    impl Component {
        fn new(label: String, current_temperature: f32, max_temperature: f32) -> Component {
            Component{
                label,
                current_temperature,
                max_temperature
            }
        }
        pub fn get_components() -> Vec<Component> {
            let mut all_components = Vec::<Component>::new();
            let mut sys = System::new_all();
            sys.refresh_all();

            let sys_components = Components::new_with_refreshed_list();
            for component in &sys_components {
                let comp = Component::new(component.label().to_string(), component.temperature(), component.max());
                all_components.push(comp);
            }
            all_components
        }

        pub fn json_data(data: Vec<Component>) -> Json<Vec<Component>> {Json(data)}
    }

}