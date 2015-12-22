// Implentation of stack using custom datatype

/// The standard `Stack` type for storing values of type `T`. 
pub enum Stack<T> {
    /// Empty stack
    Nil,
    /// Recursive case, implementation following Okasaki's book
    Cons(T, Box<Stack<T>>)
}


/// Constructs a new `Stack<T>`
///
/// # Examples
///
/// ```
/// use stack;
///
/// let s: stack::Stack<u64> = stack::new_stack();
/// ```
///
/// A new stack is always empty.
/// ```
/// use stack;
///
/// assert!(stack::is_empty(stack::new_stack()));
/// ```
pub fn new_stack<T>() -> Stack<T> {
    Stack::Nil
}
    
/// Checks if a stack `s` is empty.
///
/// # Examples
///
/// ```
/// use stack;
///
/// let b: bool = stack::is_empty::<u64>(stack::Stack::Nil);
/// ```
pub fn is_empty<T>(s: Stack<T>) -> bool {
    match s {
        Stack::Nil => true,
        _   => false,
    }    
}

pub fn cons<T>(x: T, s: Stack<T>) -> Stack<T> {
    Stack::Cons(x, Box::new(s))
}


pub fn head<T>(s: Stack<T>) -> Option<T> {
    match s {
        Stack::Nil => None,
        Stack::Cons(x, _) => Some(x),
    }
}

pub fn tail<T>(s: Stack<T>) -> Option<Stack<T>> {
    match s {
        Stack::Nil => None,
        Stack::Cons(_, s) => Some(*s),
    }
}

// Tests follow
#[test]
fn empty_new_stack() {
    let s: Stack<u64> = new_stack();
    assert!(is_empty(s));
}

#[test]
fn add_not_empty() {
    let mut s: Stack<u64> = new_stack();
    s = cons(2, s);
    assert!(! is_empty(s));
}

#[test]
fn empty_tail() {
    let mut s: Stack<u64> = new_stack();
    s = cons(2, s);
    let opt = tail(s);
    match opt {
        None => assert!(false),
        Some(t) => assert!(is_empty(t)),
    }
}
