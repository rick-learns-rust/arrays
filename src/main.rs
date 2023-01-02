use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let mut ys: [i32; 500] = [0; 500];

    let q: usize = 12412412412412341234;
    ys[0] = q as i32;
    let opt = i32::try_from(q).ok();
    match opt {
        Some(val) => ys[1] = val,
        None => println!("too big"),
    }

    for i in 0..xs.len() {
        //this block of code
        let opt = i32::try_from(i).ok();
        match opt {
            Some(val) => ys[i + 1] = val,
            None => println!("too big"),
        }

        //and this line of code
        ys[i + 1] = i as i32;

        //do the same thing in most cases, but the block is safer
        //because it handles indexes larger than a signed i32
    }

    for i in xs {
        ys[i as usize] = i;
    }

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // OOPS, one element too far
        match xs.get(i) {
            Some(x_val) => println!("{}: {}", i, x_val),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing causes runtime error
    // println!("{}", xs[5]);
}
