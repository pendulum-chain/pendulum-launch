use std::{io, process};

pub trait Command {
    fn as_command(&self) -> process::Command;
}

pub trait Run<P> {
    fn run(&self) -> io::Result<P>;
}
