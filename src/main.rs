use clap::{Parser, Subcommand};
use song_length_cli::{
    Error,
    Exit,
    Result,
    calculate_bars_to_length,
    calculate_length_in_bars,
    dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "song-length-cli command-line")]
pub struct SongLengthCliCli {
    #[command(subcommand)]
    command: TopLevelCommand,
}
impl SongLengthCliCli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for SongLengthCliCli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum TopLevelCommand {
    BarsToMinutes(BarsToMinutesOpt),
    MinutesToBars(MinutesToBarsOpt),
}
impl SubcommandDispatcher<Error> for TopLevelCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            TopLevelCommand::BarsToMinutes(op) => op.dispatch()?,
            TopLevelCommand::MinutesToBars(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct BarsToMinutesOpt {
    #[arg()]
    bars: f32,
    #[arg(short, long, default_value = "128.0")]
    bpm: f32,
    #[arg(short = 'p', long = "beats-per-bar", default_value = "4.0")]
    per_bar: f32,
}
impl BarsToMinutesOpt {
    pub fn length_in_minutes(&self) -> f32 {
        calculate_bars_to_length(self.bars, self.bpm, self.per_bar)
    }
}
impl ArgsDispatcher<Error> for BarsToMinutesOpt {
    fn dispatch(&self) -> Result<()> {
        let bpm = self.bpm;
        println!("total minutes for a {bpm}bpm song: {length}m", length = self.length_in_minutes());
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct MinutesToBarsOpt {
    #[arg()]
    length_in_minutes: f32,
    #[arg(short, long, default_value = "128.0")]
    bpm: f32,
    #[arg(short = 'p', long = "beats-per-bar", default_value = "4.0")]
    per_bar: f32,
}
impl MinutesToBarsOpt {
    pub fn length_in_bars(&self) -> f32 {
        calculate_length_in_bars(self.length_in_minutes, self.bpm, self.per_bar)
    }
}
impl ArgsDispatcher<Error> for MinutesToBarsOpt {
    fn dispatch(&self) -> Result<()> {
        let bpm = self.bpm;
        let length = self.length_in_minutes;
        println!(
            "total bars for a {length}m at {bpm}bpm song: {bars}",
            bars = self.length_in_bars()
        );
        Ok(())
    }
}

fn main() -> Exit {
    SongLengthCliCli::main()
}
