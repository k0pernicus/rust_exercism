fn d_to_r(c: char) -> char {
    match c {
        'A' => 'U',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => c,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct RibonucleicAcid {

    nucleotides: String

}

impl RibonucleicAcid {

    pub fn new(chain: &str) -> RibonucleicAcid {
        RibonucleicAcid { nucleotides: chain.to_string() }
    }

}

#[derive(Debug, Eq, PartialEq)]
pub struct DeoxyribonucleicAcid {
    
    nucleotides: String

}

impl DeoxyribonucleicAcid {

    pub fn new(chain: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { nucleotides: chain.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let transcription = self.nucleotides.chars().map(d_to_r).collect::<String>();
        RibonucleicAcid::new(&transcription)
    }

}
