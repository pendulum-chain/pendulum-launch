use crate::node::{Command, Run};
use crate::{Collator, Validator};
use std::{io, process};

struct Children(Vec<process::Child>);

impl Children {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, child: process::Child) {
        self.0.push(child);
    }

    pub fn append(&mut self, children: &mut Children) {
        self.0.append(&mut children.0)
    }
}

impl From<Vec<process::Child>> for Children {
    fn from(children: Vec<process::Child>) -> Self {
        Self(children)
    }
}

impl FromIterator<process::Child> for Children {
    fn from_iter<I: IntoIterator<Item = process::Child>>(iter: I) -> Self {
        Self::from(iter.into_iter().fold(vec![], |mut acc, item| {
            acc.push(item);
            acc
        }))
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    validators: Vec<Validator<'a>>,
    collators: Vec<Collator<'a>>,
}

impl<'a> Config<'a> {
    pub fn new(validators: Vec<Validator<'a>>, collators: Vec<Collator<'a>>) -> Self {
        Self {
            validators,
            collators,
        }
    }

    fn run_collection<N>(&self, nodes: &Vec<N>) -> io::Result<Children>
    where
        N: Command + Run<process::Child>,
    {
        nodes
            .iter()
            .map(|node| node.run())
            .collect::<io::Result<Children>>()
    }
}

impl<'a> Run<Children> for Config<'a> {
    fn run(&self) -> io::Result<Children> {
        let mut handles = self.run_collection(&self.validators)?;
        let mut collator_handles = self.run_collection(&self.collators)?;
        handles.append(&mut collator_handles);

        Ok(handles)
    }
}
