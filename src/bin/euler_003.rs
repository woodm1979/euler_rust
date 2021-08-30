// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

use ::euler_rust::incrementer;

fn main() {
    let foo = (5, "dog");
    let derp = (2, "yo");

    if derp < foo {
        println!("Derp < Foo: {:?} {:?}", foo, derp)
    } else {
        println!("Foo < Derp: {:?} {:?}", foo, derp)
    }

    let max: u64 = 30;
    // let primes = Primes::new().take_while(|i| i <= &max);
    // let primes: Vec<_> = primes.collect();
    let primes: Vec<_> = incrementer::Incrementer::new(2)
        .take_while(|i| i < &max)
        .collect();
    println!("{:?}", primes);

    println!("Hello 003")
}
