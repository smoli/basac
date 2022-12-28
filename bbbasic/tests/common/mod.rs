use std::io::Write;

pub type Buffer = std::io::Cursor<Vec<u8>>;

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
