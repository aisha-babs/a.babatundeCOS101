    // The iter functions fetches values of all elements in an array.

    fn main() {
        let arr1:[i32;4] = [10,20,30,40];
        println!("array is {:?}", arr);
        println!("array size is :{}",arr.len());
       
        for val in arr.iter(){
             println!(":{}",val);
        }

    }
