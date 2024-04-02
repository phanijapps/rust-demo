use std::collections::HashSet;

fn main() {

    let array1=[1,3,453,32,55];
    let array2=[43,32,1,4,5];

    // Convert arrays into slices and concatenate
    let merged_array: Vec<_> = [array1.as_slice(), array2.as_slice()].concat();

    println!("Merged array: {:?}", merged_array);

    let set:HashSet<_> = merged_array.into_iter().collect();
    println!("Set: {:?}", set);



}
