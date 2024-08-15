//// command line interface setup

use clap::{Args, Parser};

/// Interface to generate linear/exponential frequency sine sweeps (chirps)
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Length of sweep
    #[command(flatten)]
    length: Length,

    /// Starting frequency of the sweep
    #[arg(short, long, default_value_t = 0_f32)]
    begin: f32,

    /// Ending frequency of the sweep
    #[arg(short, long, default_value_t = 22050_f32)]
    end: f32,

    /// Samplerate of the output file
    #[arg(short = 'r', long, default_value_t = 44100_u32)]
    samplerate: u32, // shouldn't be higher than 2^31

    /// Exponential sweep (defaults to linear otherwise)
    #[arg(short = 'x', long, default_value_t = false)]
    exponential: bool,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

/// Length specification args (mutually exclusive)
#[derive(Args)]
#[group(required = true, multiple = false)]
struct Length {
    /// Length (in seconds) of the sweep -- conflicts with samples
    #[arg(short = 's', long)]
    seconds: Option<f32>,

    /// Length (in samples) of the sweep -- conflicts with seconds
    #[arg(short = 'n', long)]
    samples: Option<u32>,
}

use std::fmt;

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Length {
                seconds: Some(t), ..
            } => write!(f, "{}s", t),
            &Length {
                samples: Some(t), ..
            } => write!(f, "{}n", t),
            _ => panic!(), // unreachable
        }
        // not enumerating every possible case because we'll panic elsewhere anyway
    }
}

//// library imports

use chirp_generator::{exponential_chirp, linear_chirp, write_to_wav};

fn main() {
    let args = Cli::parse();
    // generate filename from params unless given
    let filename = match args.output {
        Some(file) => file,
        None => format!(
            "{}chirp_{}_{}-{}_({}).wav",
            if args.exponential { "exp" } else { "lin" },
            args.length,
            args.begin,
            args.end,
            args.samplerate,
        ),
    };

    // calculate one from the other
    let (samples, seconds): (u32, f32) = match args.length {
        Length {
            seconds: Some(_),
            samples: Some(_),
        } => panic!(), // unreachable
        Length {
            seconds: Some(s), ..
        } if s.is_sign_negative() => panic!("Negative length requested!"),
        Length {
            seconds: Some(s), ..
        } => ((s * args.samplerate as f32) as u32, s),
        Length {
            samples: Some(n), ..
        } => (n, n as f32 / args.samplerate as f32),
        Length {
            seconds: None,
            samples: None,
        } => panic!(), // unreachable
    };

    // printing the specs of the requested chirp
    println!(
        "{} frequency sweep:",
        if args.exponential {
            "exponential"
        } else {
            "linear"
        },
    );
    println!("  length: {} samples ({} seconds)", samples, seconds);
    println!("  begin: {}", args.begin);
    println!("  end: {}", args.end);
    println!("  samplerate: {}", args.samplerate);

    // generate chirp and write to file
    let chirp_signal = if args.exponential {
        exponential_chirp(args.begin, args.end, samples as usize, args.samplerate)
    } else {
        linear_chirp(args.begin, args.end, samples as usize, args.samplerate)
    };
    match write_to_wav(&filename, &chirp_signal, args.samplerate) {
        Ok(_) => println!("Written to \"{filename}\""),
        Err(e) => println!("ERROR: {e}"),
    }
}
