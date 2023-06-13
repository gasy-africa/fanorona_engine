
# :two: Coroutine

```rust
// Define a Coroutine structure
struct Coroutine {
    state: u32,
}

impl Coroutine {
    fn new() -> Self {
        Coroutine { state: 0 }
    }

    fn resume(&mut self) {
        match self.state {
            0 => {
                println!("Coroutine started");
                self.state = 1;
            }
            1 => {
                println!("Coroutine resumed");
                self.state = 2;
            }
            2 => {
                println!("Coroutine finished");
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    // Create a new coroutine
    let mut coroutine = Coroutine::new();

    // Resume the coroutine
    coroutine.resume();
    coroutine.resume();
    coroutine.resume();
    coroutine.resume();
}
```

## [:back:](../)
