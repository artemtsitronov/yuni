pub fn frame(data: &[u8]) -> Vec<u8> {
    let mut out = (data.len() as u16).to_be_bytes().to_vec();
    out.extend_from_slice(data);
    out
}

pub fn unframe(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() < 2 {
        return None;
    }
    let len = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
    if bytes.len() - 2 < len {
        return None;
    }
    Some(bytes[2..2 + len].to_vec())
}
