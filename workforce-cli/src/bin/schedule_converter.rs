use clap::Parser;
use std::path::PathBuf;
use workforce_rs::build_calander::build;
use workforce_rs::Schedule;

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
    let schedule: Schedule = serde_json::from_reader(file)?;
    let calendar = build(schedule);
    calendar.save_file(args.output.unwrap_or(PathBuf::new().join("output.ics")))?;
    Ok(())
}
