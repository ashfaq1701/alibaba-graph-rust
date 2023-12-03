pub fn load_files(start: u32, end: u32) {

}

pub fn get_start_end_time_given_breakdown(
    start_day: u32,
    start_hour: u32,
    start_minute: u32,
    start_second: u32,
    end_day: u32,
    end_hour: u32,
    end_minute: u32,
    end_second: u32
) -> (u32, u32) {
    let start_time = start_day * 24 * 60 * 60 +
        start_hour * 60 * 60 +
        start_minute * 60 +
        start_second;

    let end_time = end_day * 24 * 60 * 60 +
        end_hour * 60 * 60 +
        end_minute * 60 +
        end_second;

    (start_time, end_time)
}