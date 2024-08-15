# chirp-generator

Command line interface (about as straightforward as it gets--see below
or run the binary with `-h` to print help)

```
Interface to generate linear/exponential frequency sine sweeps (chirps)

Usage: chirp-generator [OPTIONS] <--seconds <SECONDS>|--samples <SAMPLES>>

Options:
  -s, --seconds <SECONDS>        Length (in seconds) of the sweep -- conflicts with samples
  -n, --samples <SAMPLES>        Length (in samples) of the sweep -- conflicts with seconds
  -b, --begin <BEGIN>            Starting frequency of the sweep [default: 0]
  -e, --end <END>                Ending frequency of the sweep [default: 22050]
  -r, --samplerate <SAMPLERATE>  Samplerate of the output file [default: 44100]
  -x, --exponential              Exponential sweep (defaults to linear otherwise)
  -o, --output <OUTPUT>          Output file path
  -h, --help                     Print help
  -V, --version                  Print version
```
