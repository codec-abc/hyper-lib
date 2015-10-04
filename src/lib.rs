extern crate hyper;
extern crate libc;

use std::io::Read;
use std::sync::Mutex;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Handler;
use libc::c_char;
use std::ffi::CStr;
use std::ptr;
use std::mem;
use std::sync::Arc;
use std::thread;

pub struct RustSrvHandler
{
    strs :  Arc<Mutex<Vec<String>>>,
}

impl Handler for RustSrvHandler
{
    fn handle(& self, mut req: Request, res: Response)
    {
        res.send(b"").unwrap();
        if req.method == hyper::method::Method::Post
        {
            let mut buffer = String::new();
            let result = req.read_to_string(&mut buffer);
            match result
            {
                Ok(_) =>
                {
                    let mut x = self.strs.lock().unwrap();
                    x.push(buffer);
                }
                Err(_) => {}
            }
        }
    }
}

#[no_mangle]
pub extern fn start(port : u32) -> *const Mutex<Vec<String>>
{
    let h = RustSrvHandler { strs : Arc::new(Mutex::new(Vec::new())) };
    let w = h.strs.clone();
    let s = String::from("0.0.0.0:") + &(*port.to_string());
    thread::spawn(move ||
        {
            Server::http(&(*s)).unwrap().handle(h).unwrap();
        }
    );
    return &(*w) ;
}

#[no_mangle]
pub extern fn get_string (h : *const Mutex<Vec<String>>) -> *mut c_char
{
    let mut x = unsafe { (*h).lock().unwrap() };
    let st = x.pop();
    match st
    {
        None =>
        {
            let y : *mut i8 = ptr::null_mut();
            return y;
        }
        Some(y) =>
        {
            let p = y.as_ptr();
            mem::forget(y);
            p as *mut _
        }
    }
}

#[no_mangle]
pub extern fn free_string(c : *mut c_char)
{
    let _ = unsafe { CStr::from_ptr(c) };
}
