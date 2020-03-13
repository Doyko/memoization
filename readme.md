# Memoization

This code aims at implementing a function in Rust that add a memory to another function. This function takes a pure function f as argument and returns a function that behaves almost exactly like f, except that it only calls the original function once for every argument, stores the result internally, and subsequently returns this stored result every time it’s called with the same argument.

## Files

- `scr/main.rs` : the main program with the function.
- `scr/test.rs` : the unit tests.

## Example

When the program calls the original function, it will print `Base(arg)`, and when it calls the generate function, it will print `New (arg)`.

Main.rs :

```Rust
let mut g = memoization(f);

println!("function g :\n");

g(1);
g(1);
g(0);
g(1);
g(0);

let mut i = memoization(j);

println!("\nfunction j :\n");
i(1);
i(1);
i(0);
```

Output :

```Bash
function g :

New (1)
Base(1)
New (1)
New (0)
Base(0)
New (1)
New (0)

function j :

New (1)
Base(1)
New (1)
New (0)
Base(0)
```

This example shows that for each argument and for each function, the original function is called only once, even if the generated function is called again.
