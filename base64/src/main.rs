use std::io::stdin;

const B64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub const ENCODE: &str = "1";
pub const DECODE: &str = "2";

pub const EXIT: &str = "3";

fn main() -> Result<(), String> {
    loop {
        help();
        let input = get_input();
        let input = input.as_str();

        let res = match input {
            ENCODE => b64_encode(get_input().as_bytes()),
            DECODE => {
                let decode_byte = b64_decode(get_input().as_str())
                    .map_err(|e| log_err(e))
                    .unwrap_or(vec![]);

                let res = String::from_utf8(decode_byte)
                    .map_err(|e| log_err(e))
                    .unwrap_or(String::new());
                if res.len() == 0 {
                    continue;
                }
                res
            },
            EXIT => return Ok(()),
            _ => continue,
        };

        println!("Decoded: \"{}\"", res);
    }
}

pub fn help() {
    println!("Вберите действие");
    println!("1: Для превода строки в base64");
    println!("2: Для превода строки из base64");
    println!("3: выход");
}
pub fn get_input<'a>() -> String {
    let handle = stdin();
    let mut input = String::new();

    handle
        .read_line(&mut input)
        .expect("Не удалось считать строку");
    input.trim().to_string()
}

/// Кодирование произвольного среза байтов в Base64-строку.
pub fn b64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);

    for chunk in input.chunks(3) {
        let bytes = match *chunk {
            [a, b, c] => [a, b, c, 0],
            [a, b] => [a, b, 0, 2], // 2 байта -> 1 «=»
            [a] => [a, 0, 0, 1],    // 1 байт  -> 2 «=»
            _ => unreachable!(),
        };

        let buf = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);

        // Выбираем 4 шестибитовых блока
        for i in (0..4).rev() {
            let six_bits = ((buf >> (i * 6)) & 0x3F) as usize;
            out.push(B64_TABLE[six_bits] as char);
        }

        // Добавляем паддинг, если нужно
        match bytes[3] {
            1 => {
                out.pop();
                out.pop();
                out.push('=');
                out.push('=');
            }
            2 => {
                out.pop();
                out.push('=');
            }
            _ => {}
        }
    }
    out
}

/// Декодирование Base64-строки обратно в вектор байтов.
pub fn b64_decode(s: &str) -> Result<Vec<u8>, &'static str> {
    // Проверяем кратность 4 и допустимые символы
    if s.len() % 4 != 0 {
        return Err("length not multiple of 4");
    }

    let mut bytes = Vec::with_capacity(s.len() / 4 * 3);

    let mut val = 0u32;
    let mut valb = -8i32;

    for &c in s.as_bytes() {
        let idx = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => {
                // паддинг: прекращаем чтение
                if c != b'=' {
                    return Err("invalid char");
                }
                break;
            }
            _ => return Err("invalid char"),
        } as u32;

        val = (val << 6) | idx;
        valb += 6;
        if valb >= 0 {
            bytes.push((val >> valb) as u8);
            valb -= 8;
        }
    }
    Ok(bytes)
}

fn log_err<T: ToString>(err: T) -> String {
    let e_str = format!("Error decoding base64: {}", err.to_string());
    println!("{}", e_str);
    e_str
}
