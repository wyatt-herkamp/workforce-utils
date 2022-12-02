use std::path::PathBuf;
use clap::Parser;
use log::warn;
use schedule_stealer::Schedule;
use std::io::Write;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Convert {
    input: PathBuf,
    output: Option<PathBuf>,
}
fn main() -> anyhow::Result<()> {
    simple_log::quick!();
    let args = Convert::parse();
    let file = std::fs::File::open(args.input)?;
    let mut schedule: Schedule = serde_json::from_reader(file)?;
    let mut days = schedule.days.into_values().collect::<Vec<_>>();
    days.sort_by_key(|day| day.start);

    let file = std::fs::File::create(args.output.unwrap_or(PathBuf::new().join("output.txt")))?;
    let mut writer = std::io::BufWriter::new(file);
writeln!(writer, "|Day| |Date(mm/dd/yy)| from |start| to |end|")?;
    for day in days {
        for (_,  shift) in day.shifts {
            if shift.segments.len() == 0 {
                warn!("No segments on shift {}", day.start);
                continue;
            }
            let time = day.start;
            let  date = format!("{}: ", time.format("%A %D"));
            let start = shift.start.format("%I:%M%p");
            let end = shift.end.format("%I:%M%p");
            writeln!(writer, "{date} from {start} to {end}")?;
        }
    }
    Ok(())
}
