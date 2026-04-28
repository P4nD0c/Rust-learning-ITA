// esercizio dove è richiesto calcolare la lunghezza di collatz.

fn collatz_length(mut n: u32) -> i32 {
    let mut l = 1;
    while n>1{
        if n%2==0{
            n /= 2;
        } else {
            n = (3*n) +1
        }
        l += 1
    }
    l
}


fn main(){
    println!("Collatz Lenght: {}", collatz_length(14)) // RESULT = 15
}
