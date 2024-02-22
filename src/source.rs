use core::str::FromStr;

use std::io::{BufRead, BufReader, Read};

use exr::prelude::Vec2;

pub fn pos2val1d<F, T>(getter: &F, width: usize, alt: T, pos: Vec2<usize>) -> T
where
    F: Fn(usize) -> Option<T>,
    T: Copy,
{
    let x: usize = pos.0;
    let y: usize = pos.0;
    let wy: usize = width * y;
    let ix: usize = wy + x;
    getter(ix).unwrap_or(alt)
}

pub fn strings2vec_fn<I, F, T>(strings: I, conv: F) -> Vec<T>
where
    I: Iterator<Item = String>,
    F: Fn(&str) -> T,
{
    strings.map(|s| conv(s.as_str())).collect()
}

pub fn strings2vec<I, T>(strings: I, alt: Option<T>) -> Vec<T>
where
    I: Iterator<Item = String>,
    T: FromStr + Copy,
{
    strings
        .flat_map(|s| str::parse(s.as_str()).ok().or(alt))
        .collect()
}

pub fn read2vec<R, T>(rdr: R, alt: Option<T>) -> Vec<T>
where
    R: Read,
    T: FromStr + Copy,
{
    let br = BufReader::new(rdr);
    let lines = br.lines();
    lines
        .flat_map(|rslt| rslt.ok().and_then(|s| str::parse(s.as_str()).ok().or(alt)))
        .collect()
}
