use sequences::integer::{Naturals, NonNegative, NonPositive, Negative, Fibonnaci, Odd};

fn main() {
   
    //let mut primes = Primes::new();
    //println!("{:?}", primes.take(10_000_000).last());
    //println!("{:?}", Sequence::nth(&mut primes, BigInt::from(10000000)));
    //println!("{:?}", trial(7920).len());
    
     
    let fib = Fibonnaci::default();
    println!("{:?}", fib.take(100).collect::<Vec<_>>());


    let nat = Naturals::default();
    println!("{:?}", nat.take(100).collect::<Vec<_>>());
   
    let nonneg = NonNegative::default();
    println!("{:?}", nonneg.take(100).collect::<Vec<_>>());

    let neg = Negative::default();
    println!("{:?}", neg.take(100).collect::<Vec<_>>());

    let nonpos = NonPositive::default();
    println!("{:?}", nonpos.take(100).collect::<Vec<_>>());

    let test = Odd::default();
    println!("{:?}", test.take(100).collect::<Vec<_>>());

}
