// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 1:45:53 to 1:55:45

fn main(){
    // iterators
    let mut arr: [i8;4] = [1,2,3,4];
    let mut idx: i32 = 0;
    for val in &mut arr{
        println!("Index {}, value: {}", &idx,val);
        *val *= 2;
        idx += 1;
    }
    println!("{}",arr[1]);
}
