pub mod cli;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

/// The type of strand to be parsed. Either DNA or RNA.
pub enum StrandType {
    DNA,
    RNA,
}

/// A single nucleotide; either A, C, T, G, or U.
#[derive(Clone, Copy)]
pub enum Nucleotide {
    Adenine,
    Cytosine,
    Thymine,
    Guanine,
    Uracil,
    Other(char),
}

impl From<char> for Nucleotide {
    /// Parses the character as a nucleotide.
    fn from(c: char) -> Self {
        // Handle all possible base codes. If an unrecognized code is passed, return unknown.
        match c {
            'a' | 'A' => Self::Adenine,
            'c' | 'C' => Self::Cytosine,
            't' | 'T' => Self::Thymine,
            'g' | 'G' => Self::Guanine,
            'u' | 'U' => Self::Uracil,
            unknown => Self::Other(unknown),
        }
    }
}

impl Nucleotide {
    /// Construct a complementary nucleotide from the current nucleotide.
    pub fn complementary_base(&self) -> Self {
        match self {
            Nucleotide::Adenine => Nucleotide::Thymine,
            Nucleotide::Cytosine => Nucleotide::Guanine,
            Nucleotide::Thymine => Nucleotide::Adenine,
            Nucleotide::Guanine => Nucleotide::Cytosine,
            Nucleotide::Uracil => Nucleotide::Thymine,
            unknown => *unknown,
        }
    }
}
