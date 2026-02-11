pub mod v1_puzrs;

static mut SHARED_ARRAY: Vec<u8> = vec![];

#[unsafe(no_mangle)]
fn solve_problem(problem: *const u8, height: i32, width: i32, limit: usize) -> *const u8 {
    let problem = unsafe { std::slice::from_raw_parts(problem, (height * width) as usize)};
    let problem = problem.iter().map(|&x| x as i32).collect::<Vec<_>>();
    let ret_string = v1_puzrs::solve_problem(&problem, height, width, limit);

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
