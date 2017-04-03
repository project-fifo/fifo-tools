#[cfg(not(target_os = "solaris"))]
use std::process::Command;
use serde_json;
use serde_json::Value;
use rustc_serialize::json;
use rustc_serialize::Encodable;

#[cfg(target_os = "solaris")]
use std::process;
#[cfg(target_os = "solaris")]
use libc;
#[cfg(target_os = "solaris")]
use std::ffi::CString;
#[cfg(target_os = "solaris")]
use door;

#[derive(RustcEncodable)]
struct GenericReq {
    action: String
}

pub fn run_generic(action: String) -> Value {
    let action = GenericReq{action: action};
    run(action)
}

// run is a generic function that takes any
// encoable, this will make it easyer to allow
// for calling it with `RustcEncodable` structs
// from the outside
pub fn run<E: Encodable>(obj: E) -> Value {
    let str = json::encode(&obj).unwrap();
    return run_str(str);
}

#[cfg(not(target_os = "solaris"))]
pub fn run_str(str: String) -> Value {
    let executable = "/opt/local/lib/fifo-tools/fifo";
    let output = Command::new(executable)
        .arg(str)
        .output()
        .expect("failed to execute process");
    let o = String::from_utf8_lossy(&output.stdout);
    let result: Value = serde_json::from_str(&o).unwrap();
    return result;
}
#[cfg(target_os = "solaris")]
macro_rules! check_err {
    ( $e:expr ) => {
        match $e {
            -1 => Err(Error::last_os_error()),
            other => Ok(other)
        }
    }
}
#[cfg(target_os = "solaris")]
pub fn run_str(str: String) -> Value {
    #[allow(unused_mut)]
    let rsize: usize = 65536;
    let mut rbuf = Vec::with_capacity(rsize as usize);
    let door = CString::new("/var/tmp/._fifo").unwrap();
    let mut cmd = str.clone();
    rbuf.resize(rsize, 0);
    cmd.push(0 as char);
    let door_args = &mut door::door_arg{
        data_ptr: cmd.as_ptr(),
        data_size: cmd.len(),
        rbuf: rbuf.as_mut_ptr(),
        rsize: rsize,
        desc_num: 0 as u32,
        desc_ptr: 0 as *mut door::door_desc
    } as *mut door::door_arg;
    unsafe {

        let fd = libc::open(door.as_ptr(), libc::O_RDWR);
        if fd < 0 {
            println!("open failed with: {}", fd);
            process::exit(1);
        }
        //try!(
        //check_err!(
        let res = door::door_call(fd, door_args);
        if res < 0 {
            println!("door_Call failed with: {}", res);
            process::exit(1);
        }
        libc::close(fd);
    }
    rbuf.retain(|&c| c != 0);
    match rbuf.split_first().unwrap() {
        // 49 is the character 1
        (&49, rrest) => {
            let rstr = String::from_utf8_lossy(rrest);
            return serde_json::from_str(&rstr).unwrap();
        },
        (_, rrest) => {
            let rstr = String::from_utf8_lossy(rrest);
            println!("Call error: {}", rstr);
            process::exit(1);
        }
    };
}
