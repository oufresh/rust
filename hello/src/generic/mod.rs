mod definitions;
mod traits;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// generic largest function
fn largest<T>(list: &[T]) -> &T {
    let largest = &list[0];

    /*for item in list {
        if item > largest {
            largest = item;
        }
    }*/

    largest
}

///
/// Test generic
///
pub fn test_generic() {
    println!("Test struct with generic");
    definitions::use_struct();
    definitions::test_struct_methods();

    println!("Test largest obj with generic");
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    println!("\n");
}

pub fn test_traits() {

    println!("");
    println!("Test traits");
    traits::test_trait_summarize();
    println!("");
}