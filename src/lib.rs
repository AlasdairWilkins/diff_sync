pub fn compare(string1: &str, string2: &str) {
    if string1 == string2 {
        println!("The strings are the same.");
        return
    } 

    if string1.len() < string2.len() {
        let (position, insertion) = check_ends(string1, string2);
        return
    }

    if string1.len() > string2.len() {
        let (position, deletion) = check_ends(string2, string1);
        return
    }

    let (prefix1, prefix2) = remove_prefix(string1.to_string(), string2.to_string());

    let (suffix1, suffix2) = remove_suffix(prefix1.to_string(), prefix2.to_string());

    //check insertion or deletion

    let common_middle = find_common_middle(suffix1.to_string(), suffix2.to_string());

    println!("Common middle is {}", common_middle);
}

fn check_ends(shorter: &str, longer: &str) -> (usize, String) {
    let len = shorter.len();
    let start = longer.len() - len;

    if shorter == &longer[..len] {
        println!("Something got added/deleted at the end!");
        return (len, longer[len..].to_string())
    }

    if shorter == &longer[start..] {
        println!("Something got added/deleted at the start!");
        return (0, longer[..start].to_string())
    }

    return (0, String::new())
}

fn remove_prefix(string1: String, string2: String) -> (String, String) {
    if string1[..1] == string2[..1] {
        println!("{} and {} are the same!", string1[0..1].to_string(), string2[0..1].to_string());
        return remove_prefix(string1[1..].to_string(), string2[1..].to_string());
    }
    return (string1, string2)
}

fn remove_suffix(string1: String, string2: String) -> (String, String) {
    let remainder1 = string1.len() - 1;
    let remainder2 = string2.len() - 1;
    if string1[remainder1..] == string2[remainder2..] {
        println!("{} and {} are the same!", 
            string1[remainder1..].to_string(), 
            string2[remainder2..].to_string());
        return remove_suffix(string1[..remainder1].to_string(), string2[..remainder2].to_string())
    }
    return (string1, string2)
}

fn find_common_middle(string1: String, string2: String) -> String {
    if string1.len() < 4 || string2.len() < 4 {
        println!("No common middle!");
        return String::new()
    }
    let midpoint = string1.len()/2;
    let q2 = midpoint - midpoint/2;
    let q3 = midpoint + midpoint/2;
    let second_quarter = &string1[q2..midpoint].to_string();
    let third_quarter = &string1[midpoint..q3].to_string();
    let mut common_middle = String::new();
    if string2.contains(second_quarter) {
        common_middle = second_quarter.to_string() 
    } else if string2.contains(third_quarter) {
        common_middle = third_quarter.to_string()
    } else {
        println!("No common middle!");
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
    let remainder1 = string1.len() - 1;
    let remainder2 = string2.len() - 1;
    if string1[remainder1..] == string2[remainder2..] {
        println!("{} and {} are the same!", 
            string1[remainder1..].to_string(), 
            string2[remainder2..].to_string());
        return format!("{}{}",
            find_common_start(string1[..remainder1].to_string(), string2[..remainder2].to_string()), &string1[remainder1..])
    }
    return String::new()
}

fn find_common_end(string1: String, string2: String) -> String {
    if string1[..1] == string2[..1] {
        println!("{} and {} are the same!", string1[0..1].to_string(), string2[0..1].to_string());
        return format!("{}{}",
           &string1[..1], find_common_end(string1[1..].to_string(), string2[1..].to_string()))
    }
    return String::new()
}