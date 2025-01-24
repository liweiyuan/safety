use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::collections::HashMap;

// 1. Memory Safety with Ownership and Borrowing
struct SafeContainer<T> {
    data: Vec<T>,
}

impl<T> SafeContainer<T> {
    // Constructor takes ownership of data
    fn new() -> Self {
        SafeContainer {
            data: Vec::new()
        }
    }

    // Demonstrates borrowing - returns reference without transferring ownership
    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // Mutable borrowing example
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

// 2. Thread Safety Examples
struct ThreadSafeCounter {
    // Atomic types for lock-free concurrent access
    atomic_count: AtomicUsize,
    // Mutex for when we need exclusive access to complex data
    protected_data: Mutex<HashMap<String, i32>>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            atomic_count: AtomicUsize::new(0),
            protected_data: Mutex::new(HashMap::new()),
        }
    }

    fn increment(&self) -> usize {
        // Lock-free atomic increment
        self.atomic_count.fetch_add(1, Ordering::SeqCst)
    }
}

// 3. Error Handling with Result and Option
#[derive(Debug)]
struct CustomError(String);

fn divide(numerator: f64, denominator: f64) -> Result<f64, CustomError> {
    if denominator == 0.0 {
        Err(CustomError("Division by zero".to_string()))
    } else {
        Ok(numerator / denominator)
    }
}

// 4. Safe Abstractions with Types
#[derive(Debug)]
struct NonNegativeNumber(u32);

impl NonNegativeNumber {
    // Type safety: Constructor ensures value constraint
    fn new(value: i32) -> Option<Self> {
        if value >= 0 {
            Some(NonNegativeNumber(value as u32))
        } else {
            None
        }
    }

    fn get(&self) -> u32 {
        self.0
    }
}

// 5. Performance Optimization Patterns
struct CachedValue<T> {
    value: Option<T>,
    computation: Box<dyn Fn() -> T>,
}

impl<T: Clone> CachedValue<T> {
    fn new<F>(computation: F) -> Self 
    where F: 'static + Fn() -> T
    {
        CachedValue {
            value: None,
            computation: Box::new(computation),
        }
    }

    fn get(&mut self) -> T {
        // Lazy evaluation and caching
        if let Some(ref value) = self.value {
            value.clone()
        } else {
            let result = (self.computation)();
            self.value = Some(result.clone());
            result
        }
    }
}

fn main() {
    // 1. Memory Safety Example
    let mut container = SafeContainer::new();
    container.push(42);
    println!("First element: {:?}", container.get(0));

    // 2. Thread Safety Example
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 3. Error Handling Example
    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    // 4. Type Safety Example
    if let Some(num) = NonNegativeNumber::new(42) {
        println!("Valid non-negative number: {}", num.get());
    }

    // 5. Performance Optimization Example
    let mut cached = CachedValue::new(|| {
        println!("Expensive computation");
        42
    });

    println!("First call: {}", cached.get());  // Will compute
    println!("Second call: {}", cached.get());  // Will use cached value
}

