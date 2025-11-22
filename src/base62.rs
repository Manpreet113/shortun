const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn encode(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut chars = Vec::new();

    while num > 0 {
        let remainder = ( num % 62) as usize;
        chars.push(CHARSET[remainder] as char);
        num /= 62;
    }

    chars.into_iter().rev().collect()
}

