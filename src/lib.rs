extern crate hyper;
extern crate libc;

//use std::io::Write;
use std::io::Read;
use std::sync::Mutex;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Handler;

//use std::ffi::CString;
use std::ptr;

pub struct RustSrvHandler
{
    strs :  Mutex<Vec<String>>,
}

fn main()
{
    start(3000);
}

impl Handler for RustSrvHandler
{
    fn handle(& self, mut req: Request, res: Response)
    {
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
                    /*
                    for y in &(*x)
                    {
                        println!("{}", y);
                    }
                    println!("");
                    */
                }
                Err(_) => {}
            }
        }
        res.send(b"").unwrap();
    }
}


#[no_mangle]
pub extern fn start(port : u32)
{
    let h = RustSrvHandler { strs : Mutex::new(Vec::new())};
    let s = String::from("0.0.0.0:") + &(*port.to_string());
    Server::http(&(*s)).unwrap().handle(h).unwrap();
}

#[no_mangle]
pub extern fn get_string (h : RustSrvHandler) -> *const u8
{
    let mut x = h.strs.lock().unwrap();
    let st = x.pop();
    match st
    {
        None => { return ptr::null(); }
        Some(y) => { return y.as_ptr(); }
    }
}
