#![allow(unused)]
#![allow(dead_code)]

pub struct Config<'a> {
    pub query: &'a str,
    pub path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(query: &'a str, path: &'a str) -> Self {
        Self {
            query: query,
            path: path,
        }
    }
}

pub fn search() {
    todo!()
}

pub fn search_case_sensitive() {
    todo!()
}
