use rand::Rng;

pub fn generate_random_chars<const N: usize>() -> [u8; N] {
    let mut rng = rand::thread_rng();
    let charset = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let charset_len = charset.len();

    // Create an array of random u8 characters
    let mut data = [0u8; N];
    for i in 0..N {
        let idx = rng.gen_range(0..charset_len);
        data[i] = charset[idx];
    }

    data
}

pub fn parse_u8_array_to_string<const N: usize>(data: [u8; N]) -> String {
    data.iter().map(|&byte| byte as char).collect()
}

pub fn parse_u8_vec_to_string(data: &Vec<u8>) -> String {
    data.iter().map(|&byte| byte as char).collect()
}
