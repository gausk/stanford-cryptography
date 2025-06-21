use num_bigint::BigUint;
use num_traits::identities::One;

// |p - q| < 2 * N^1/4
// A = (p + q)/2 =  ceil(sqrt(N)), then p = A - x, q = A + x where x^2 = A^2 - N
fn break_rsa_q1(n: &BigUint) -> (BigUint, BigUint) {
    let a = n.sqrt() + BigUint::one();
    let x_sqr = &a * &a - n;
    let x = x_sqr.sqrt();
    (&a - &x, &a + &x)
}

// |p - q| < 2^11 * N^1/4
// A = (p + q)/2 = ceil(sqrt(N)) + k, for 0 ≤ k ≤ 2^19
fn break_rsa_q2(n: &BigUint) -> Option<(BigUint, BigUint)> {
    let one = BigUint::one();
    let mut a = n.sqrt() + &one;
    let limit = 1u32 << 19;

    for _ in 0..=limit {
        let x_sqr = &a * &a - n;
        let x = x_sqr.sqrt();

        // Check if x^2 is a perfect square and A^2 - x^2 = N
        if &a * &a == &x * &x + n {
            return Some((&a - &x, &a + &x));
        }

        // Increment A to try next candidate
        a += &one;
    }

    // Failed to find factors within range
    None
}

// |3p - 2q| < N^1/4
// (3p + 2q)/2 >  (6N)^1/2
// A > 0.4 * (6N)^1/2
fn break_rsa_q3(n: &BigUint) -> Option<(BigUint, BigUint)> {
    let one = BigUint::one();
    let mut a = (26u32 * n).sqrt() / 5u32;
    let limit = 1u32 << 30;

    for _ in 0..=limit {
        let x_sqr = &a * &a - n;
        let x = x_sqr.sqrt();

        // Check if x^2 is a perfect square and A^2 - x^2 = N
        if &a * &a == &x * &x + n {
            return Some((&a - &x, &a + &x));
        }

        // Increment A to try next candidate
        a += &one;
    }

    // Failed to find factors within range
    None
}


fn main() {
    println!("Hello, Week 6 Assignment!\n");
    println!("\nQuestion 1:");
    let n = BigUint::parse_bytes(b"179769313486231590772930519078902473361797697894230657273430081157732675805505620686985379449212982959585501387537164015710139858647833778606925583497541085196591615128057575940752635007475935288710823649949940771895617054361149474865046711015101563940680527540071584560878577663743040086340742855278549092581", 10).unwrap();
    let (p, q) = break_rsa_q1(&n);
    println!("p: {}\nq: {}", p, q);

    println!("\nQuestion 2:");
    let n = BigUint::parse_bytes(b"648455842808071669662824265346772278726343720706976263060439070378797308618081116462714015276061417569195587321840254520655424906719892428844841839353281972988531310511738648965962582821502504990264452100885281673303711142296421027840289307657458645233683357077834689715838646088239640236866252211790085787877", 10).unwrap();
    let (p, q) = break_rsa_q2(&n).unwrap();
    println!("p: {}\nq: {}", p, q);

    println!("\nQuestion 3:");
    let n = BigUint::parse_bytes(b"720062263747350425279564435525583738338084451473999841826653057981916355690188337790423408664187663938485175264994017897083524079135686877441155132015188279331812309091996246361896836573643119174094961348524639707885238799396839230364676670221627018353299443241192173812729276147530748597302192751375739387929", 10).unwrap();
    let (p, q) = break_rsa_q3(&n).unwrap();
    println!("p: {}\nq: {}", p, q);
}
