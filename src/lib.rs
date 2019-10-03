/// Check if the given string is a valid ID number.
///
/// # Examples
///
/// ```
/// assert_eq!(true, taiwan_id::is_valid("A123456789"));
/// assert_eq!(false, taiwan_id::is_valid("A987654321"));
/// ```
pub fn is_valid(id: &str) -> bool {
    if id.len() != 10 {
        return false;
    }
    let mut a: [u8; 11] = [0; 11];
    let mut iter = id.chars();
    let first_letter = iter.next().unwrap();
    if let 'A'...'Z' = first_letter {
        let pair = code_map(first_letter);
        a[0] = pair[0];
        a[1] = pair[1];
    } else {
        return false;
    }

    let mut i = 2;
    for c in iter {
        if let '0'...'9' = c {
            a[i] = c as u8 - '0' as u8;
            i += 1;
        } else {
            return false;
        }
    }
    sum(&a) % 10 == 0
}

/// Generate a random ID with the given prefix.
/// Same as `generate_prefix("")`
pub fn generate() -> String {
    generate_prefix("")
}

/// Generate a random ID with the given prefix.
///
/// For more information, please refere to [wiki](https://zh.wikipedia.org/wiki/%E4%B8%AD%E8%8F%AF%E6%B0%91%E5%9C%8B%E5%9C%8B%E6%B0%91%E8%BA%AB%E5%88%86%E8%AD%89#%E9%A9%97%E8%AD%89%E8%A6%8F%E5%89%87)
///
/// # Examples
///
/// ```
/// // Generate a random ID for Taipei City:
/// let id = taiwan_id::generate_prefix("A");
/// assert!(id.starts_with("A"));
/// assert!(taiwan_id::is_valid(&id));
///
/// // Generate a random female ID for Taipei City
/// let id = taiwan_id::generate_prefix("A2");
/// assert!(id.starts_with("A2"));
/// assert!(taiwan_id::is_valid(&id));
/// ```
pub fn generate_prefix(prefix: &str) -> String {
    if prefix.len() > 9 {
        panic!("prefix is too long");
    }

    use rand::Rng;
    let mut rng = rand::thread_rng();

    if prefix.is_empty() {
        return generate_prefix(&format!(
            "{}{}",
            rng.gen_range(b'A', b'Z') as char,
            rng.gen_range(1, 3)
        ));
    }

    if prefix.len() == 1 {
        return generate_prefix(&format!("{}{}", prefix, rng.gen_range(1, 3)));
    }

    let first_letter = prefix.chars().next().unwrap();
    if let 'A'...'Z' = first_letter {
    } else {
        panic!("prefix is not valid")
    }

    let pair = code_map(first_letter);
    let mut a: [u8; 11] = [pair[0], pair[1], 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut a_index = 2;
    for i in prefix[1..].chars() {
        if let '0'...'9' = i {
        } else {
            panic!("prefix is not valid")
        }
        a[a_index] = i as u8 - '0' as u8;
        a_index += 1;
    }
    let len = a.len() - 1;
    for i in &mut a[a_index..len] {
        *i = rng.gen::<u8>() % 10;
    }
    a[len] = (10 - (sum(&a) % 10) as u8) % 10;
    a[prefix.len() + 1..]
        .iter()
        .fold(String::from(prefix), |s, i| s + &i.to_string())
}

fn sum(ary: &[u8]) -> u16 {
    static MULTIPLIERS: [u8; 11] = [1, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
    ary.iter().enumerate().fold(0, |acc, (index, value)| {
        acc + (MULTIPLIERS[index] * value) as u16
    })
}

fn code_map(c: char) -> [u8; 2] {
    static CODE_MAP: [[u8; 2]; 26] = [
        [1, 0],
        [1, 1],
        [1, 2],
        [1, 3],
        [1, 4],
        [1, 5],
        [1, 6],
        [1, 7],
        [3, 4],
        [1, 8],
        [1, 9],
        [2, 0],
        [2, 1],
        [2, 2],
        [3, 5],
        [2, 3],
        [2, 4],
        [2, 5],
        [2, 6],
        [2, 7],
        [2, 8],
        [2, 9],
        [3, 2],
        [3, 0],
        [3, 1],
        [3, 3],
    ];
    CODE_MAP[(c as u8 - 'A' as u8) as usize]
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_valid() {
        assert!(super::is_valid("A123456789"));
        assert!(!super::is_valid("A1234567899"));
        assert!(!super::is_valid("Z123456789"));
        assert!(!super::is_valid(""));
        assert!(!super::is_valid("A一二三四五六七八九"));
    }

    #[test]
    fn generate() {
        let id = super::generate_prefix("A1");
        assert!(id.starts_with("A1"));
        assert!(super::is_valid(&id));

        let id = super::generate_prefix("A");
        assert!(id.starts_with("A"));
        assert!(super::is_valid(&id));

        let id = super::generate_prefix("");
        assert!(super::is_valid(&id));

        let id = super::generate();
        assert!(super::is_valid(&id));
    }
}
