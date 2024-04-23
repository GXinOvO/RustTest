use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();

    let mut sr1 = String::new();
    let mut map1 = HashMap::new();

    for i in 0..s.len() {
        if let Some(x) = map1.get(&sv[i]) {
            sr1.push(*x);
        } else {
            map1.insert(sv[i], tv[i]);
            sr1.push(tv[i]);
        }
    }

    let mut sr2 = String::new();
    let mut map2 = HashMap::new();
    for i in 0..s.len() {
        if let Some(x) = map2.get(&tv[i]) {
            sr2.push(*x);
        } else {
            map2.insert(tv[i], sv[i]);
            sr2.push(sv[i]);
        }
    }
    sr1 == t && sr2 == s
}
