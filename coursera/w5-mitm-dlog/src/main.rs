use num_bigint::BigUint;
use std::collections::HashMap;

fn dlog_solver(p: &BigUint, g: &BigUint, h: &BigUint) -> Option<u64> {
    let mut seen: HashMap<BigUint, u64> = HashMap::new();
    for i in 0..2_u64.pow(20) {
        let g_i_inverse = g.modpow(&BigUint::from(i), p).modinv(p).unwrap();
        let value = (h * g_i_inverse) % p;
        seen.insert(value, i);
    }

    let k = g.modpow(&BigUint::from(2_u64.pow(20)), p);

    for j in 0..2_u64.pow(20) {
        let k_j = k.modpow(&BigUint::from(j), p);
        if let Some(x1) = seen.get(&k_j) {
            return Some(j * 2_u64.pow(20) + x1);
        }
    }
    None
}

fn main() {
    println!("Hello, Week 5 Assignment!\n");
    let p = BigUint::parse_bytes(
        b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171",
        10,
    ).unwrap();
    let g = BigUint::parse_bytes(
        b"11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568",
        10,
    ).unwrap();
    let h = BigUint::parse_bytes(
        b"3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333",
        10,
    ).unwrap();

    println!("{}", dlog_solver(&p, &g, &h).expect("No solution found!"));
}
