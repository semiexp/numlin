pub mod v1_puzrs;

static mut SHARED_ARRAY: Vec<u8> = vec![];

fn hex_to_i32(c: u8) -> Option<i32> {
    match c {
        b'0'..=b'9' => Some((c - b'0') as i32),
        b'a'..=b'f' => Some((c - b'a' + 10) as i32),
        _ => None,
    }
}

fn decode_url(url: &str) -> Option<(usize, usize, Vec<i32>)> {
    let url = url.strip_prefix("http://").or_else(|| url.strip_prefix("https://"))?;
    let url = url.strip_prefix("puzz.link/").or_else(|| url.strip_prefix("pzv.jp/")).or_else(|| url.strip_prefix("pzprxs.vercel.app/"))?;
    let url = url.strip_prefix("p?").or_else(|| url.strip_prefix("p.html?"))?;
    let url = url.strip_prefix("numlin/").or_else(|| url.strip_prefix("numberlink/"))?;

    let parts = url.split('/').collect::<Vec<_>>();
    if parts.len() != 3 {
        return None;
    }

    let width = parts[0].parse::<usize>().ok()?;
    let height = parts[1].parse::<usize>().ok()?;
    let puzzle_data = parts[2].as_bytes();

    let mut ret = vec![0; height * width];
    let mut idx = 0;
    let mut i = 0;

    while i < puzzle_data.len() {
        if b'g' <= puzzle_data[i] && puzzle_data[i] <= b'z' {
            idx += (puzzle_data[i] - b'f') as usize;
            i += 1;
            continue;
        } else {
            let n;
            if puzzle_data[i] == b'-' {
                if i + 2 >= puzzle_data.len() {
                    return None;
                }
                let high = hex_to_i32(puzzle_data[i + 1])?;
                let low = hex_to_i32(puzzle_data[i + 2])?;
                n = high * 16 + low;
                i += 3;
            } else if puzzle_data[i] == b'+' {
                if i + 3 >= puzzle_data.len() {
                    return None;
                }
                let high = hex_to_i32(puzzle_data[i + 1])?;
                let mid = hex_to_i32(puzzle_data[i + 2])?;
                let low = hex_to_i32(puzzle_data[i + 3])?;
                n = high * 256 + mid * 16 + low;
                i += 4;
            } else {
                n = hex_to_i32(puzzle_data[i])?;
                i += 1;
            }
            if idx >= ret.len() {
                return None;
            }
            ret[idx] = n;
            idx += 1;
        }
    }

    Some((height, width, ret))
}

#[unsafe(no_mangle)]
fn enumerate_answers_problem(url: *const u8, len: usize, limit: usize) -> *const u8 {
    let url = unsafe { std::slice::from_raw_parts(url, len) };
    let url = std::str::from_utf8(url).unwrap_or("");
    let problem = decode_url(url);

    let ret_string;
    if problem.is_none() {
        ret_string = "{\"status\":\"error\",\"description\":\"failed to decode URL\"}".to_string();
    } else {
        let (height, width, problem) = problem.unwrap();
        ret_string = v1_puzrs::solve_problem(&problem, height as i32, width as i32, limit);
    }

    let ret_len = ret_string.len();

    #[allow(static_mut_refs)]
    unsafe {
        SHARED_ARRAY.clear();
        SHARED_ARRAY.reserve(4 + ret_len);
        SHARED_ARRAY.push((ret_len & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 8) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 16) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 24) & 0xff) as u8);
        SHARED_ARRAY.extend_from_slice(ret_string.as_bytes());
        SHARED_ARRAY.as_ptr()
    }
}
