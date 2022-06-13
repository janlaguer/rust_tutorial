// basically points to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    /*
    With non primitives, if you assign another variable to a
    piece of data, the first variable will no longer have
    that value. You'll need to use a reference (&) to point
    to that resource
    */

    // let vec1 = vec![1,2,3];
    // let vec2 = vec1;

    // this errors out because vec1 no longer has any data
    // println!("Values: {:?}", (vec1, vec2));

    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

}