#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() {
        if second_list.is_empty() {
            Comparison::Equal
        } else {
            Comparison::Sublist
        }
    } else {
        if second_list.is_empty() {
            Comparison::Superlist
        } else {
            if first_list.len() == second_list.len() {
                let mut i = 0;
                while i < first_list.len() {
                    if first_list[i] != second_list[i] {
                        return Comparison::Unequal;
                    }
                    i += 1;
                }
                return Comparison::Equal;
            } else if first_list.len() > second_list.len() {
                let mut i = 0;
                while i <= first_list.len() - second_list.len() {
                    if &first_list[i..i+second_list.len()] == second_list {
                        return Comparison::Superlist;
                    }
                    i += 1;
                }
                return Comparison::Unequal;
            } else {
                let mut i = 0;
                while i <= second_list.len() - first_list.len() {
                    if &second_list[i..i+first_list.len()] == first_list {
                        return Comparison::Sublist;
                    }
                    i += 1;
                }
                return Comparison::Unequal;
            }
        }
    }
    
}
