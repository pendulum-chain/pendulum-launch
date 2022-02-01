Add Command and Task trait

```Rust
use std::process;
use crate::error::Result;

trait Command<T> {
  fn as_command(&self) -> std::process::Command;
}

trait Process<T> {
  fn spawn(&self) -> Result<process::Child>;
  fn kill(&self) -> Result<()>;
}
```

Define client structure

```yaml
name: pendulum-launch  
version: "0.1.0"
author: xiuxiu62 <justin@satoshipay.io>
about: Launches a substrate parachain
args:
  - config:
      short: c
      long: config
      value_name: FILE
      help: Sets an alternate config path
      takes_value: true
  - log_dir:
      short: l
      long: log-dir
      value_name: DIR
      help: Sets an alternate output directory for logged data
      takes_value: true
subcommands:
  - export-genesis:
      about: Exports collator genesis data
      args:
        - bin
            short: b
            long: bin
            value_name: FILE
            help: Collator binary
            takes_value: true
        - outdir:
            short: l
            long: outdir
            value_name: DIR
            help: Sets an alternate output directory for genesis data
            takes_value: true
  - generate-specs:
      about: Generates collator specs
      args:
        - bin
            short: b
            long: bin
            value_name: FILE
            help: Collator binary
            takes_value: true
        - outdir:
            short: l
            long: outdir
            value_name: DIR
            help: Sets an alternate output directory for specs
            takes_value: true
```
