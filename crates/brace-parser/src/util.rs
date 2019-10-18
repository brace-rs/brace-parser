#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_ascii_linebreak(ch: &char) -> bool {
    match *ch as u8 {
        b'\n' | b'\r' | b'\x0C' => true,
        _ => false,
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_ascii_indent(ch: &char) -> bool {
    match *ch as u8 {
        b' ' | b'\t' => true,
        _ => false,
    }
}
