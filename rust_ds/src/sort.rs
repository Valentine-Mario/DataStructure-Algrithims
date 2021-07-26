pub fn bubble_sort(list: &mut Vec<i32>) -> &Vec<i32> {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..list.len() {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    list
}
