fn main() {
    for n in 0..10 {
        println!("{}", fib_recursive(n));
    }

    for n in 0..10 {
        println!("{}", fib_iterative(n));
    }
}

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        n
    } else  {
        fib_recursive(n-1) + fib_recursive(n-2)
    }
}

fn fib_iterative(n:u32) -> u32 {
    if n < 2 {
        n
    } else {
        let mut f1 = 0;
        let mut f2 = 1;

        for _ in 2..=n {
            (f1,f2) = (f2,f1+f2);
        }
        f2
    }
}