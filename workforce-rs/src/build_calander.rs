use crate::{Schedule, Type};

use ics::properties::{Description, DtEnd, DtStart, Summary};
use ics::{Event, ICalendar};
use log::warn;

use uuid::Uuid;

const FORMAT: &'static str = "%Y%m%dT%H%M%S%Z";

pub fn build(schedule: Schedule) -> ICalendar<'static> {
    let mut calendar = ICalendar::new("2.0", "ics-rs");
    for (_, day) in schedule.days {
        for (_, mut shift) in day.shifts {
            if shift.segments.len() == 0 {
                warn!("No segments on shift {}", day.start);
                continue;
            }
            let string = shift.shift_id;
            let uuid = Uuid::from_u128(string.parse::<u128>().unwrap());
            let s = format!("{}", shift.start.format(FORMAT));
            let mut event = Event::new(uuid.hyphenated().to_string(), s);

            event.push(DtStart::new(shift.start.format(FORMAT).to_string()));
            event.push(DtEnd::new(format!("{}", shift.end.format(FORMAT))));
            if shift.segments.len() == 1 {
                let segment = shift.segments.remove(0);
                if segment.segment_type == Type::Special {
                    continue;
                }
                let desc = format!("{} at {}", segment.department, segment.location);
                event.push(Summary::new(desc));
                event.push(Description::new("No Breaks"));
            } else if shift.segments.len() == 3 {
                let _ = shift.segments.remove(2);

                let brake = shift.segments.remove(1);

                let start = shift.segments.remove(0);
                let summary = format!("{} at {}", start.department, start.location);

                let desc = format!("Break From {} to {}", brake.start, brake.end);

                event.push(Summary::new(summary));
                event.push(Description::new(desc));
            } else {
                warn!("Too many segments");
            }

            calendar.add_event(event);
        }
    }
    calendar
}
