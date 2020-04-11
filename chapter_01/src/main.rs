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

fn main() {
    assert_eq!(reversed_string("stressed"), "desserts");
    assert_eq!(schooled("パタトクカシーー"), "パトカー");
    assert_eq!(concat_each_letter("パトカー", "タクシー"), "パタトクカシーー");
}
