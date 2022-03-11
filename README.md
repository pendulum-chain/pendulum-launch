# Pendulum Launch
A launcher for Pendulum and other substrate based parachains.  
For full documenation visit our [gitbook][gitbook].

# Configuration
Relative paths in your config are assumed to be relative to the current working directory the binary is being run in.
This is planned to be changed in the future.

# Usage 
## Launch parachain 
```
pendulum-launch 0.2.0

USAGE:
    pendulum-launch [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>
    -l, --log <log>

SUBCOMMANDS:
    export-genesis     Export genesis data
    generate-docker    Generate docker-compose.yml
    generate-specs     Generate specs
    help               Prints this message or the help of the given subcommand(s)
```

## Export genesis data
```
pendulum-launch-export-genesis 0.2.0
Export genesis data

USAGE:
    pendulum-launch export-genesis [OPTIONS] <bin> <chain>

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
pendulum-launch-generate-specs 0.2.0
Generate specs

USAGE:
    pendulum-launch generate-specs [OPTIONS] <bin>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>
    -o, --outdir <outdir>
    -i, --para-id <para-id>

ARGS:
    <bin>
```

## Generate docker-compose config
```
pendulum-launch-generate-docker 0.2.0
Generate docker-compose.yml

USAGE:
    pendulum-launch generate-docker [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --outdir <outdir>
```

[gitbook]: https://app.gitbook.com/o/axoDOM7fvGlVLdMc0tdk/s/JPteeI8zaYldKmZxPrYG/build/using-pendulum-launch
