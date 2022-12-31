use std::io::Write;

pub type Buffer = std::io::Cursor<Vec<u8>>;


pub trait Stringify {
    fn stringify(&self) -> String;
}

impl Stringify for Buffer {
    fn stringify(&self) -> String {
        match std::str::from_utf8(self.get_ref()) {
            Ok(s) => format!("{s}"),
            Err(_) => format!("Not a UTF-8 string")
        }
    }
}

#[allow(dead_code)]
pub fn make_buffer(expected_value: &str) -> (Buffer, Buffer) {
    let b = expected_value.as_bytes();
    let l = b.len();

    let out_inner: Vec<u8> = Vec::with_capacity(l);
    let out = Buffer::new(out_inner);

    let exp_inner: Vec<u8> = Vec::with_capacity(l);
    let mut exp = Buffer::new(exp_inner);

    let _ = exp.write_all(b);

    (out, exp)
}

#[allow(dead_code)]
pub fn print_buffer(buffer: &Buffer) {

    match std::str::from_utf8(buffer.get_ref()) {
        Ok(s) => println!("{s}"),
        Err(_) => println!("Not a UTF-8 string")
    }
}