pub fn fibonacci(n: u32) -> u32{
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
} 

// To run : cargo run
fn main(){
    let n: u32 = 4;
    println!("{} => {}", n, fibonacci(n));
}

//init test 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(4);
        assert_eq!(result, 3);
    }
}