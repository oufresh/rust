# String vs str
As a Rust newbie, I was confused by the different ways used to represent strings. The “References and Borrowing” chapter of the Rust book uses three different types of string variables in the examples: String, &String and &str.

Let’s start with the difference between str and String: **String** is a growable, heap-allocated data structure whereas **str** is an immutable fixed-length string somewhere in memory.

### String
If you’re a Java programmer, a Rust String is semantically equivalent to StringBuffer (this was probably a factor in my confusion, as I’m so used to equating String with immutable). As such, a String maintains a length and a capacity whereas a str only has a len() method. As an example:

let mut s = String::from("Hello, Rust!");
println!("{}", s.capacity()); // prints 12
s.push_str("Here I come!");
println!("{}", s.len()); // prints 24

let s = "Hello, Rust!";
println!("{}", s.capacity()); // compile error: no method named `capacity` found for type `&str`
println!("{}", s.len()); // prints 12

### &str
You can only ever interact with str as a borrowed type aka &str. This is called a string slice, an immutable view of a string. This is the preferred way to pass strings around, as we shall see.

### &String
This is a reference to a String, also called a borrowed type. This is nothing more than a pointer which you can pass around without giving up ownership. Turns out a &String can be coerced to a &str:

fn main() {
    let s = String::from("Hello, Rust!");
    foo(&s);
}

fn foo(s: &str) {
    println!("{}", s);
}
In the above example, foo() can take either string slices or borrowed Strings, which is super convenient. As such, you almost never need to deal with &Strings. The only real use case I can think of is if you want to pass a mutable reference to a function that needs to modify the string:

fn main() {
    let mut s = String::from("Hello, Rust!");
    foo(&mut s);
}

fn foo(s: &mut String) {
    s.push_str("appending foo..");
    println!("{}", s);
}
### Summary

Prefer &str as a function parameter or if you want a read-only view of a string; String when you want to own and mutate a string.

str data can live on the heap, stack or in the binary. This excellent stackoverflow answer explains the scenarios for each. ↩


[http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html]

String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.

str is an immutable1 sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice". A slice [https://doc.rust-lang.org/book/ch04-03-slices.html] is just a view onto some data, and that data can be anywhere, e.g.

- In static storage: a string literal "foo" is a &'static str. The data is hardcoded into the executable and loaded into memory when the program runs.
- Inside a heap allocated String: String [https://doc.rust-lang.org/std/string/struct.String.html#deref] dereferences to a &str view of the String's data.
- On the stack: e.g. the following creates a stack-allocated byte array, and then gets a view of that data as a &str [https://doc.rust-lang.org/std/str/fn.from_utf8.html]:

```rust
use std::str;

let x: &[u8] = &[b'a', b'b', b'c'];
let stack_str: &str = str::from_utf8(x).unwrap();
```

In summary, use String if you need owned string data (like passing strings to other threads, or building them at runtime), and use &str if you only need a view of a string.

This is identical to the relationship between a vector Vec<T> and a slice &[T], and is similar to the relationship between by-value T and by-reference &T for general types.

----------------------------------------------------------------------------------------------------------------------------------------------------------------

1 A str is fixed-length; you cannot write bytes beyond the end, or leave trailing invalid bytes. Since UTF-8 is a variable-width encoding, this effectively forces all strs to be immutable in many cases. In general, mutation requires writing more or fewer bytes than there were before (e.g. replacing an a (1 byte) with an ä (2+ bytes) would require making more room in the str). There are specific methods that can modify a &str in place, mostly those that handle only ASCII characters, like make_ascii_uppercase [https://doc.rust-lang.org/std/primitive.str.html#method.make_ascii_uppercase].

2 Dynamically sized types allow things like Rc<str> for a sequence of reference counted UTF-8 bytes since Rust 1.2. Rust 1.21 allows easily creating these types.