use std::convert::TryFrom;

pub fn binary_search(list: &Vec<u32>, target: u32, low: u32, high: u32) -> bool {
    if low > high {
        return false;
    } else {
        let mid = usize::try_from((high + low) / 2).unwrap();
        if target == list[mid] {
            return true;
        } else if target < list[mid] {
            return binary_search(list, target, low, mid as u32 - 1);
        } else {
            return binary_search(list, target, mid as u32 + 1, high);
        }
    }
}
