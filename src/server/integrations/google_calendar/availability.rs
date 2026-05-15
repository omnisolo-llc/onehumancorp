use crate::integrations::google_calendar::models::{FreeBusyResponse, TimePeriod};
use chrono::{DateTime, Datelike, Duration, NaiveTime, Utc};

#[derive(Debug, Clone)]
pub struct AvailableSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BusinessHours {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    // Using an offset timezone for simplicity
    pub timezone: chrono::FixedOffset,
    pub work_days: Vec<chrono::Weekday>,
}

pub struct AvailabilityCalculator;

impl AvailabilityCalculator {
    pub fn calculate_available_slots(
        freebusy: &FreeBusyResponse,
        calendar_id: &str,
        business_hours: &BusinessHours,
        search_start: DateTime<Utc>,
        search_end: DateTime<Utc>,
        slot_duration: Duration,
    ) -> Result<Vec<AvailableSlot>, String> {
        let mut available_slots = Vec::new();

        let busy_periods = if let Some(calendar) = freebusy.calendars.get(calendar_id) {
            &calendar.busy
        } else {
            return Err(format!("Calendar {} not found in FreeBusy response", calendar_id));
        };

        let mut current_time = search_start;

        while current_time + slot_duration <= search_end {
            let slot_end = current_time + slot_duration;

            // 1. Check if within business hours
            let local_start = current_time.with_timezone(&business_hours.timezone);
            let local_end = slot_end.with_timezone(&business_hours.timezone);

            let is_work_day = business_hours.work_days.contains(&local_start.weekday());
            let is_within_hours = local_start.time() >= business_hours.start_time
                && local_end.time() <= business_hours.end_time;

            if is_work_day && is_within_hours {
                // 2. Check for conflicts with busy periods
                let mut conflict = false;
                for busy in busy_periods {
                    // Conflict if slot overlaps with busy period
                    // Overlap happens if: slot_start < busy_end AND slot_end > busy_start
                    if current_time < busy.end && slot_end > busy.start {
                        conflict = true;
                        break;
                    }
                }

                if !conflict {
                    available_slots.push(AvailableSlot {
                        start: current_time,
                        end: slot_end,
                    });
                }
            }

            // Move to the next possible slot (e.g., every 30 minutes)
            // Can be configurable, but assuming contiguous slots of `slot_duration` for simplicity
            current_time += slot_duration;
        }

        Ok(available_slots)
    }
}

// Add the chrono Datelike trait that's needed for .weekday()
