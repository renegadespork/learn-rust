fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("My values are {:?}.", v);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    print_n_check_express(&v);
    let mut v: Vec<i32> = vec! [9, 10];
    print_n_check_express(&v);
    v.push(11);
    print_n_check_express(&v);
    v.push(13);
    print_n_check_express(&v);
    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    v2.append(&mut v);
    print_n_check_express(&v2);
    // mirror(&v2);

}

fn print_n_check_express (v: &Vec<i32>) {
    print_values(&v);
    check_values(&v);
}

fn print_values(v: &Vec<i32>) {
    println!("My values are now {:?}", v);
}

fn check_values (v: &Vec<i32>) {
    let third: Option<&i32> = v.get(2);
    let fourth: Option<&i32> = v.get(3);
    match third {
        Some(third) => match fourth {
            Some(fourth) => println!("What comes between {} and {}?", third, fourth),
            None => println!("There is no fourth value."),
        }
        None => println!("There is no third value."),
    }
}

// Broken
// fn mirror(v: &Vec<i32>) -> Vec<i32> {
//     let mut size = v.len();
//     let mut prepend = -1
//     while size > 0 {
//         let mut vtemp: Vec<i32> = [size*-1];
//         let vtemp = v.append(prepend);
//         println!("{:?}", vtemp);
//         let prepend = prepend -1;
//         let size = size - 1;
//     }
//     let vout: Vec<i32> = vec![1, 2, 3];
//     vout
// }