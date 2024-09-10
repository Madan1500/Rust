// String methods in rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("{} {}", hello, world);

// string predefined methods
let s = String::from("hello world");

// length of string
let len = s.len();
// push a character to string
s.push('!');
// push a string to string
s.push_str(" world");
// pop a character from string
let popped = s.pop();
// check if string is empty
let is_empty = s.is_empty();
// check if string contains a substring
let contains = s.contains("world");
// replace a substring with another substring
let replaced = s.replace("world", "there");
// split string by whitespace
let split: Vec<&str> = s.split_whitespace().collect();
// split string by a character
let split: Vec<&str> = s.split(' ').collect();
// create a string from a vector of strings
let s = split.join(" ");
// check if string starts with a substring
let starts_with = s.starts_with("hello");
// check if string ends with a substring
let ends_with = s.ends_with("there");
// find a substring in a string
let found = s.find("there");
// get a substring from a string
let hello = &s[0..5];
// get a character from a string
let h = s.chars().nth(0).unwrap();
// get a character from a string by index
let h = s.chars().nth(0).unwrap();

// Vector methods in rust
let mut v = vec![1, 2, 3, 4, 5];

// length of vector
let len = v.len();
// push an element to vector
v.push(6);
// pop an element from vector
let popped = v.pop();
// check if vector is empty
let is_empty = v.is_empty();
// get an element from vector by index
let third = v[2];
// get an element from vector by index with error handling
let third = v.get(2).unwrap();
// set an element in vector by index
v[2] = 10;
// append a vector to another vector
v.append(&mut vec![6, 7, 8]);
// remove an element from vector by index
let removed = v.remove(2);
// iterate over vector
for i in &v {
    println!("{}", i);
}
// iterate over vector with index
for (i, e) in v.iter().enumerate() {
    println!("{} {}", i, e);
}
// create a vector from a range
let v: Vec<i32> = (1..5).collect();
// create a vector from a range with a step
let v: Vec<i32> = (1..5).step_by(2).collect();
// create a vector from a range in reverse
let v: Vec<i32> = (1..5).rev().collect();
// create a vector from a range in reverse with a step
let v: Vec<i32> = (1..5).rev().step_by(2).collect();
// sort a vector
v.sort();
// sort a vector in reverse
v.sort_by(|a, b| b.cmp(a));
// reverse a vector
v.reverse();
// find an element in vector
let found = v.contains(&3);
// find an element in vector by a predicate
let found = v.iter().any(|&x| x == 3);
// find an element in vector by a predicate with index
let found = v.iter().enumerate().any(|(i, &x)| i == 2 && x == 3);

// Hashmap methods in rust
use std::collections::HashMap;

let mut map = HashMap::new();

// insert a key-value pair into hashmap
map.insert("key", "value");
// get a value from hashmap by key
let value = map.get("key");
// remove a key-value pair from hashmap by key
let removed = map.remove("key");
// check if hashmap is empty
let is_empty = map.is_empty();
// length of hashmap
let len = map.len();
// iterate over hashmap
for (key, value) in &map {
    println!("{} {}", key, value);
}
// check if hashmap contains a key
let contains = map.contains_key("key");
// get a value from hashmap by key with default value
let value = map.get("key").unwrap_or(&"default");
// get a value from hashmap by key with default value from a closure
let value = map.get("key").unwrap_or_else(|| "default");
// create a hashmap from a vector of key-value pairs
let map: HashMap<_, _> = vec![("key", "value")].into_iter().collect();
// create a hashmap from two vectors
let keys = vec!["key"];
let values = vec!["value"];
let map: HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
// merge two hashmaps
let mut map1 = HashMap::new();
let mut map2 = HashMap::new();
map1.insert("key1", "value1");
map2.insert("key2", "value2");
map1.extend(map2);
// clear hashmap
map.clear();

// HashSet methods in rust
use std::collections::HashSet;

let mut set = HashSet::new();

