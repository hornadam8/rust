use base64::{engine::general_purpose, Engine as _};
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_traits::{FromBytes, ToBytes, Zero};
use rand;
use rand::Rng;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;
e
fn main() {
    // Encrypted message
    // uses a private key comprised of the larger prime of pk1 and the smaller prime of pk2
    let en_msg = "MO2bkW3pfzSifUfpvnXKZM49a00wcuVktbW3WVQvTMSp3xU25QQPmaKwDktkQbGxij0Adi49qiPsjwT2CItXjpw+25nCFpXwiC5cWVxhRCayW/YEUfkpTS0xbPIwNDX/BnFd2FHmwxE/qmQih9bbOdUqX4EOMW7AAuMhNh0JjyIB0XaSqqSHc7Xvn2FEkLhgv8LYOqy3zj9k7nIGGoQrdY2ZH78afpHaYMEeEYVFoKfry7CMNufR9L45j8GFJebBb5Tn6I3Kd0q7d1l+zxpP056j37klVsb+JbOAfNe7JIpvtDCYOPmVD+pmFbBedpdXT+7Au0tMC9DWt6XtPaNP/A==";

    // Public key 1: prime factors are too close together
    let pk1 = BigInt::from_str("8079251517827751825178719172167487990111025667428871008032586356881163784716972723299300352880728365922179490230474504873529889787622730273772038096612070780157719341825249022937549437597413026699014409596016892069198054660654939040459523584619042617645411463009076260721893972885266452151888099780982605530556870483118824356561734293698134015855069127277805074837152262268222372370637570266351881437220218174162272503665975814140030673067079739542296521901778511814788853406040130937964759256677416909888813819872802576793079866106453800257294709162903711263543868616301516236200642330318236863190784946149375457513").unwrap();
    // Public key 2: shares a prime factor with companion private key
    let pk2 = BigInt::from_str("87745270228830357627970028347684962028903640024173591540668978994243244414901079288232985712927971819208052749160894761854436318749310622431471330376711680900466728059395822523814004800669865944206058474713093757387209573007769718507556993435069887333504430537884030407951908031187457482806861659225203924481903108487576287175834580929177672013678506744359364629213785244832939313976590818350259628884118381184566456182938281824742752449460289267522247334746324960216490990897573422237284575166933463718606744531340839850523623627008296986130616744527565782088558881362136314958560711715015724113279084989627715260283").unwrap();
    // Companion public key: shares a prime factor with private key 2
    let cpk = BigInt::from_str(    "50881103001705233807300123654393357377736942657583928495620961990121030786923374151916722187488957409169171593637713880300824573384461444809804546290097661292288242320394127296208262311525235884025900556554692617769995633349580980468723294231221555122137771779461685915377046566966503220213424180519114574895904764387555518641682248518773580302577267985769293749348666102922093445967971453731776624240815886885658279709928815115059217767790752128608542730385192411077908761490774191119059815361267054733683538796434472872587115732592751494823069442899740238768016413036901160881175805699355362982859357590110227039267").unwrap();

    // Timing the computation
    let start = SystemTime::now();

    // Finding the greatest common divisor of pk2 and cpk i.e. shared prime
    let gcd = gcd(&cpk, &pk2);
    let pk2_f1 = gcd.clone();

    // Find the other prime factor in pk2
    let pk2_f2 = pk2 / gcd;

    // Find prime factorization of pk1, take just the larger prime
    let (_, pk1_big_p) = find_factors(pk1);

    // determine which factor of pk2 is smaller, generate new pk with primes and get phi of n
    let (phi, n) = if pk2_f1 > pk2_f2 {
        let p_key = &pk2_f2 * &pk1_big_p;
        if &p_key % &pk2_f2 != 0.to_bigint().unwrap()
            || &p_key % &pk1_big_p != 0.to_bigint().unwrap()
        {
            println!("YO THERE'S A PROBLEM")
        }
        (get_totient(&pk2_f2, &pk1_big_p), pk2_f2 * pk1_big_p)
    } else {
        let p_key = &pk2_f1 * &pk1_big_p;
        if &p_key % &pk2_f1 != 0.to_bigint().unwrap()
            || &p_key % &pk1_big_p != 0.to_bigint().unwrap()
        {
            println!("YO THERE'S A PROBLEM")
        }
        (get_totient(&pk2_f1, &pk1_big_p), pk2_f1 * pk1_big_p)
    };

    // set e, (this came from the decompiled TopSecret.class)
    let e = 65537.to_bigint().unwrap();

    // use e and phi of n to get d
    let d = extended_gcd(e, phi);

    // Base64 decode encrypted message
    let de_msg = general_purpose::STANDARD.decode(en_msg).unwrap();

    // convert bytes to biguint to do fast modular exponentiation
    let c = BigUint::from_bytes_be(&de_msg);

    // get m (i.e. the message as a number)
    let m = fme(c.into(), d, n);

    // convert it to bytes
    let mut m_bytes_be = m.to_signed_bytes_be();

    // strip padding from bytes
    strip_padding(&mut m_bytes_be);

    // convert message bytes to string and print it
    let the_msg = String::from_utf8(m_bytes_be);
    match the_msg {
        Ok(msg) => {
            println!("MESSAGE:\n\n{msg}\n");
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }

    // stop timer
    let elapsed = start.elapsed().unwrap();
    println!("Found encrypted message in {elapsed:?}");
}

fn fme(a: BigInt, b: BigInt, n: BigInt) -> BigInt {
    let mut result: BigInt = 1.to_bigint().unwrap();
    let mut square = a;
    let mut b = b;
    let one = 1.to_bigint().unwrap();
    let two = 2.to_bigint().unwrap();
    while b > Zero::zero() {
        let k = &b % &two;
        if k == one {
            result = (&result * &square) % &n;
        }
        b /= &two;
        square = (&square * &square) % &n;
    }
    return result % n;
}

fn pollards_rho_u128(num: u128) -> (u128, u128) {
    let mut x: u128 = 2;
    let mut y: u128 = x;
    let mut d = 1;
    while d == 1 {
        x = (&x * &x + 1) % &num;
        y = (&y * &y + 1) % &num;
        y = (&y * &y + 1) % &num;
        let diff = if x > y { x - y } else { y - x };
        d = gcd_u128(diff, num)
    }
    let p2 = num / d;
    (d, p2)
}

fn pollards_rho_bigint(num: BigInt) -> (BigInt, BigInt) {
    let mut x: BigInt = 2.to_bigint().unwrap();
    let mut y: BigInt = x.clone();
    let mut d = 1.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    while &d == &one {
        x = (&x * &x + 1) % &num;
        y = (&y * &y + 1) % &num;
        y = (&y * &y + 1) % &num;
        let diff = if &x > &y { &x - &y } else { &y - &x };
        d = gcd(&diff, &num)
    }
    let p2 = &num / &d;
    (d, p2)
}

fn find_factors(num: BigInt) -> (BigInt, BigInt) {
    let sqrt = num.sqrt();
    let mut done: bool = false;
    let mut result: (BigInt, BigInt) = (0.to_bigint().unwrap(), 0.to_bigint().unwrap());
    let mut diff_factor = 0.to_bigint().unwrap();
    while !done {
        let f1 = &sqrt - &diff_factor;
        if &num % &f1 == 0.to_bigint().unwrap() {
            result.1 = &num / &f1;
            result.0 = f1;
            done = true;
        } else {
            diff_factor += 1.to_bigint().unwrap();
        }
    }
    return result;
}

fn gcd_u128(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    } else {
        gcd_u128(b, a % b)
    }
}
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b == &0.to_bigint().unwrap() {
        return a.clone();
    } else {
        gcd(&b, &(a % b))
    }
}

fn extended_gcd(a: BigInt, b: BigInt) -> BigInt {
    let (mut old_r, mut r) = (a, b.clone());
    let (mut old_s, mut s) = (1.to_bigint().unwrap(), 0.to_bigint().unwrap());
    let (mut old_t, mut t) = (0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    while r != 0.to_bigint().unwrap() {
        let quotient = &old_r / &r;
        let temp_r = &old_r - &quotient * &r;
        let temp_s = &old_s - &quotient * &s;
        let temp_t = &old_t - &quotient * &t;
        old_r = r;
        r = temp_r;
        old_s = s;
        s = temp_s;
        old_t = t;
        t = temp_t;
    }
    if &old_s < &0.to_bigint().unwrap() {
        &b + &old_s
    } else {
        old_s
    }
}

fn get_totient(p: &BigInt, q: &BigInt) -> BigInt {
    return (p - 1.to_bigint().unwrap()) * (q - 1.to_bigint().unwrap());
}

fn strip_padding(input: &mut Vec<u8>) {
    let mut curr_bit: u8 = 1;
    while curr_bit != 0 {
        input.remove(0);
        curr_bit = input.get(0).unwrap().clone();
    }
}
