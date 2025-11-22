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

pub fn decode(id: &str) -> Result<u64, String> {
    let mut result: u64 = 0;
    
    for (i, c) in id.chars().rev().enumerate() {
        // Find the index of the character in our charset (e.g., 'a' -> 10)
        let val = match CHARSET.iter().position(|&x| x == c as u8) {
            Some(v) => v as u64,
            None => return Err(format!("Invalid character: {}", c)),
        };

        // Ex: "1C" -> ('C' * 62^0) + ('1' * 62^1)
        let power = 62u64.pow(i as u32);
        result += val * power;
    }
    
    Ok(result)
}