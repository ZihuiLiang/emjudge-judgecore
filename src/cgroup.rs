use crate::quantity::MemorySize;

extern crate libc;
extern "C" {
    fn cgroup_init() -> libc::c_int;
    fn cgroup_new_cgroup(name: *const libc::c_char) -> *mut libc::c_void;
    fn cgroup_add_controller(
        cgroup: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn cgroup_add_value_uint64(
        controller: *mut libc::c_void,
        name: *const libc::c_char,
        value: libc::c_ulonglong,
    ) -> libc::c_int;
    fn cgroup_create_cgroup(
        cgroup: *mut libc::c_void,
        ignore_ownership: libc::c_int,
    ) -> libc::c_int;
    fn cgroup_modify_cgroup(cgroup: *mut libc::c_void) -> libc::c_int;
    fn cgroup_get_cgroup(cgroup: *mut libc::c_void) -> libc::c_int;
    fn cgroup_get_controller(
        cgroup: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn cgroup_set_value_uint64(
        controller: *mut libc::c_void,
        name: *const libc::c_char,
        value: libc::c_ulonglong,
    ) -> libc::c_int;
    fn cgroup_get_value_uint64(
        controller: *mut libc::c_void,
        name: *const libc::c_char,
        value: *mut libc::c_ulonglong,
    ) -> libc::c_int;
    fn cgroup_get_value_string(
        controller: *mut libc::c_void,
        name: *const libc::c_char,
        value: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn cgroup_attach_task_pid(cgroup: *mut libc::c_void, pid: libc::pid_t) -> libc::c_int;
    fn cgroup_free(cgroup: *mut *mut libc::c_void);
    fn cgroup_delete_cgroup_ext(cgroup: *mut libc::c_void, flags: libc::c_int) -> libc::c_int;
    #[cfg(feature="cgroup_v2")]
    fn cgroup_setup_mode() -> libc::c_int;
}

pub struct Cgroup {
    cgroup_name: std::ffi::CString,
    mem_controller: *mut libc::c_void,
    cgroup: *mut libc::c_void,
    oom: u64,
    is_v2: bool,
}

impl Cgroup {
    pub fn new(cgroup_name: &str, memory_limit: MemorySize) -> Result<Self, String> {
        if check_admin_privilege() == false {
            return Err("check_admin_privilege() failed".to_string());
        }
        unsafe {
            if cgroup_init() != 0 {
                return Err("cgroup_init() failed".to_string());
            }
        }
        let cgroup_name = std::ffi::CString::new(cgroup_name).unwrap();
        let mut cgroup = unsafe { cgroup_new_cgroup(cgroup_name.as_ptr()) };
        let is_v2 = {
            #[cfg(feature="cgroup_v2")]
            unsafe { cgroup_setup_mode() == 3}
            #[cfg(not(feature="cgroup_v2"))]
            false
        };
        if cgroup.is_null() {
            return Err("cgroup_new_cgroup() failed".to_string());
        }
        let memory_string = std::ffi::CString::new("memory").unwrap();
        let mem_controller = unsafe { cgroup_add_controller(cgroup, memory_string.as_ptr()) };
        if mem_controller.is_null() {
            unsafe {
                cgroup_free(&mut cgroup);
            }
            return Err("cgroup_add_controller() failed".to_string());
        }

        let memory_limit_in_bytes_string = if is_v2 {
            std::ffi::CString::new("memory.max").unwrap()
        } else {
            std::ffi::CString::new("memory.limit_in_bytes").unwrap()
        };
        let ret = unsafe {
            cgroup_add_value_uint64(
                mem_controller,
                memory_limit_in_bytes_string.as_ptr(),
                memory_limit.as_bytes() as u64,
            )
        };
        if ret != 0 {
            unsafe {
                cgroup_free(&mut cgroup);
            }
            return Err("cgroup_set_value_uint64() failed".to_string());
        }

        let ret = unsafe { cgroup_create_cgroup(cgroup, 0) };
        if ret != 0 {
            unsafe {
                cgroup_free(&mut cgroup);
            }
            return Err("cgroup_create_cgroup() failed".to_string());
        }
        let mut result = Cgroup {
            cgroup_name: cgroup_name,
            mem_controller: mem_controller,
            cgroup: cgroup,
            oom: 0,
            is_v2: is_v2,
        };
        result.update_cgroup_and_controller()?;
        if result.update_oom().is_err() {
            return Err("update_oom() failed".to_string());
        }
        Ok(result)
    }

    pub fn new_tmp(memory_limit: MemorySize) -> Result<Self, String> {
        let name = format!(
            "emjudge-judgecore-cgroup-{}",
            uuid::Uuid::new_v4().to_string()
        );
        Self::new(name.as_str(), memory_limit)
    }

    pub fn update_cgroup_and_controller(&mut self) -> Result<(), String> {
        let mut cgroup = unsafe { cgroup_new_cgroup(self.cgroup_name.as_ptr()) };
        if cgroup.is_null() {
            return Err("cgroup_new_cgroup() failed".to_string());
        }
        if unsafe { cgroup_get_cgroup(cgroup) } != 0 {
            unsafe {
                cgroup_free(&mut cgroup);
            }
            return Err("cgroup_get_cgroup() failed".to_string());
        }
        let memory_string = std::ffi::CString::new("memory").unwrap();
        let mem_controller = unsafe { cgroup_get_controller(cgroup, memory_string.as_ptr()) };
        if mem_controller.is_null() {
            unsafe {
                cgroup_free(&mut cgroup);
            }
            return Err("cgroup_add_controller() failed".to_string());
        }
        unsafe {
            cgroup_free(&mut self.cgroup);
        }
        self.cgroup = cgroup;
        self.mem_controller = mem_controller;
        Ok(())
    }

    pub fn update_oom(&mut self) -> Result<(), String> {
        let mut value: *mut libc::c_char = std::ptr::null_mut();
        if self.is_v2 {
            let memory_oom_control_string = std::ffi::CString::new("memory.events").unwrap();
            let ret = unsafe {
                cgroup_get_value_string(
                    self.mem_controller,
                    memory_oom_control_string.as_ptr(),
                    &mut value,
                )
            };
            if ret != 0 {
                return Err("cgroup_get_value_string() failed".to_string());
            }
            let string_value = unsafe { std::ffi::CString::from_raw(value) }
                .into_string()
                .unwrap();
            let value_vec = string_value.split_whitespace().collect::<Vec<&str>>();
            for i in 0..value_vec.len() {
                if value_vec[i] == "oom" {
                    self.oom = value_vec[i + 1].parse::<u64>().unwrap();
                    break;
                }
            }
        } else {
            let memory_oom_control_string = std::ffi::CString::new("memory.oom_control").unwrap();
            let ret = unsafe {
                cgroup_get_value_string(
                    self.mem_controller,
                    memory_oom_control_string.as_ptr(),
                    &mut value,
                )
            };
            if ret != 0 {
                return Err("cgroup_get_value_string() failed".to_string());
            }
            let string_value = unsafe { std::ffi::CString::from_raw(value) }
                .into_string()
                .unwrap();
            let value_vec = string_value.split_whitespace().collect::<Vec<&str>>();
            for i in 0..value_vec.len() {
                if value_vec[i] == "oom_kill" {
                    self.oom = value_vec[i + 1].parse::<u64>().unwrap();
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn update_cgroup_and_controller_and_check_oom(&mut self) -> Result<bool, String> {
        let last_oom = self.oom;
        self.update_cgroup_and_controller()?;
        self.update_oom()?;
        Ok(last_oom != self.oom)
    }

    pub fn get_max_usage_in_bytes(&mut self) -> Result<u64, String> {
        let mut value: u64 = 0;
        let memory_max_usage_in_bytes_string = if self.is_v2 {
            std::ffi::CString::new("memory.peak").unwrap()
        } else {
            std::ffi::CString::new("memory.max_usage_in_bytes").unwrap()
        };
        let ret = unsafe {
            cgroup_get_value_uint64(
                self.mem_controller,
                memory_max_usage_in_bytes_string.as_ptr(),
                &mut value,
            )
        };
        if ret != 0 {
            return Err("cgroup_get_value_uint64() failed".to_string());
        }
        Ok(value)
    }

    pub fn reset_max_usage_in_bytes(&mut self) -> Result<(), String> {
        let memory_max_usage_in_bytes_string = if self.is_v2 {
            std::ffi::CString::new("memory.peak").unwrap()
        } else {
            std::ffi::CString::new("memory.max_usage_in_bytes").unwrap()
        };
        let ret = unsafe {
            cgroup_set_value_uint64(
                self.mem_controller,
                memory_max_usage_in_bytes_string.as_ptr(),
                0,
            )
        };
        if ret != 0 {
            return Err("cgroup_set_value_uint64() failed".to_string());
        }
        if unsafe { cgroup_modify_cgroup(self.cgroup) } != 0 {
            return Err("cgroup_modify_cgroup() failed".to_string());
        }
        Ok(())
    }

    pub fn add_task(&mut self, pid: libc::pid_t) -> Result<(), String> {
        let ret = unsafe { cgroup_attach_task_pid(self.cgroup, pid) };
        if ret != 0 {
            return Err("cgroup_attach_task_pid() failed".to_string());
        }
        Ok(())
    }
}

impl Drop for Cgroup {
    fn drop(&mut self) {
        unsafe {
            cgroup_delete_cgroup_ext(self.cgroup, 0);
        }
        unsafe {
            cgroup_free(&mut self.cgroup);
        }
    }
}

fn check_admin_privilege() -> bool {
    users::get_current_uid() == 0
}
