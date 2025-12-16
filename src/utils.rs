pub fn remove_accents(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'á' | 'à' | 'ã' | 'â' => 'a',
            'Á' | 'À' | 'Ã' | 'Â' => 'A',
            'é' | 'ê' => 'e',
            'É' | 'Ê' => 'E',
            'í' => 'i',
            'Í' => 'I',
            'ó' | 'õ' | 'ô' => 'o',
            'Ó' | 'Õ' | 'Ô' => 'O',
            'ú' => 'u',
            'Ú' => 'U',
            'ç' => 'c',
            'Ç' => 'C',
            _ => c,
        })
        .collect()
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn l33t_vowels(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a' | 'A' => '4',
            'e' | 'E' => '3',
            'i' | 'I' => '1',
            'o' | 'O' => '0',
            _ => c,
        })
        .collect()
}

pub fn l33t_vowels_s(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            's' | 'S' => '$',
            'a' | 'A' => '4',
            'e' | 'E' => '3',
            'i' | 'I' => '1',
            'o' | 'O' => '0',
            _ => c,
        })
        .collect()
}

pub fn l33t_a(s: &str) -> String { s.replace(['a', 'A'], "4") }
pub fn l33t_e(s: &str) -> String { s.replace(['e', 'E'], "3") }
pub fn l33t_i(s: &str) -> String { s.replace(['i', 'I'], "1") }
pub fn l33t_o(s: &str) -> String { s.replace(['o', 'O'], "0") }
pub fn l33t_s_only(s: &str) -> String { s.replace(['s', 'S'], "$") }