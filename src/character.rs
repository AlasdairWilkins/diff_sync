use std::collections::HashMap;

pub fn compare(string1: &str, string2: &str) -> HashMap<usize, HashMap<String, String>> {

    let mut updates = HashMap::new();

    if string1 == string2 {
        return updates
    } 

    let (no_pre1, no_pre2) = remove_prefix(string1.to_string(), string2.to_string());

    let (no_pre_suf1, no_pre_suf2) = remove_suffix(no_pre1.to_string(), no_pre2.to_string());

    let pre_len1 = string1.len() - no_pre1.len();

    if no_pre_suf1.len() == 0 {
        let mut diff = HashMap::new();
        diff.insert(String::new(), no_pre_suf2);
        updates.insert(pre_len1, diff);
        return updates
    }

    if no_pre_suf2.len() == 0 {
        let mut diff = HashMap::new();
        diff.insert(no_pre_suf1, String::new());
        updates.insert(pre_len1, diff);
        return updates
    }

    return find_diffs(no_pre_suf1.to_string(), no_pre_suf2.to_string(), pre_len1);

}

fn find_diffs(string1: String, string2: String, pre_len1: usize) -> HashMap<usize, HashMap<String, String>> {
    let common_middle = find_common_middle(string1.to_string(), string2.to_string());
    if common_middle != "" {
        let mid_start1 = string1.find(&common_middle).unwrap();
        let mid_start2 = string2.find(&common_middle).unwrap();
        let update1 = find_diffs(string1[..mid_start1].to_string(), string2[..mid_start2].to_string(), pre_len1);
        let mid_end1 = mid_start1 + common_middle.len();
        let mid_end2 = mid_start2 + common_middle.len();
        let update2 = find_diffs(string1[mid_end1..].to_string(), string2[mid_end2..].to_string(), pre_len1 + mid_end1);
        return update1.into_iter().chain(update2).collect();
    }

    let mut updates = HashMap::new();
    let mut diff = HashMap::new();
    diff.insert(string1, string2);
    updates.insert(pre_len1, diff);
    return updates
     
}

fn remove_prefix(string1: String, string2: String) -> (String, String) {
    if string1.len() > 0 && string2.len() > 0 {
        if string1[..1] == string2[..1] {
            return remove_prefix(string1[1..].to_string(), string2[1..].to_string());
        }
    }
    return (string1, string2)
}

fn remove_suffix(string1: String, string2: String) -> (String, String) {
    if string1.len() > 0 && string2.len() > 0 {
        let remainder1 = string1.len() - 1;
        let remainder2 = string2.len() - 1;
        if string1[remainder1..] == string2[remainder2..] {
            return remove_suffix(string1[..remainder1].to_string(), string2[..remainder2].to_string())
        }
    }
    return (string1, string2)
}

fn find_common_middle(string1: String, string2: String) -> String {
    let mut common_middle = String::new();
    if string1.len() < 4 || string2.len() < 4 {
        return common_middle
    }
    let midpoint = string1.len()/2;
    let q2 = midpoint - midpoint/2;
    let q3 = midpoint + midpoint/2;
    let second_quarter = &string1[q2..midpoint].to_string();
    let third_quarter = &string1[midpoint..q3].to_string();
    if string2.contains(second_quarter) {
        common_middle = second_quarter.to_string() 
    } else if string2.contains(third_quarter) {
        common_middle = third_quarter.to_string()
    } else {
        return common_middle
    }
    let start1 = string1.find(&common_middle).unwrap();
    let start2 = string2.find(&common_middle).unwrap();
    let end1 = start1 + common_middle.len();
    let end2 = start2 + common_middle.len();

    let common_start = find_common_start(string1[..start1].to_string(), string2[..start2].to_string());
    let common_end = find_common_end(string1[end1..].to_string(), string2[end2..].to_string());

    return format!("{}{}{}", common_start, common_middle, common_end)
}

fn find_common_start(string1: String, string2: String) -> String {
    if string1.len() > 0 && string2.len() > 0 {
        let remainder1 = string1.len() - 1;
        let remainder2 = string2.len() - 1;
        if string1[remainder1..] == string2[remainder2..] {
            return format!("{}{}",
                find_common_start(string1[..remainder1].to_string(), string2[..remainder2].to_string()), &string1[remainder1..])
        }
    }    
    return String::new()
}

fn find_common_end(string1: String, string2: String) -> String {
    if string1.len() > 0 && string2.len() > 0 {
        if string1[..1] == string2[..1] {
            return format!("{}{}",
                &string1[..1], find_common_end(string1[1..].to_string(), string2[1..].to_string()))
        }
    }
    return String::new()
}

pub fn update(original: String, updates: &HashMap<usize, HashMap<String, String>>, position: usize) -> String {
    if original.len() > 0 {
        if updates.contains_key(&position) {
            let diff = updates.get(&position).unwrap();
            let key = find_key(diff, &original, 1);
            if key == "" {
                return format!("{}{}{}", diff.get(&key).unwrap(), original[..1].to_string(), update(original[1..].to_string(), updates, position+1))
            }
            return format!("{}{}", diff.get(&key).unwrap(), update(original[key.len()..].to_string(), updates, position + key.len()))
        }
        return format!("{}{}", original[..1].to_string(), update(original[1..].to_string(), updates, position + 1))
    }
    if original.len() == 0 {
        if updates.contains_key(&position) {
            let diff = updates.get(&position).unwrap();
            let key = find_key(diff, &original, 1);
            return diff.get(&key).unwrap().to_string()
        }
    }
    return String::new()
}

fn find_key (diff: &HashMap<String, String>, original: &String, length: usize) -> String {
    if diff.contains_key("") {
        return String::new()
    }
    if !diff.contains_key(&original[..length]) {
        return find_key(diff, original, length + 1);
    }
    return original[..length].to_string()
}