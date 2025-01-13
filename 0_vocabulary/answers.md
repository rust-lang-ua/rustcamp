Chapter 0: Building Up a Vocabulary
========================================

#### What memory model Rust has? Is it single-threaded or multi-threaded? Is it synchronous or asynchronous? What are the memory layouts of Box and Vector? What are a heap and a stack? Where, but on heap and stack data could live in RAM?
1. Rust does not yet have a defined memory model. Even though this is an under-defined place in the language, we could say that it uses a strong ownership-based memory model. Key aspects include:
   - **Ownership:** each value has a single owner. When ownership transfers, the original owner can no longer access the value.
   - **Borrowing:** references allow temporary, non-owning access to data (`&` for immutable, `&mut` for mutable).
   - **Lifetimes:** ensures references remain valid for the right scope, preventing dangling references.
2. Rust supports both single-threaded (by default) and multi-threaded programming (availible via `std::thread`).
3. Rust supports both synchronous (by default) and asynchronous programming (availible via `async`/`await`). Rust's async model requires an executor or runtime (like Tokio or async-std) to poll futures and drive their progress.
4. - `Box` pointer lives on a stack, but stores a single value on the heap.
   - `Vec` struct lives on a stack, but stores multiple contiguous elements on the heap.
5. The **heap** and **stack** are two types of memory.
   - The **stack** is a fast region of memory used by default for storing local variables, function calls, and control flow data. It is limited in size.
   - The **heap** is slower, and is explicitly allocated by your program for objects whose size may not be known at compile time. It’s unlimited in size and is globally accessible.
6. In Rust, memory is organized into **stack**, **heap** and a special **static** memory region.
   - **Static** memory region is for data that is declared as variables within a function. The location of this memory never changes for the duration of a function call; because of this compilers can optimize code so stack data is very fast to access.
     
#### What runtime Rust has? Does it use a GC (garbage collector)?
- Rust has a minimal runtime and has **no Garbage Collector**, since memory is managed at compile time via ownership and borrowing rules.
  
#### What is special about slice? What is the layout of Rust standard data types? Difference between fat and thin pointers?
1. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership. Unique characteristics of slices:
   - **Dynamically Sized:** Slices don’t have a fixed size known at compile time. They provide a view into a contiguous sequence of elements rather than owning the data.
   - **Borrowed References:** Slices borrow data rather than own it. They are often created from arrays or Vec collections.
   - **Pointer + Length:** A slice is represented internally by a pointer to the first element and a length.
   - **Efficient Access:** Slices allow efficient access to parts of arrays or vectors without copying data.
   - **Useful Methods:** Slices offer many methods for manipulation, such as .len(), .iter(), and .sort()
2. The layout of a type is its size, alignment, and the relative offsets of its fields.
- Primitive Data Layout

| **Type**       | **Size** (in bytes) | **Layout**        |
|----------------|---------------------|-------------------|
| `bool`         | 1                   | `0` (false) or `1` (true) |
| `char`         | 4                   | UTF-32 encoding  |
| `u8`, `i8`     | 1                   | Raw byte         |
| `u16`, `i16`   | 2                   | Little/Big-endian |
| `u32`, `i32`   | 4                   | Little/Big-endian |
| `u64`, `i64`   | 8                   | Little/Big-endian |
| `u128`, `i128` | 16                  | Little/Big-endian |
| `f32`          | 4                   | IEEE-754         |
| `f64`          | 8                   | IEEE-754         |

`usize` and `isize` have a size big enough to contain every address on the target platform.
    
- Pointers and References Layout
  - Pointers and references have the same layout. Mutability of the pointer or reference does not change the layout.
Pointers to sized types have the same size and alignment as `usize`. Pointers to unsized types are sized. The size and alignment is guaranteed to be at least equal to the size and alignment of a pointer.
https://doc.rust-lang.org/reference/type-layout.html
  - References are typically represented as a simple memory address (similar to raw pointers).
They carry a lifetime that the compiler uses to enforce safety rules but do not affect the runtime layout.
  - Raw Pointers `(*const T, *mut T)`:
     - These are similar to pointers in C/C++.
     - They have no ownership or safety guarantees and are often used in unsafe code.
     - Layout:
     - Typically, a raw pointer is just the memory address (usually the size of usize). 
  - Smart Pointers (Box<T>, Rc<T>, Arc<T>):
     - These are abstractions over raw pointers that provide ownership semantics and other features.
     - `Box<T>`: Owns the memory it points to and automatically deallocates it.
     - `Rc<T>/Arc<T>`: Reference-counted pointers for shared ownership.
     - The data is stored on a heap.

- Array Layout
  - An array of `[T; N]` has a size of `size_of::<T>() * N` and the same alignment of `T`. Arrays are laid out so that the zero-based `nth` element of the array is offset from the start of the array by `n * size_of::<T>()` bytes. They are stored in a heap.
- Slice Layout
  - Pointers to slices (e.g., &[T], Box<[T]>) consist of two components:
     - A pointer to the first element of the slice.
     - The length of the slice (in elements, not bytes).
  - This allows slices to reference a dynamically sized portion of an array or collection. They are also stored in a heap.
- str Layout
  - String slices are a UTF-8 representation of characters that have the same layout as slices of type [u8].
- Tuple Layout
  - Tuples are laid out according to the Rust representation.
  - The exception to this is the unit tuple (()), which is guaranteed as a zero-sized type to have a size of 0 and an alignment of 1.
- Trait Object Layout
  - Trait objects have the same layout as the value the trait object is of.
  - Note: This is about the raw trait object types, not pointers (`&dyn Trait`, `Box<dyn Trait>`, etc.) to trait objects.
3. Thin pointers
   - Thin pointers contain only the memory address of the data they reference.
   - The size of a thin pointer is only the platform's word size (e.g., 8 bytes on a 64-bit system).
   -  Examples:
        - `&T` (reference to a concrete type)
        - `*const T` and `*mut T` (raw pointers)
        - `Box<T>`
```
let x = 42;
let thin_ptr: &i32 = &x; // Thin pointer to an i32
```
4. Fat pointers
   - Fat pointers store additional metadata alongside the memory address.
   - Double the size of a thin pointer (e.g., 16 bytes on a 64-bit system).
   - Used for:
        -  Slices (`&[T]` or `&str`) — include a length.
        -  Trait Objects (`&dyn Trait`) — include a pointer to the vtable (virtual method table).
```
let arr = [1, 2, 3];
let fat_ptr: &[i32] = &arr; // Slice Pointer
```
#### Why does Rust have &str and String types? How do they differ? When should you use them? Why str slice coexists with slice? What is the difference between String and Vec?
1. `&str` and `String` differences
   - `&str` is a string slice — a reference to a sequence of UTF-8 encoded bytes. It is immutable, borrowed and fixed in size.
   - `&str` avoids heap allocation and copying, making it lightweight for cases where ownership isn't needed.
   - `String` is a heap-allocated, growable string type. It mutable, owned and resizable.
   - `String` allows you to dynamically manage string content when mutability and ownership are necessary.
  
| **Aspect**           | **`&str`**                          | **`String`**                       |
|----------------------|--------------------------------------|------------------------------------|
| **Ownership**        | Borrowed reference                  | Owned, heap-allocated              |
| **Mutability**       | Immutable                           | Mutable (if declared `mut`)        |
| **Memory Location**  | Stack or static memory              | Heap memory                        |
| **Size**             | Fixed at runtime                    | Dynamic (can grow or shrink)       |
| **Use Case**         | Referencing existing string data    | Creating and managing string data  |

2. `String` and `Vec` differences
   - Use `String` when working with text and you need UTF-8 guarantees and string-specific operations.
   - Use `Vec` when dealing with binary data or byte sequences that may not be valid UTF-8.
     
#### What static typing means? What are the benefits of using it? Weak vs strong typing? Implicit vs explicit typing?
1. Static Typing and its Benefits
   - Static typing is a type system where variable types are determined at compile time. This means that every variable, function parameter, and return value has a known and fixed type during compilation.
   - Static typing provides safety, efficiency, and reliability by enforcing type rules at compile time. It helps catch errors early and allows for better performance and tooling support, making it ideal for large-scale, performance-critical, or safety-focused applications.
2. Weak vs Strong Typing
   - Strong Typing enforces strict type rules, promoting safety and predictability.
   - Weak Typing allows more implicit conversions, offering flexibility at the cost of potential bugs.
3. Implicit vs Explicit Typing
   - Implicit typing can look concise and readable, but may lead to ambiguity and debugging complexity.
   - Explicit typing provides clarity and safety, but can also look verbose and be less flexible.
     
#### What are generics and parametric polymorphism? Which problems do they solve?
1. Generics `<T>` (Generic Data Types)
   - We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
2. Parametric Polymorphism
   - Parametric polymorphism refers to the ability to write code that can operate on multiple types while being type-agnostic.
3. Problems they solve
   - They allow you to write reusable code that works with multiple types by using type placeholders, which helps reduce code duplication.
     
#### What are nominative typing and structural typing? What is the difference?
- **Nominative Typing (Nominal Typing):** Types are compatible only if they share the same name. The type identity is based on its name.
-  **Structural Typing:** Types are compatible if they share the same structure, regardless of their names. If the fields (or methods) match, the types are treated as compatible.
   
#### What are traits? How are they used? How do they compare to interfaces? What are auto trait and blanket impl? Uncovered type? What are marker traits?
1. Traits
   - In Rust, a trait is a way to define shared behavior (methods or functionality) that can be implemented by multiple types. Traits enable polymorphism and help ensure code reuse and type safety.
```
// defining a trait

trait Speak {
    fn speak(&self);
}
```
```
// implementing a trait

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
```
```
// using traits

fn make_speak<T: Speak>(animal: &T) {
    animal.speak();
}

let dog = Dog;
let cat = Cat;

make_speak(&dog); // Output: "Woof!"
make_speak(&cat); // Output: "Meow!"
```
2. Traits vs Interfaces
   - Traits in Rust are similar to interfaces in languages like Java, but traits allow for multiple inheritance, default implementations, and associated types, while interfaces typically offer simpler polymorphism with dynamic dispatch.
3. Auto Traits
   - Auto traits, like `Send` or `Sync` in the standard library, are marker traits that are automatically implemented for every type, unless the type, or a type it contains, has explicitly opted out via a negative impl. (Negative impls are separately controlled by the negative_impls feature.)
4. Blanket Impls
   - Any implementation where a type appears uncovered. `impl<T> Foo for T`, `impl<T> Bar<T> for T`, `impl<T> Bar<Vec<T>> for T`, and `impl<T> Bar<T> for Vec<T>` are considered blanket impls. However, `impl<T> Bar<Vec<T>> for Vec<T>` is not a blanket `impl`, as all instances of `T` which appear in this `impl` are covered by `Vec`.
5. Uncovered type
   - A type which does not appear as an argument to another type. For example, `T` is uncovered, but the `T` in `Vec<T>` is covered. This is only relevant for type arguments.
6. Marker traits
   - Marker traits are traits with no methods, used to tag types with specific properties.
        - `fn is_copy<T: Copy>() {}` it means that it can be `Clone`d by using bitwise copy
        - `fn is_send<T: Send>() {}` it means that value of type `T` can be send across thread boundary
        - `fn is_sync<T: Sync>() {}` it means that value of type `T` can be shared between threads (that is `&T` is `Send`)
        - `fn is_sized<T: Sized>() {}` means that size of type `T` is known at compile time
        - `fn is_unpin<T: Unpin>() {}` means that value of type `T` can be moved after it was pinned
          
#### What are static and dynamic dispatches? Which should you use, and when? What is monomorphization?
1. Static Dispatch
   - Static dispatch is the process where the method to call is determined at compile-time. (by default in rust)
2. Dynamic Dispatch
   - Dynamic dispatch occurs when the method to call is determined at runtime.
3. Monomorphization
   - Monomorphization is a process in Rust where the compiler generates a specific implementation of a generic function or type for each concrete type used with it.

#### What are a crate, a module, and a package in Rust? How do they differ? How are they used? What is a workspace?

1. Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
    - Crates: The smallest unit of code compilation in Rust, which can be either a binary (executable) or a library.
    - Modules and use: Logical divisions within a crate, used to organize and encapsulate code with privacy boundaries.
    - Packages: A collection of one or more crates that provide a set of functionality and include a Cargo.toml file for metadata and dependencies.
    - Paths: Namespaces used to locate and access items like functions, structs, or modules within a crate or module hierarchy.
2. A workspace is a collection of one or more packages, called workspace members, that are managed together.
The key points of workspaces are:
    - Common commands can run across all workspace members, like cargo check --workspace.
    - All packages share a common Cargo.lock file which resides in the workspace root.
    - All packages share a common output directory, which defaults to a directory named target in the workspace root.
    - Sharing package metadata, like with workspace.package.
    - The [patch], [replace] and [profile.*] sections in Cargo.toml are only recognized in the root manifest, and ignored in member crates’ manifests.
      
#### What is cloning? What is copying? How do they compare? What is trait Drop for? What is special about this trait?
1. Differences Between `Copy` and `Clone`

| **Aspect**           | **Copy**                                | **Clone**                                |
|----------------------|-----------------------------------------|------------------------------------------|
| **Trait**            | `Copy`                                 | `Clone`                                  |
| **Type of Copy**     | Shallow copy (bitwise copy)            | Deep copy (may involve heap allocation)  |
| **Invocation**       | Implicit (via assignment)              | Explicit (via `clone()` method)          |
| **Cost**             | Very cheap                             | Can be expensive (heap, computation)     |
| **Use Case**         | Simple types (primitives, small structs)| Complex types (e.g., `String`, `Vec`)    |
| **Ownership**        | Both original and copy are valid       | Original and clone are independent       |

2. Drop trait
    - The Drop trait only has one method: drop, which is called automatically when an object goes out of scope. The main use of the Drop trait is to free the resources that the implementor instance owns.
      
#### What is immutability? What is the benefit of using it? What is the difference between immutability and const?
1. Immutability
   - Immutability means that a variable's value cannot be changed after it is assigned. In Rust, by default, all variables are immutable. To declare a mutable variable, you need to explicitly use the `mut` keyword.
   - Immutability makes code safe, predictable, easier to debug and optimize for the compiler
2. Immutability vs Const
   - **Immutability** refers to variables that cannot change after being assigned, evaluated at **runtime** (`let`), while `const` defines compile-time constants that must be known and fixed at **compile-time**.

#### What are move semantics? What are borrowing rules? What is the benefit of using them?
1. Move Semantics (`y = x`)
   - Move semantics refers to the way Rust transfers ownership of a value from one variable to another. When a value is moved, the original owner can no longer use the value. This ensures that each piece of data has a single owner, preventing data races and ensuring memory safety.

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of `s1` is moved to `s2`

    // println!("{}", s1); // Error: `s1` is no longer valid after the move
    println!("{}", s2);
}
```
2. Borrowing (`&T`, `&mut T`)
   - Borrowing allows you to temporarily use a value without taking ownership. This is crucial for allowing multiple parts of your code to access data safely and efficiently.
   - Rule 1: You can have either:
      - Any number of immutable borrows (`&T`)
      - Or one mutable borrow (`&mut T`)
      - But not both at the same time
   - Rule 2: References must always be valid. They cannot outlive the data they refer to.

#### What is RAII? How is it implemented in Rust? What is the benefit of using it?
1. RAII (Resource Acquisition Is Initialization)
   - RAII is a programming paradigm where resource management (e.g., memory, file handles, sockets) is tied to the lifetime of an object. When an object is created, it acquires the necessary resource, and when the object goes out of scope, it releases the resource automatically. RAII ensures that resources are properly cleaned up and helps prevent common issues like resource leaks.
2. RAII in Rust
   - Rust enforces RAII using its **ownership model** and the **Drop trait**. When a value goes out of scope, Rust automatically calls the drop method to release any associated resources.

#### What are lifetimes? Which problems do they solve? Which benefits do they provide?
Lifetimes are Rust's way of ensuring that references are valid for as long as they need to be. They describe the scope during which a reference remains valid, helping Rust enforce memory safety by preventing dangling references (references to invalid or freed data).
In Rust, the compiler uses lifetimes to track how long references live and ensure they don't outlive the data they point to.
```
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```
#### What is an iterator? What is a collection? How do they differ? How are they used?
1. Itertator
   - An iterator in Rust is an object that allows you to sequentially access elements in a collection (or any iterable structure) without exposing its internal representation. The iterator produces items one by one and provides a way to process them lazily.
   - Rust's iterator functionality is provided through the Iterator trait, which defines the core method:
```
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
- next returns:
    - Some(item) if there are more elements to yield.
    - None when the iterator is exhausted.

Example of an Iterator:
```
fn main() {
    let numbers = vec![1, 2, 3];

    let mut iter = numbers.iter(); // Create an iterator over `numbers`

    println!("{:?}", iter.next()); // Output: Some(1)
    println!("{:?}", iter.next()); // Output: Some(2)
    println!("{:?}", iter.next()); // Output: Some(3)
    println!("{:?}", iter.next()); // Output: None (iterator is exhausted)
}
```
Common Methods on Iterators
- `map`: Transforms each item.
- `filter`: Filters items based on a condition.
- `collect`: Consumes the iterator and collects items into a collection.
- `for_each`: Applies a function to each item.
```
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // Output: [2, 4, 6, 8]
}
```
2. Collection
   - A collection is a data structure that can store multiple values. Collections in Rust are flexible and allow dynamic allocation and management of elements.
   - Rust's standard collections are found in the std::collections module.

Common Collections in Rust:
- `Vec<T>` (Vector):
   - A dynamically-sized, growable array.
```
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
println!("{:?}", numbers); // Output: [1, 2]
```
- `HashMap<K, V>`:
   - A key-value store where each key maps to a value.
```
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
println!("{:?}", scores); // Output: {"Alice": 10, "Bob": 20}
```
- `HashSet<T>`:
   - A collection of unique values.
```
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(2); // Duplicate, won't be added
println!("{:?}", set); // Output: {1, 2}
```
- `LinkedList<T>`:
   - A doubly-linked list.
```
use std::collections::LinkedList;
let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
println!("{:?}", list); // Output: [1, 2]
```
- `VecDeque<T>`:
   - A double-ended queue optimized for pushing/popping elements from both ends.
```
use std::collections::VecDeque;
let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(0);
println!("{:?}", deque); // Output: [0, 1]
```
#### What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
1. Macros
   - Macros in Rust are a way to write code that writes other code (metaprogramming).
   - They enable developers to generate code at compile time, which can reduce boilerplate, simplify repetitive patterns, and improve efficiency.
2. Declarative Macros (`macro_rules!`)
   - Declarative macros use the `macro_rules!` syntax and work similarly to pattern matching.
   - They allow developers to define patterns and how those patterns should expand into code.

Parameterized Declarative Macro:
```
macro_rules! square {
    ($x:expr) => {
        println!("The square of {} is {}", $x, $x * $x);
    };
}

fn main() {
    square!(4); // Expands to println!("The square of 4 is 16");
}
```
3. Procedural Macros
   - Procedural macros are more powerful and customizable than declarative macros. They operate on Rust code as input and produce Rust code as output. Procedural macros are defined in a separate crate and are executed at compile time.
   - Rust macros can expand to more complex expressions than regular functions, making them useful for tasks that require generating code dynamically based on input.
   - Procedural macros are divided into three categories:
      1. Custom derive Macros
      2. Attribute Macros
      3. Function-Like Macros
   - To create procedural macros, you'll need the proc_macro crate.

- Custom derive Macros
   - These macros allow you to automatically implement traits for structs or enums.
```
// In a procedural macro crate (e.g., `my_macros`)
use proc_macro::TokenStream;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // Parse the input and generate code
    input
}

// In your main crate
#[derive(MyTrait)]
struct MyStruct;
```
- Attribute Macros
   - Attribute macros allow you to define custom attributes for functions, structs, or enums.
```
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Modify the item with custom logic
    item
}

#[my_attribute]
fn example() {
    println!("This function has a custom attribute!");
}
```
- Function-Like Macros
   - Function-like macros look similar to declarative macros but are more powerful. They take a TokenStream and return a TokenStream.
```
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    input
}

my_macro!(some input);
```
4. Use Cases
> Macros generate code at compile time to reduce boilerplate and automate repetitive tasks.
- Use Declarative Macros when:
    - You need simple code generation or pattern matching.
    - You're avoiding external dependencies.
    - You want easy-to-read syntax for common tasks.
- Use Procedural Macros when:
    - You need more powerful and customizable code generation.
    - You're implementing custom traits, attributes, or DSLs.
    - You need to manipulate Rust syntax trees directly.
  
#### How code is tested in Rust? Where should you put tests and why?
1. Tests
   - Rust has built-in support for testing, enabling developers to write unit tests, integration tests, and documentation tests.
   - The testing framework is part of Rust's standard library and doesn't require additional dependencies.
2. Unit Tests
   - Unit tests test individual functions or modules in isolation. They typically focus on small, specific pieces of functionality.
   - Place unit tests inside the same file as the code you're testing.
   - Use the #[cfg(test)] attribute to ensure tests are only compiled when running tests.
   - To run use `cargo test`
```
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```
3. Integration Tests
   - Integration tests verify how multiple parts of your code work together. These tests are kept separate from the main code and test public APIs.
   - Create a tests directory at the root of your project.
   - Each file in the tests directory is a separate integration test.
   - To run a specific integration test use `cargo test --test integration_test`
```
// tests/integration_test.rs

use my_project::add; // Import the function from your library

#[test]
fn test_add_integration() {
    assert_eq!(add(4, 6), 10);
}
```
4. Documentation Tests
    - Documentation tests ensure that the examples in your documentation comments are correct and up-to-date.
    - Documentation tests are written as part of your Rust doc comments (using ///). When you include code blocks in the comments, Rust can test them.
    - To run documentatation tests use `cargo test --doc`
```
/// Adds two numbers together.
///
/// ```
/// use my_project::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Is Rust an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance? Is Rust a functional language? What variance rules does Rust have?
1. Is Rust an OOP language?
    - Not strictly, but it supports OOP concepts like encapsulation, abstraction, and polymorphism via traits and composition.
2. Does Rust support SOLID/GRASP?
    - Yes, these principles can be effectively implemented in Rust using traits, composition, and modules.
3. Does Rust have inheritance?
    - No, Rust favors composition and traits over inheritance.
4. Is Rust a functional language?
    - Rust supports functional programming features while being a multi-paradigm language.
5. What are Rust's variance rules?
    - Rust supports covariant, contravariant, and invariant types to ensure memory safety and avoid unsoundness.
