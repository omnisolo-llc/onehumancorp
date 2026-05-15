use super::availability::{AvailabilityCalculator, BusinessHours};
use super::models::{FreeBusyCalendar, FreeBusyResponse, TimePeriod};
use chrono::{Duration, NaiveTime, TimeZone, Utc, Weekday};
use std::collections::HashMap;

#[test]
fn test_models_serialization() {
    let json_data = r#"{
        "kind": "calendar#freeBusy",
        "timeMin": "2023-01-01T00:00:00Z",
        "timeMax": "2023-01-02T00:00:00Z",
        "calendars": {
            "primary": {
                "busy": [
                    {
                        "start": "2023-01-01T10:00:00Z",
                        "end": "2023-01-01T11:00:00Z"
                    }
                ]
            }
        }
    }"#;

    let response: FreeBusyResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(response.kind, "calendar#freeBusy");
    assert!(response.calendars.contains_key("primary"));
    assert_eq!(response.calendars["primary"].busy.len(), 1);
}

#[test]
fn test_availability_calculation_no_conflicts() {
    let mut calendars = HashMap::new();
    calendars.insert(
        "primary".to_string(),
        FreeBusyCalendar {
            errors: None,
            busy: vec![],
        },
    );

    let freebusy = FreeBusyResponse {
        kind: "calendar#freeBusy".to_string(),
        time_min: Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap(), // Monday
        time_max: Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap(),
        groups: None,
        calendars,
    };

    let business_hours = BusinessHours {
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        timezone: chrono::FixedOffset::east_opt(0).unwrap(),
        work_days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri],
    };

    let search_start = Utc.with_ymd_and_hms(2023, 1, 2, 8, 0, 0).unwrap();
    let search_end = Utc.with_ymd_and_hms(2023, 1, 2, 12, 0, 0).unwrap();
    let slot_duration = Duration::hours(1);

    let slots = AvailabilityCalculator::calculate_available_slots(
        &freebusy,
        "primary",
        &business_hours,
        search_start,
        search_end,
        slot_duration,
    )
    .unwrap();

    // 8-9 (outside BH), 9-10 (in), 10-11 (in), 11-12 (in)
    assert_eq!(slots.len(), 3);
    assert_eq!(slots[0].start, Utc.with_ymd_and_hms(2023, 1, 2, 9, 0, 0).unwrap());
    assert_eq!(slots[2].start, Utc.with_ymd_and_hms(2023, 1, 2, 11, 0, 0).unwrap());
}

#[test]
fn test_availability_calculation_with_conflicts() {
    let mut calendars = HashMap::new();
    // Add a busy period from 10:00 to 11:30
    calendars.insert(
        "primary".to_string(),
        FreeBusyCalendar {
            errors: None,
            busy: vec![TimePeriod {
                start: Utc.with_ymd_and_hms(2023, 1, 2, 10, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2023, 1, 2, 11, 30, 0).unwrap(),
            }],
        },
    );

    let freebusy = FreeBusyResponse {
        kind: "calendar#freeBusy".to_string(),
        time_min: Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap(),
        time_max: Utc.with_ymd_and_hms(2023, 1, 3, 0, 0, 0).unwrap(),
        groups: None,
        calendars,
    };

    let business_hours = BusinessHours {
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        timezone: chrono::FixedOffset::east_opt(0).unwrap(),
        work_days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri],
    };

    let search_start = Utc.with_ymd_and_hms(2023, 1, 2, 9, 0, 0).unwrap();
    let search_end = Utc.with_ymd_and_hms(2023, 1, 2, 13, 0, 0).unwrap();
    let slot_duration = Duration::hours(1);

    let slots = AvailabilityCalculator::calculate_available_slots(
        &freebusy,
        "primary",
        &business_hours,
        search_start,
        search_end,
        slot_duration,
    )
    .unwrap();

    // Checked slots:
    // 9-10 (available)
    // 10-11 (conflict)
    // 11-12 (conflict)
    // 12-13 (available)
    assert_eq!(slots.len(), 2);
    assert_eq!(slots[0].start, Utc.with_ymd_and_hms(2023, 1, 2, 9, 0, 0).unwrap());
    assert_eq!(slots[1].start, Utc.with_ymd_and_hms(2023, 1, 2, 12, 0, 0).unwrap());
}

#[test]
fn test_availability_outside_work_days() {
    let mut calendars = HashMap::new();
    calendars.insert(
        "primary".to_string(),
        FreeBusyCalendar {
            errors: None,
            busy: vec![],
        },
    );

    let freebusy = FreeBusyResponse {
        kind: "calendar#freeBusy".to_string(),
        time_min: Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(), // Sunday
        time_max: Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap(),
        groups: None,
        calendars,
    };

    let business_hours = BusinessHours {
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        timezone: chrono::FixedOffset::east_opt(0).unwrap(),
        work_days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri],
    };

    let search_start = Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap();
    let search_end = Utc.with_ymd_and_hms(2023, 1, 1, 17, 0, 0).unwrap();
    let slot_duration = Duration::hours(1);

    let slots = AvailabilityCalculator::calculate_available_slots(
        &freebusy,
        "primary",
        &business_hours,
        search_start,
        search_end,
        slot_duration,
    )
    .unwrap();

    // It's Sunday, so 0 slots should be available
    assert_eq!(slots.len(), 0);
}
