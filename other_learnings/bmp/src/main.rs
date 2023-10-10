use std::time::{SystemTime, UNIX_EPOCH};
pub struct GccLcg {
    state: u64,
}

const MODULUS: u64 = 1 << 31;

impl GccLcg {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // for some reason get 128 bit int, so take the low bits as the seed
        let seed = (seed & ((1 << 31) - 1)) as u64;
        GccLcg { state: seed }
    }

    pub fn from(seed: u64) -> Self {
        GccLcg { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let next_state = (1103515245 * self.state + 12345) % MODULUS;
        self.state = next_state;
        ((next_state << 1) >> 1) as u32
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next()
    }

    pub fn next_u8(&mut self) -> u8 {
        let num = self.next();
        (num & 0xff) as u8
    }
}

// 0 .. 9 -> 10
// 19 -> 20

// 0 .. 9 -> A, B, C, D, E, F
// hex => dec
// 10  => 16
// 19  => 25
// 1A  => 26
// 1F  => 31
// 20  => 32
// etc.

// FF = 255, 99 -> 100
// FF -> 100 = 256

fn gen_image(height: u16, width: u16) -> Vec<u8> {
    let mut data = vec![];
    let mut rng = GccLcg::new();
    let center_x = width / 2;
    let center_y = height / 2;
    for i in 0..height {
        for j in 0..width {
            let dist = f64::sqrt(
                (center_x as i32 - j as i32).pow(2) as f64
                    + (center_y as i32 - i as i32).pow(2) as f64,
            );
            let scale = (height as f64 - dist) / height as f64;
            let r1 = rng.next_u8();
            let r2 = rng.next_u8();
            let base = rng.next_u8();
            if r1 % 2 == 0 && r2 % 2 == 0 {
                data.push(255 - ((255.0 - (base / rng.next_u8()) as f64) * scale) as u8);
                data.push((rng.next_u8() as f64 * scale) as u8);
                data.push((rng.next_u8() as f64 * scale) as u8);
            } else if r1 % 2 == 0 {
                data.push((rng.next_u8() as f64 * scale) as u8);
                data.push((rng.next_u8() as f64 * scale) as u8);
                data.push(255 - ((255.0 - rng.next_u8() as f64) * scale) as u8);
            } else if r2 % 2 == 0 {
                data.push((rng.next_u8() as f64 * scale) as u8);
                data.push(255 - ((255.0 - base as f64) * scale) as u8);
                data.push((rng.next_u8() as f64 * scale) as u8);
            } else {
                data.push(((base / rng.next_u8()) as f64 * scale) as u8);
                data.push(((base / rng.next_u8()) as f64 * scale) as u8);
                data.push(((base / rng.next_u8()) as f64 * scale) as u8);
            }
        }
    }
    data
}

fn main() {
    println!("Trying to write a bitmap");

    let height: u16 = 200;
    let width: u16 = 100;

    let file_size: u32 = (height as u32) * (width as u32) * 3 + 26;

    let mut image = gen_image(height, width);

    let mut bmp = vec![];

    bmp.push(b'B');
    bmp.push(b'M');
    // 2. size
    bmp.append(&mut file_size.to_le_bytes().to_vec());
    //bmp.push(0); bmp.push(0); bmp.push(0); bmp.push(0);
    // 6. reserved
    bmp.push(0);
    bmp.push(0);
    bmp.push(0);
    bmp.push(0);
    // 10. offset
    bmp.push(26);
    bmp.push(0);
    bmp.push(0);
    bmp.push(0);
    // 14. size of header
    bmp.push(12);
    bmp.push(0);
    bmp.push(0);
    bmp.push(0);
    // 18. width
    bmp.append(&mut width.to_le_bytes().to_vec());
    // 20. height
    bmp.append(&mut height.to_le_bytes().to_vec());
    // 22. color planes, must be 1
    bmp.push(1);
    bmp.push(0);
    // 24. bits per pixel
    bmp.push(24);
    bmp.push(0);
    // 26. the data?
    bmp.append(&mut image);

    use std::fs::File;
    use std::io::Write;

    let mut f = File::create("rand2.bmp").unwrap();
    f.write(bmp.as_slice());
    println!("did that work??");
}
