use std::collections::HashMap;
use anyhow::anyhow;
use log::info;
use crate::{data, utils};
use crate::data::structs::{ConnectionProp, WindowIndexingType};
use crate::utils::get_int_option_value;

pub fn run_get_data(options: &HashMap<&str, &str>) -> anyhow::Result<()> {
    let mut start_time: u32 = 0;
    let mut end_time: u32 = 0;

    match (get_int_option_value(options, "start_time"), get_int_option_value(options,"end_time")) {
        (Some(start), Some(end)) => {
            start_time = start;
            end_time = end;
        }
        (Some(start), None) => {
            start_time = start;
        }
        (None, Some(end)) => {
            end_time = end;
        }
        _ => {}
    }

    let start_day = match get_int_option_value(options, "start_day") {
        Some(sd) => { sd }
        _ => { 0 }
    };

    let start_hour = match get_int_option_value(options, "start_hour") {
        Some(sh) => { sh }
        _ => { 0 }
    };

    let start_minute = match get_int_option_value(options, "start_minute") {
        Some(sm) => { sm }
        _ => { 0 }
    };

    let start_second = match get_int_option_value(options, "start_second") {
        Some(ss) => { ss }
        _ => { 0 }
    };

    let end_day = match get_int_option_value(options, "end_day") {
        Some(ed) => { ed }
        _ => { 0 }
    };

    let end_hour = match get_int_option_value(options, "end_hour") {
        Some(eh) => { eh }
        _ => { 0 }
    };

    let end_minute = match get_int_option_value(options, "end_minute") {
        Some(em) => { em }
        _ => { 0 }
    };

    let end_second = match get_int_option_value(options, "end_second") {
        Some(es) => { es }
        _ => { 0 }
    };

    let connection_prop = match options.get("connection_prop") {
        Some(&"instance_id") => ConnectionProp::InstanceId,
        Some(&"microservice_id") => ConnectionProp::MicroserviceId,
        _ => ConnectionProp::MicroserviceId
    };

    let indexing_type = match options.get("window_indexing_type") {
        Some(&"from_zero") => WindowIndexingType::FromZero,
        Some(&"seq_from_start") => WindowIndexingType::SeqFromStart,
        _ => WindowIndexingType::SeqFromStart
    };

    let start = data::structs::TimeBreakdown {
        day: start_day,
        hour: start_hour,
        minute: start_minute,
        second: start_second
    };

    let end = data::structs::TimeBreakdown {
        day: end_day,
        hour: end_hour,
        minute: end_minute,
        second: end_second
    };

    let (calc_start_time, calc_end_time) = utils::get_start_end_time_given_breakdown(
        &start,
        &end
    );

    if start_time == 0 {
        start_time = calc_start_time;
    }

    if end_time == 0 {
        end_time = calc_end_time;
    }

    if end_time <= start_time {
        return Err(anyhow!("Invalid start or end parameter passed"));
    }

    if let Some(window_size) = get_int_option_value(options, "window_size") {
        let loaded_window_files = data::get::load_files(
            start_time,
            end_time,
            window_size,
            &connection_prop,
            &indexing_type
        )?;

        info!("Windowed graphs are stored in {} files.", loaded_window_files.len());
        Ok(())
    } else {
        Err(anyhow!("Window size is a required parameter"))
    }
}