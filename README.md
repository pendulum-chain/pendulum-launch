# Pendulum Launch
A launcher for Pendulum and other substrate based parachains

# Usage 
## Launch parachain 
```
pendulum-launch 0.1.0

USAGE:
    pendulum_launch [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    
    -d, --debug <debug>      

SUBCOMMANDS:
    export-genesis    Export genesis data
    generate-specs    Generate specs
    help              Prints this message or the help of the given subcommand(s)
```

## Export genesis data
```
pendulum_launch-export-genesis 0.1.0
Export genesis data

USAGE:
    pendulum_launch export-genesis [OPTIONS] <bin> <chain>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>        
    -o, --outdir <outdir>    

ARGS:
    <bin>      
    <chain>
```

## Generate specs
```
pendulum_launch-generate-specs 0.1.0
Generate specs

USAGE:
    pendulum_launch generate-specs [OPTIONS] <bin>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>        
    -o, --outdir <outdir>    

ARGS:
    <bin>
```
