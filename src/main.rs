fn get_primes(n: usize) -> Vec<usize> {

    let mut prs = vec![true; n];
    assert!(n >= 2);
    prs[0] = false;
    prs[1] = false;

    for i in 2..prs.len() {

        let mut multiple = i * i;
        while multiple < prs.len()  {

            prs[multiple] = false;
            multiple += i;

        }
    }

    prs.iter().enumerate()
              .filter_map(|(i, &pr)| {
                   if pr { Some(i) }
                   else { None }
               })
              .collect()
}

// return either a type that implements the Iterator trait,
// or return a Box<Iterator> since we don't know the size at compile time
fn get_iterator_primes(n: usize) -> impl Iterator<Item = usize> {

    let mut prs = vec![true; n+1];
    assert!(n >= 2);
    prs[0] = false;
    prs[1] = false;

    let sqrtlimit = (n as f32).sqrt() as usize + 1;
    for i in 2..sqrtlimit {

        let mut multiple = i * i;
        while multiple < prs.len()  {

            prs[multiple] = false;
            multiple += i;

        }
    }

    // or a Box::new(...)
    prs.into_iter().enumerate()
                   .filter_map(|(i, pr)| {
                      if pr { Some(i) }
                      else { None }
                   })
}

fn main() {

    println!("primes: {:?}", get_primes(100));

    let n = 1000;
    let _vrslt = get_iterator_primes(n).collect::<Vec<_>>();
    println!("{:?}", _vrslt);

    // run five times
    let _rslt = (0..5).map(|_| get_iterator_primes(n)).last().unwrap();

}

