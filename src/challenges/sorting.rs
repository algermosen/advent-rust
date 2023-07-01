pub fn selection_sort() {
    println!("You're making a selection sort")
}

pub fn bubble_sort() {
    println!("You're making a bubble sort");
    // let s = String::from("hello");

    // let r1 = s;
    // let r3 = r1;
    // println!("{}", r3);
    // // let r2 = &s;
    
    // // // println!("{}, {}", r1, r2);
    // // println!("{}", r2);

    // let x: i32 = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);

}

pub fn insertion_sort(arr: &mut [u32]) {
    // let len: usize = list.len();
    // let mut index: usize = 1;

    // while index < len {
    //     let comparator = list.get(index).expect("Get elementof index out of bounds");
    //     let mut lower_index: usize = index - 1;

    //     while list.get(lower_index).expect("Item out of bounds") > comparator {
    //         // switch
    //         list.swap(index, lower_index);

    //         if lower_index == usize::MIN {
    //             break;
    //         }

    //         lower_index = lower_index - 1;
    //     }

    //     index = index + 1;
    // }

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}
