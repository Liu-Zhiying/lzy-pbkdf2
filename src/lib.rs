use hmac_sha512::*;
use sha1::Sha1;
use sha256::*;
//convert byte array to base16 string.
//When i develop this library,i do not know the hex library.
//src: any type valua can convert to byte array reference,is the source.
//dest: the String converted.
//is_upprcase: if this is true,the character will be uppercase.
//the function will clean the data of the string "dest" before convert.
//return true for success and false for failure.
//转换字节数组到base16字符串
//编写该库时不知道hex库
//src: 可转换为字节数组引用的任意类型，原始值
//dest: 转换后的字符串
//is_upprcase: 为真时字母为大写
//该函数会在转换前清除dest的数据
//返回真成功，假失败
pub fn raw_to_base16_string<T: std::convert::AsRef<[u8]>>(
    dest: &mut String,
    src: T,
    is_uppercase: bool,
) -> bool {
    let src: &[u8] = src.as_ref();
    dest.clear();
    for i in 0..src.len() * 2 {
        let mut b: u8;
        if let Some(tb) = src.get(i / 2) {
            b = *tb;
        } else {
            return false;
        }
        if i % 2 != 0 {
            b &= 0xf
        } else {
            b &= 0xf0;
            b >>= 4;
        }
        dest.push(match b {
            0x0 => '0',
            0x1 => '1',
            0x2 => '2',
            0x3 => '3',
            0x4 => '4',
            0x5 => '5',
            0x6 => '6',
            0x7 => '7',
            0x8 => '8',
            0x9 => '9',
            0xa => 'a',
            0xb => 'b',
            0xc => 'c',
            0xd => 'd',
            0xe => 'e',
            0xf => 'f',
            _ => {
                return false;
            }
        });
    }
    if is_uppercase {
        let temp: String = dest.to_ascii_uppercase();
        *dest = temp;
    }
    return true;
}
//convert base16 string to byte array.
//When i develop this library,i do not know the hex library.
//src: String type,is the source.
//dest: the vector of byte converted.
//the function will clean the data of the vector "dest" before convert.
//the function copy the source and convert the source characters is lowercase.
//return true for success and false for failure.
//转换base16字符串到字节数组
//编写该库时不知道hex库
//src: String字符串，原始值
//dest: 转换后的变长字节数字
//该函数会在转换前清除dest的数据
//该函数会拷贝原始值，并把副本字母转换为小写
//返回真成功，假失败
pub fn base16_string_to_raw(dest: &mut Vec<u8>, src: &String) -> bool {
    if !src.is_ascii() || src.len() % 2 != 0 {
        return false;
    }
    dest.clear();
    let src_copy = src.to_ascii_lowercase();
    for i in 0..src_copy.len() / 2 {
        let mut b = 0;
        if let Some(c) = src_copy.chars().nth(i * 2) {
            b |= match c {
                '0' => 0x00,
                '1' => 0x10,
                '2' => 0x20,
                '3' => 0x30,
                '4' => 0x40,
                '5' => 0x50,
                '6' => 0x60,
                '7' => 0x70,
                '8' => 0x80,
                '9' => 0x90,
                'a' => 0xa0,
                'b' => 0xb0,
                'c' => 0xc0,
                'd' => 0xd0,
                'e' => 0xe0,
                'f' => 0xf0,
                _ => {
                    return false;
                }
            };
        } else {
            return false;
        }
        if let Some(c) = src_copy.chars().nth(i * 2 + 1) {
            b |= match c {
                '0' => 0x0,
                '1' => 0x1,
                '2' => 0x2,
                '3' => 0x3,
                '4' => 0x4,
                '5' => 0x5,
                '6' => 0x6,
                '7' => 0x7,
                '8' => 0x8,
                '9' => 0x9,
                'a' => 0xa,
                'b' => 0xb,
                'c' => 0xc,
                'd' => 0xd,
                'e' => 0xe,
                'f' => 0xf,
                _ => {
                    return false;
                }
            };
        } else {
            return false;
        }
        dest.push(b);
    }
    return true;
}
//this function is used only by this library.
//本函数仅内部使用
fn base16_string_to_raw_without_copy(dest: &mut Vec<u8>, src: &String) -> bool {
    if !src.is_ascii() || src.len() % 2 != 0 {
        return false;
    }
    dest.clear();
    for i in 0..src.len() / 2 {
        let mut b = 0;
        if let Some(c) = src.chars().nth(i * 2) {
            b |= match c {
                '0' => 0x00,
                '1' => 0x10,
                '2' => 0x20,
                '3' => 0x30,
                '4' => 0x40,
                '5' => 0x50,
                '6' => 0x60,
                '7' => 0x70,
                '8' => 0x80,
                '9' => 0x90,
                'a' => 0xa0,
                'b' => 0xb0,
                'c' => 0xc0,
                'd' => 0xd0,
                'e' => 0xe0,
                'f' => 0xf0,
                _ => {
                    return false;
                }
            };
        } else {
            return false;
        }
        if let Some(c) = src.chars().nth(i * 2 + 1) {
            b |= match c {
                '0' => 0x0,
                '1' => 0x1,
                '2' => 0x2,
                '3' => 0x3,
                '4' => 0x4,
                '5' => 0x5,
                '6' => 0x6,
                '7' => 0x7,
                '8' => 0x8,
                '9' => 0x9,
                'a' => 0xa,
                'b' => 0xb,
                'c' => 0xc,
                'd' => 0xd,
                'e' => 0xe,
                'f' => 0xf,
                _ => {
                    return false;
                }
            };
        } else {
            return false;
        }
        dest.push(b);
    }
    return true;
}
//hmac-sha1 function.
//key: the password value.
//message: the sault like pbkdf2
//return the u8 array with 20 byte length.
//if the function failed,the value of the array will all be zero.
//hmac-sha1函数
//key: 密码值
//message: 类似于pbkdf2中的盐
//返回20字节长的u8数组
//如果函数失败，数组全为0
pub fn hmac_sha1<T1: std::convert::AsRef<[u8]>, T2: std::convert::AsRef<[u8]>>(
    key: T1,
    message: T2,
) -> [u8; 20] {
    const IPAD: u8 = 0x36;
    const OPAD: u8 = 0x5c;
    let mut result: [u8; 20] = [0; 20];
    let mut key_vec1: Vec<u8> = Vec::new();
    let mut key_vec2: Vec<u8> = Vec::new();
    let key_ref: &[u8] = key.as_ref();
    let message_ref: &[u8] = message.as_ref();
    let mut sha1: Sha1 = Sha1::new();
    if key_ref.len() == 0 || message_ref.len() == 0 {
        return result;
    }
    for i in 0..key_ref.len() {
        key_vec1.push(key_ref[i]);
        key_vec2.push(key_ref[i]);
    }
    if key_vec1.len() < 64 {
        for _ in key_vec1.len()..64 {
            key_vec1.push(0);
            key_vec2.push(0);
        }
    }
    if key_vec1.len() > 64 {
        sha1.update(&key_vec1);
        base16_string_to_raw_without_copy(&mut key_vec1, &sha1.digest().to_string());
        base16_string_to_raw_without_copy(&mut key_vec2, &sha1.digest().to_string());
        sha1 = Sha1::new();
        for _ in key_vec1.len()..64 {
            key_vec1.push(0);
            key_vec2.push(0);
        }
    }
    for i in 0..key_vec1.len() {
        key_vec1[i] ^= IPAD;
        key_vec2[i] ^= OPAD;
    }
    for i in 0..message_ref.len() {
        key_vec1.push(message_ref[i]);
    }
    sha1.update(&key_vec1);
    base16_string_to_raw_without_copy(&mut key_vec1, &sha1.digest().to_string());
    sha1 = Sha1::new();
    for i in 0..key_vec1.len() {
        key_vec2.push(key_vec1[i]);
    }
    sha1.update(&key_vec2);
    base16_string_to_raw_without_copy(&mut key_vec2, &sha1.digest().to_string());
    for i in 0..result.len() {
        result[i] = key_vec2[i];
    }
    return result;
}
//hmac-sha256 function.
//key: the password value.
//message: the sault like pbkdf2
//return the u8 array with 32 byte length.
//if the function failed,the value of the array will all be zero.
//hmac-sha256函数
//key: 密码值
//message: 类似于pbkdf2中的盐
//返回32字节长的u8数组
//如果函数失败，数组全为0
pub fn hmac_sha256<T1: std::convert::AsRef<[u8]>, T2: std::convert::AsRef<[u8]>>(
    key: T1,
    message: T2,
) -> [u8; 32] {
    const IPAD: u8 = 0x36;
    const OPAD: u8 = 0x5c;
    let mut result: [u8; 32] = [0; 32];
    let mut key_vec1: Vec<u8> = Vec::new();
    let mut key_vec2: Vec<u8> = Vec::new();
    let key_ref: &[u8] = key.as_ref();
    let message_ref: &[u8] = message.as_ref();
    if key_ref.len() == 0 || message_ref.len() == 0 {
        return result;
    }
    for i in 0..key_ref.len() {
        key_vec1.push(key_ref[i]);
        key_vec2.push(key_ref[i]);
    }
    if key_vec1.len() < 64 {
        for _ in key_vec1.len()..64 {
            key_vec1.push(0);
            key_vec2.push(0);
        }
    }
    if key_vec1.len() > 64 {
        let hash_str: String = digest_bytes(&key_vec1);
        base16_string_to_raw_without_copy(&mut key_vec1, &hash_str);
        base16_string_to_raw_without_copy(&mut key_vec2, &hash_str);
        for _ in key_vec1.len()..64 {
            key_vec1.push(0);
            key_vec2.push(0);
        }
    }
    for i in 0..key_vec1.len() {
        key_vec1[i] ^= IPAD;
        key_vec2[i] ^= OPAD;
    }
    for i in 0..message_ref.len() {
        key_vec1.push(message_ref[i]);
    }
    let hash_str: String = digest_bytes(&key_vec1);
    base16_string_to_raw_without_copy(&mut key_vec1, &hash_str);
    for i in 0..key_vec1.len() {
        key_vec2.push(key_vec1[i]);
    }
    let hash_str: String = digest_bytes(&key_vec2);
    base16_string_to_raw_without_copy(&mut key_vec2, &hash_str);
    for i in 0..result.len() {
        result[i] = key_vec2[i];
    }
    return result;
}
//the hmac-sha512 function you can use jedisct1's hmac-sha512 libraty in https://github.com/jedisct1/rust-hmac-sha512
//Do not provide any hmac-sha512 fnuction in future.
//hmac-sha512函数可用jedisct1的库，网站https://github.com/jedisct1/rust-hmac-sha512
//本库不再提供hmac-sha512函数

//pbkdf2-sha1 function.
//key: the password value.
//sault: sault value.
//result_len: the length of result.
//conut: the count the the function use hmac-sha1 function.
//return a vector of u8 with the raw bytes.
//if the function failed,the length of the return vector is zero.
//pbkdf2-sha1 函数
//key: 密码值
//sault: 盐值
//result_len: 结果长
//conut: 使用hmac-sha1函数的次数
//若函数失败，返回的变长数组的长度将为0
pub fn pbkdf2_hmac_sha1<T1: std::convert::AsRef<[u8]>, T2: std::convert::AsRef<[u8]>>(
    key: T1,
    sault: T2,
    result_len: usize,
    count: usize,
) -> Vec<u8> {
    const HASH_LEN: usize = 20;
    let mut result: Vec<u8> = Vec::new();
    let key_ref = key.as_ref();
    let sualt_ref = sault.as_ref();
    if result_len > 0xffffffff * HASH_LEN
        || result_len == 0
        || count == 0
        || key_ref.len() == 0
        || sualt_ref.len() == 0
    {
        return result;
    }
    let mut l: u32 = (result_len / HASH_LEN) as u32;
    if result_len % HASH_LEN != 0 {
        l += 1;
    }
    let r = result_len - (l as usize - 1) * HASH_LEN;
    let mut _block_result: [u8; HASH_LEN] = [0; HASH_LEN];
    let mut _block_temp: [u8; HASH_LEN] = [0; HASH_LEN];
    let mut block_sault: Vec<u8> = Vec::new();
    let tsault = sault.as_ref();
    for i0 in 0..tsault.len() {
        block_sault.push(tsault[i0]);
    }
    for i1 in 1..l + 1 {
        if i1 != 1 {
            for _ in 0..4 {
                block_sault.pop();
            }
        }
        for i2 in 0..4 {
            unsafe {
                let pb: *const u8 = &i1 as *const u32 as *const u8;
                block_sault.push(*pb.offset(3 - i2));
            }
        }
        _block_temp = hmac_sha1(&key, block_sault.clone());
        _block_result = _block_temp;
        for _ in 0..count - 1 {
            _block_temp = hmac_sha1(&key, _block_temp.clone());
            for i3 in 0.._block_temp.len() {
                _block_result[i3] ^= _block_temp[i3];
            }
        }
        for i4 in 0.._block_temp.len() {
            result.push(_block_result[i4]);
        }
    }
    let mut ri: usize = HASH_LEN;
    while ri > r {
        result.pop();
        ri -= 1;
    }
    return result;
}
//pbkdf2-sha256 function.
//key: the password value.
//sault: sault value.
//result_len: the length of result.
//conut: the count the the function use hmac-sha1 function.
//return a vector of u8 with the raw bytes.
//if the function failed,the length of the return vector is zero.
//pbkdf2-sha1 函数
//key: 密码值
//sault: 盐值
//result_len: 结果长
//conut: 使用hmac-sha256函数的次数
//若函数失败，返回的变长数组的长度将为0
pub fn pbkdf2_hmac_sha256<T1: std::convert::AsRef<[u8]>, T2: std::convert::AsRef<[u8]>>(
    key: T1,
    sault: T2,
    result_len: usize,
    count: usize,
) -> Vec<u8> {
    const HASH_256_LEN: usize = 32;
    let mut result: Vec<u8> = Vec::new();
    let key_ref = key.as_ref();
    let sualt_ref = sault.as_ref();
    if result_len > 0xffffffff * HASH_256_LEN
        || result_len == 0
        || count == 0
        || key_ref.len() == 0
        || sualt_ref.len() == 0
    {
        return result;
    }
    let mut l: u32 = (result_len / HASH_256_LEN) as u32;
    if result_len % HASH_256_LEN != 0 {
        l += 1;
    }
    let r = result_len - (l as usize - 1) * HASH_256_LEN;
    let mut _block_result: [u8; HASH_256_LEN] = [0; HASH_256_LEN];
    let mut _block_temp: [u8; HASH_256_LEN] = [0; HASH_256_LEN];
    let mut block_sault: Vec<u8> = Vec::new();
    let tsault = sault.as_ref();
    for i0 in 0..tsault.len() {
        block_sault.push(tsault[i0]);
    }
    for i1 in 1..l + 1 {
        if i1 != 1 {
            for _ in 0..4 {
                block_sault.pop();
            }
        }
        for i2 in 0..4 {
            unsafe {
                let pb: *const u8 = &i1 as *const u32 as *const u8;
                block_sault.push(*pb.offset(3 - i2));
            }
        }
        _block_temp = hmac_sha256(&key, &block_sault);
        _block_result = _block_temp;
        for _ in 0..count - 1 {
            _block_temp = hmac_sha256(&key, &_block_temp);
            for i3 in 0.._block_temp.len() {
                _block_result[i3] ^= _block_temp[i3];
            }
        }
        for i4 in 0.._block_temp.len() {
            result.push(_block_result[i4]);
        }
    }
    let mut ri: usize = HASH_256_LEN;
    while ri > r {
        result.pop();
        ri -= 1;
    }
    return result;
}
//pbkdf2-sha512 function.
//key: the password value.
//sault: sault value.
//result_len: the length of result.
//conut: the count the the function use hmac-sha1 function.
//return a vector of u8 with the raw bytes.
//if the function failed,the length of the return vector is zero.
//pbkdf2-sha1 函数
//key: 密码值
//sault: 盐值
//result_len: 结果长
//conut: 使用hmac-sha512函数的次数
//若函数失败，返回的变长数组的长度将为0
pub fn pbkdf2_hmac_sha512<T1: std::convert::AsRef<[u8]>, T2: std::convert::AsRef<[u8]>>(
    key: T1,
    sault: T2,
    result_len: usize,
    count: usize,
) -> Vec<u8> {
    const HASH_LEN: usize = 64;
    let mut result: Vec<u8> = Vec::new();
    let key_ref = key.as_ref();
    let sualt_ref = sault.as_ref();
    if result_len > 0xffffffff * HASH_LEN
        || result_len == 0
        || count == 0
        || key_ref.len() == 0
        || sualt_ref.len() == 0
    {
        return result;
    }
    let mut l: u32 = (result_len / HASH_LEN) as u32;
    if result_len % HASH_LEN != 0 {
        l += 1;
    }
    let r = result_len - (l as usize - 1) * HASH_LEN;
    let mut _block_result: [u8; HASH_LEN] = [0; HASH_LEN];
    let mut _block_temp: [u8; HASH_LEN] = [0; HASH_LEN];
    let mut block_sault: Vec<u8> = Vec::new();
    let tsault = sault.as_ref();
    for i0 in 0..tsault.len() {
        block_sault.push(tsault[i0]);
    }
    for i1 in 1..l + 1 {
        if i1 != 1 {
            for _ in 0..4 {
                block_sault.pop();
            }
        }
        for i2 in 0..4 {
            unsafe {
                let pb: *const u8 = &i1 as *const u32 as *const u8;
                block_sault.push(*pb.offset(3 - i2));
            }
        }
        let block_sault_ref: &[u8] = block_sault.as_ref();
        _block_temp = HMAC::mac(block_sault_ref, &key);
        _block_result = _block_temp;
        for _ in 0..count - 1 {
            _block_temp = HMAC::mac(_block_temp.as_ref(), &key);
            for i3 in 0.._block_temp.len() {
                _block_result[i3] ^= _block_temp[i3];
            }
        }
        for i4 in 0.._block_temp.len() {
            result.push(_block_result[i4]);
        }
    }
    let mut ri: usize = HASH_LEN;
    while ri > r {
        result.pop();
        ri -= 1;
    }
    return result;
}
//Note there is a performance problem in pbkdf2-hmac-sha1 and pbkdf2-hmac-sha256 function in debug mode.
//Recommand use pbkdf2-hmac-sha512 function or build in release mode.
//在debug模式下，本库的pbkdf2-hmac-sha1和pbkdf2-hmac-sha256函数有性能问题
//建议使用release模式或者使用pbkdf2-hmac-sha512函数
