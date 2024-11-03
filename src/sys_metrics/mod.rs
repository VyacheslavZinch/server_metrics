
pub mod sys_metrics {
    use rocket::serde::{Serialize, json::Json};
    use sysinfo::System;

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
        start_time: f32,
        virtual_memory: u64,
        read_bytes: u64,
        write_bytes: u64,
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
        pub fn new(
            pid: u32, name: String, cpu_usage: f32, start_time: f32,
            virtual_memory: u64, read_bytes: u64, write_bytes: u64) -> Process {
            Process {
                pid,
                name,
                cpu_usage,
                start_time,
                virtual_memory,
                read_bytes,
                write_bytes,
            }
        }
        pub fn get_processes() -> Vec<Process> {
            let mut processes = Vec::<Process>::new();
            let mut sys = System::new_all();
            sys.refresh_all();

            for (pid, process) in sys.processes() {
                let proc = Process::new(
                    pid.as_u32(),
                    process.name().to_str().unwrap().to_string(),
                    process.cpu_usage(),
                    process.start_time() as f32 / 60.0,
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

}