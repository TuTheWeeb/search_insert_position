
// The function tha will run the code
fn search_insert_position (arr: &[i32;5], target: &i32) -> usize {
    
    // Declaring the return variable and the lenght of the array
    let mut _result: usize = 0;
    let arr_len: usize = arr.len();

    // For loop, right now is O(n)
    for x in 0..arr_len {
        
        // Check to see if the target is already in the array
        if arr[x] == *target {
        
            _result = x;
            break;
        } 
        
        // If it's not in the array, then check to see where it should be
        else if arr[x] > *target {
            
            // Check to see if the index is 0
            if arr[x] == 0 {
                _result = 0;
            }

            // If it's not then put _result as x 
            else {
                _result = x;
            }

            break;
        }
    }

    return _result;
}

fn main() {
    
    // Declare the array and the target   
    let arr: [i32; 5] = [1,3,4,5,6];
    let target: i32 = 2;

    // Print the result
    println!("{}", search_insert_position(&arr, &target));
}
