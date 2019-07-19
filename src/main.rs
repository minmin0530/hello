#![feature(allocator_api)]
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

use std::alloc::{Alloc, System, Layout};
use std::mem;
use std::slice;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET /index.js") {
        let mut file = File::open("index.js").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET /libweb.min.wasm") {
        let mut file = File::open("libweb.min.wasm").unwrap();

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n");

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.write_all(&buf).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Pixel {
  pub red:     u8,
  pub green:   u8,
  pub blue:    u8,
  pub opacity: u8,
}

impl Pixel {
  pub fn new() -> Pixel {
    Pixel {
      red:     255,
      green:   0,
      blue:    0,
      opacity: 0
    }
  }
}

fn mycos(mut x:f64) -> f64 {
  x = x - (x/3.14159265).trunc() * 3.14159265;

  if x < 0f64 {
    1.27323954 * x + 0.405284735 * x * x
  } else {
    1.27323954 * x - 0.405284735 * x * x
  }
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
  unsafe {
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
    mem::transmute::<std::ptr::NonNull<u8>, *mut u8 >( System.alloc(layout).unwrap() )
  }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
  unsafe  {
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
    System.dealloc(mem::transmute::< *mut u8, std::ptr::NonNull<u8> >( ptr ), layout);
  }
}

#[no_mangle]
pub fn fill(pointer: *mut u8, length: usize, time: f64) {
  let sl = unsafe { slice::from_raw_parts_mut(pointer, length * 4) };

  for i in 0..length*4 {
    let height = i / 4 / 500;
    let width  = i / 4 % 500;

    if i%4 == 3 {
      sl[i] = 255;
    } else if i%4 == 0 {
      let len = ((height*height + width*width) as f64).sqrt();
      let nb = time  + len / 12.0;
      let a = 128.0 + mycos(nb) * 128.0;
      sl[i] = a as u8;

    } else if i % 4 == 2 {
      let width = 500 - width;
      let len = ((height*height + width*width) as f64).sqrt();
      let nb = time  + len / 12.0;
      let a = 128.0 + mycos(nb) * 128.0;
      sl[i] = a as u8;
    }
  }
}