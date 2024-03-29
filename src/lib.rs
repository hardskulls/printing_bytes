use crate::maptyping::{ErrIf, Mutate, Res, WrapInRes};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use thiserror::Error;

pub mod maptyping;

pub type Byte = u8;
pub type ByteList = Vec<Byte>;

#[derive(Debug, Copy, Clone)]
pub enum PrintMode {
    Binary,
    Hexadecimal,
    Octal,
    Decimal,
}

#[derive(Debug, Copy, Clone)]
pub enum ConversionMode {
    Binary = 2,
    Hexadecimal = 16,
    Octal = 8,
    Decimal = 10,
}

#[derive(Debug, Copy, Clone, Error)]
#[error("Empty source")]
pub struct EmptySourceError;

pub fn get_sample(path: &str) -> Res<String> {
    std::fs::read_to_string(path)?
        .err_if(|s| s.is_empty(), EmptySourceError)?
        .in_ok()
}

fn byte_from_str(s: &str, mode: ConversionMode) -> Result<Byte, ParseIntError> {
    Byte::from_str_radix(s, mode as u32)
}

pub fn parse_bytes(src: &str, mode: ConversionMode) -> Res<ByteList> {
    let src = src.err_if(|s| s.is_empty(), EmptyListError)?;

    let mut list = Vec::with_capacity(src.len());
    for s in src.split_whitespace() {
        list.push(byte_from_str(s, mode)?)
    }

    list.in_ok()
}

pub fn print_bytes(src: &[Byte], print_mode: PrintMode) -> Res<String> {
    src.err_if(|s| s.is_empty(), EmptyListError)?
        .into_iter()
        .map(|b| print_byte(*b, print_mode) + " ")
        .collect::<String>()
        .mutate(|list| list.pop())
        .in_ok()
}

pub fn print_byte(b: Byte, mode: PrintMode) -> String {
    match mode {
        PrintMode::Binary => format!("{b:0>8b}"),
        PrintMode::Hexadecimal => format!("{b:x}"),
        PrintMode::Octal => format!("{b:o}"),
        PrintMode::Decimal => format!("{b}"),
    }
}

#[derive(Debug, Copy, Clone, Error)]
#[error("Not enough tags")]
pub struct NotEnoughTagsError;

#[derive(Debug, Copy, Clone, Error)]
#[error("Empty list")]
pub struct EmptyListError;

pub fn replace_with_tags<S, T>(src: &[S], tags: HashSet<T>) -> Res<Vec<T>>
where
    S: Eq + Hash + Clone,
    T: Clone,
{
    let amount = src
        .len()
        .err_if(|&len| len > tags.len(), NotEnoughTagsError)?
        .err_if(|&len| len == 0, EmptyListError)?;

    let mut mappings = HashMap::with_capacity(amount);
    let mut tags = tags.into_iter();

    let mut new_tag = || unsafe { tags.next().unwrap_unchecked() };
    let get_or_update = |replaceable| mappings.entry(replaceable).or_insert(new_tag()).clone();

    src.iter().map(get_or_update).collect::<Vec<T>>().in_ok()
}

/// Returns a map that describes how many times a certain value occurs in `src`.
pub fn make_freq_map<T: Hash + Eq>(src: &[T]) -> Res<HashMap<&T, u32>> {
    let src = src.err_if(|s| s.is_empty(), EmptyListError)?;

    let mut occurences = HashMap::with_capacity(src.len());
    src.iter()
        .for_each(|byte| *occurences.entry(byte).or_insert(0) += 1);

    occurences.in_ok()
}

pub fn make_freq_list<T: Hash + Eq>(occurrences: HashMap<&T, u32>) -> Res<Vec<u32>> {
    occurrences
        .err_if(|o| o.is_empty(), EmptyListError)?
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .mutate(|list| list.sort())
        .in_ok()
}

pub fn make_replace_list<T>(start: T, end: T) -> HashSet<T>
where
    RangeInclusive<T>: Iterator<Item = T>,
    T: Hash + Eq,
{
    HashSet::from_iter(start..=end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_tags_test() -> Res<()> {
        let sample = get_sample("sample.txt")?;

        let (print_mode, conv_mode) = (PrintMode::Decimal, ConversionMode::Binary);
        let as_some = print_bytes(&parse_bytes(&sample, conv_mode)?, print_mode)?;

        let byte_list = as_some.split_whitespace().collect::<Vec<&str>>();
        let replace_map = make_replace_list('а', 'я');

        let tag_list = replace_with_tags(&byte_list, replace_map)?;

        // Check if tags are correct.
        let ref_occur = make_freq_list(make_freq_map(&byte_list)?)?;
        let test = make_freq_list(make_freq_map(tag_list.as_slice())?)?;
        for (idx, n) in test.into_iter().enumerate() {
            assert_eq!(ref_occur[idx], n);
        }

        Ok(())
    }
}
