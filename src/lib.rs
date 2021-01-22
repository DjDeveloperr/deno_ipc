use interprocess::local_socket::LocalSocketStream;
use std::collections::HashMap;
use std::cell::RefCell;
use deno_core::plugin_api::Interface;
use deno_core::{ZeroCopyBuf, Op};
use std::str::FromStr;
use std::result::Result;
use std::io::prelude::*;

thread_local! {
    static SOCKETS: RefCell<HashMap<u32, Box<LocalSocketStream>>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("op_ipc_new", op_ipc_new);
    interface.register_op("op_ipc_close", op_ipc_close);
    interface.register_op("op_ipc_write_all", op_ipc_write_all);
    interface.register_op("op_ipc_read_string", op_ipc_read_string);
    interface.register_op("op_ipc_read_bytes", op_ipc_read_bytes);
}

fn op(res: &str) -> Op {
    Op::Sync(res.as_bytes().to_vec().into_boxed_slice())
}

fn err(res: &str) -> Op {
    op(&format!("ipc_err::{}", res))
}

fn has_id (id: u32) -> Result<bool, String> {
    SOCKETS.with(|map| {
        let hm = map.borrow_mut();
        if hm.contains_key(&id) {
            Ok(true)
        } else {
            Ok(false)
        }
    })
}

fn get_next_id() -> Result<u32, String> {
    let mut id: u32 = 0;
    while has_id(id).unwrap() {
        id += 1;
    }
    Ok(id)
}

fn op_ipc_new(
    _interface: &mut dyn Interface,
    _args: &mut [ZeroCopyBuf]
) -> Op {
    let id = get_next_id().unwrap();
    let path = std::str::from_utf8(_args.get(0).unwrap()).unwrap();

    SOCKETS.with(|mapref| {
        let mut map = mapref.borrow_mut();
        if map.contains_key(&id) {
            op("exists")
        } else {
            let sock = LocalSocketStream::connect(path);
            if sock.is_err() {
                err(&sock.unwrap_err().to_string())
            } else {
                let sock = sock.unwrap();
                map.insert(id, Box::new(sock));
                op(&id.to_string())
            }
        }
    })
}

fn op_ipc_close(
    _interface: &mut dyn Interface,
    _args: &mut [ZeroCopyBuf]
) -> Op {
    let id = u32::from_str(std::str::from_utf8(_args.get(0).unwrap()).unwrap()).unwrap();

    SOCKETS.with(|mapref| {
        let mut map = mapref.borrow_mut();
        if map.contains_key(&id) {
            map.remove(&id);
            op("done")
        } else {
            err("local socket connection not found")
        }
    })
}

fn op_ipc_write_all(
    _interface: &mut dyn Interface,
    _args: &mut [ZeroCopyBuf]
) -> Op {
    let id = u32::from_str(std::str::from_utf8(_args.get(0).unwrap()).unwrap()).unwrap();
    let data = _args.get(1).unwrap();

    SOCKETS.with(|mapref| {
        let mut map = mapref.borrow_mut();
        if let Some(sock) = map.get_mut(&id) {
            let res = sock.write_all(data);
            if res.is_err() {
                err(&res.unwrap_err().to_string())
            } else {
                op("done")
            }
        } else {
            err("local socket connection not found")
        }
    })
}

fn op_ipc_read_bytes(
    _interface: &mut dyn Interface,
    _args: &mut [ZeroCopyBuf]
) -> Op {
    let id = u32::from_str(std::str::from_utf8(_args.get(0).unwrap()).unwrap()).unwrap();
    let len = usize::from_str(std::str::from_utf8(_args.get(1).unwrap()).unwrap()).unwrap();

    SOCKETS.with(|mapref| {
        let mut map = mapref.borrow_mut();
        if let Some(sock) = map.get_mut(&id) {
            let mut slice: Vec<u8> = vec! { 1; len };
            let res = sock.read(slice.as_mut_slice());
            if res.is_err() {
                err(&res.unwrap_err().to_string())
            } else {
                let string = std::str::from_utf8(slice.as_slice()).unwrap();
                op(string)
            }
        } else {
            err("local socket connection not found")
        }
    })
}

fn op_ipc_read_string(
    _interface: &mut dyn Interface,
    _args: &mut [ZeroCopyBuf]
) -> Op {
    let id = u32::from_str(std::str::from_utf8(_args.get(0).unwrap()).unwrap()).unwrap();

    SOCKETS.with(|mapref| {
        let mut map = mapref.borrow_mut();
        if let Some(sock) = map.get_mut(&id) {
            let mut vec = Vec::<u8>::new();
            let mut left = true;
            let mut is_err: Option<String> = Option::from(None);
            while left {
                let mut slice = [0];
                let res = sock.read(&mut slice);
                if res.is_err() {
                    is_err = Option::from(res.unwrap_err().to_string());
                    left = false;
                } else if res.unwrap() == 0 {
                    left = false;
                } else {
                    vec.push(slice[0]);
                }
            }

            if is_err.is_some() && vec.len() == 0 {
                err(&is_err.unwrap().to_string())
            } else {
                let string = std::str::from_utf8(vec.as_slice()).unwrap();
                op(string)
            }
        } else {
            err("local socket connection not found")
        }
    })
}