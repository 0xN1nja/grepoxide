#![allow(unused)]
#![allow(dead_code)]

use ansi_term::{self, *};
use std::io::{self, *};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(query: &'a str, file_path: &'a str) -> Self {
        Self {
            query: query,
            file_path: file_path,
        }
    }
}

pub fn search<'a>(query: &'a str, file_path: &'a str) -> Vec<String> {
    let mut result_vec: Vec<String> = Vec::new();
    let file: String = std::fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Unable To Read File");
        String::new()
    });
    for mut i in file.lines() {
        if i.contains(query) {
            result_vec.push(i.replace(query, &ansi_term::Color::Red.paint(query).to_string()[..]))
        }
    }
    result_vec
}