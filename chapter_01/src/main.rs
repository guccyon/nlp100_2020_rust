use std::collections::HashMap;

macro_rules! hash {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn reversed_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn schooled(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            result.push(c)
        }
    }
    result
}

fn concat_each_letter(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .flat_map(|e| vec![e.0, e.1] )
        .collect()
}

fn pi(s: &str) -> Vec<usize> {
    s.split_whitespace()
     .map(|word| word.replace(",","").replace(".","").len() )
     .collect()
}

fn atomic_symbols(s: &str) -> HashMap<&str, usize> {
    const INDEXES:[usize;9] = [1,5,6,7,8,9,15,16,19];

    let mut hash:HashMap<&str, usize> = HashMap::new();
    for (i, word) in s.split_whitespace().enumerate() {
        if INDEXES.contains(&(i + 1)) {
            let k = &word[..1];
            hash.insert(k, i + 1);
        } else {
            let k = &word[..2];
            hash.insert(k, i + 1);
        }
    }
    hash
}

fn ngram<S>(seq: Vec<S>, n: usize) -> Vec<String>
where
    S: Into<String>
{
    let cloned: Vec<String> = seq.into_iter().map(|s|s.into()).collect();

    let mut result:Vec<String> = Vec::new();
    for i in 0..(cloned.len() - 1) {
        let mut tmp: Vec<String> = Vec::new();
        for j in 0..n {
            let e = &cloned[i + j];
            tmp.push(e.to_string());
        }
        result.push(tmp.join(""));
    }

    result
}

fn main() {
    // 00. Reversed string
    assert_eq!(reversed_string("stressed"), "desserts");

    // 01. “schooled”
    assert_eq!(schooled("パタトクカシーー"), "パトカー");

    // 02. “shoe” + “cold” = “schooled”
    assert_eq!(concat_each_letter("パトカー", "タクシー"), "パタトクカシーー");

    // 03. Pi
    {
        let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        assert_eq!(pi(s), vec![3,1,4,1,5,9,2,6,5,3,5,8,9,7,9]);
    }

    // 04. Atomic symbols
    {
        // 1,5,6,7,8,9,15,16,19 : 1st
        // 2,3,4,10,11,12,13,14,17,18 : 1st & 2nd
        let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King";
        assert_eq!(atomic_symbols(s), hash![
            "H"  => 1,
            "He" => 2,
            "Li" => 3,
            "Be" => 4,
            "B"  => 5,
            "C"  => 6,
            "N"  => 7,
            "O"  => 8,
            "F"  => 9,
            "Ne" => 10,
            "Na" => 11,
            "Mi" => 12,
            "Al" => 13,
            "Si" => 14,
            "P"  => 15,
            "S"  => 16,
            "Cl" => 17,
            "Ar" => 18,
            "K"  => 19
        ]);
    }

    // 05. n-gram
    {
        let s6 = "I am an NLPer";
        let seq:Vec<String> = s6.chars().map(|c| c.to_string() ).collect();
        assert_eq!(
            ngram(seq, 2),
            vec!["I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er"]
        );
    
        let seq:Vec<&str> = s6.split_whitespace().collect();
        assert_eq!(
            ngram(seq, 2),
            vec!["Iam", "aman", "anNLPer"]
        );
    }
}
