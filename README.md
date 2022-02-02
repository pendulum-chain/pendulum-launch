# Pendulum Launch
A launcher for Pendulum and other substrate based parachains

# Usage 
## Launch parachain 
```
USAGE:
    pendulum_launch [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config &lt;config&gt;    
    -d, --debug &lt;debug&gt;      

SUBCOMMANDS:
    export-genesis    Export genesis data
    generate-specs    Generate specs
    help              Prints this message or the help of the given subcommand(s)
```

## Export genesis data
```
USAGE:
    pendulum_launch export-genesis [OPTIONS] <bin> <chain>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --outdir <outdir>    

ARGS:
    <bin>      
    <chain>
```

## Generate specs
```
USAGE:
    pendulum_launch generate-specs [OPTIONS] <bin>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --outdir <outdir>    

ARGS:
    <bin>
```
