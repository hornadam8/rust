fn main() {
    let mut cube = RubiksCube::new();
    println!("{:?}", cube);
    cube.bottom_clockwise();
    println!("{:?}", cube);
}

#[derive(Debug)]
struct RubiksCube {
    front: Side,
    back: Side,
    left: Side,
    right: Side,
    top: Side,
    bottom: Side,
}
impl RubiksCube {
    fn new() -> Self {
        Self {
            front: Side {
                t1: Color::Red,
                t2: Color::Red,
                t3: Color::Red,
                m1: Color::Red,
                m2: Color::Red,
                m3: Color::Red,
                b1: Color::Red,
                b2: Color::Red,
                b3: Color::Red,
            },
            back: Side {
                t1: Color::Blue,
                t2: Color::Blue,
                t3: Color::Blue,
                m1: Color::Blue,
                m2: Color::Blue,
                m3: Color::Blue,
                b1: Color::Blue,
                b2: Color::Blue,
                b3: Color::Blue,
            },
            left: Side {
                t1: Color::Yellow,
                t2: Color::Yellow,
                t3: Color::Yellow,
                m1: Color::Yellow,
                m2: Color::Yellow,
                m3: Color::Yellow,
                b1: Color::Yellow,
                b2: Color::Yellow,
                b3: Color::Yellow,
            },
            right: Side {
                t1: Color::Green,
                t2: Color::Green,
                t3: Color::Green,
                m1: Color::Green,
                m2: Color::Green,
                m3: Color::Green,
                b1: Color::Green,
                b2: Color::Green,
                b3: Color::Green,
            },
            top: Side {
                t1: Color::Orange,
                t2: Color::Orange,
                t3: Color::Orange,
                m1: Color::Orange,
                m2: Color::Orange,
                m3: Color::Orange,
                b1: Color::Orange,
                b2: Color::Orange,
                b3: Color::Orange,
            },
            bottom: Side {
                t1: Color::White,
                t2: Color::White,
                t3: Color::White,
                m1: Color::White,
                m2: Color::White,
                m3: Color::White,
                b1: Color::White,
                b2: Color::White,
                b3: Color::White,
            },
        }
    }

    fn bottom_clockwise(&mut self) {
        let hold_1 = self.front.b1.clone();
        let hold_2 = self.front.b2.clone();
        let hold_3 = self.front.b3.clone();
        self.front.update(&self.left, "bottom");
        self.left.update(&self.back, "bottom");
        self.back.update(&self.right, "bottom");
        self.right.b1 = hold_1;
        self.right.b2 = hold_2;
        self.right.b3 = hold_3;
    }

    fn top_clockwise(&mut self) {
        let hold_1 = self.front.t1.clone();
        let hold_2 = self.front.t2.clone();
        let hold_3 = self.front.t3.clone();
        self.front.update(&self.right, "top");
        self.right.update(&self.back, "top");
        self.back.update(&self.left, "top");
        self.left.t1 = hold_1;
        self.left.t2 = hold_2;
        self.left.t3 = hold_3;
    }

    fn right_clockwise(&mut self) {
        let hold_1 = self.front.t3.clone();
        let hold_2 = self.front.m3.clone();
        let hold_3 = self.front.b3.clone();
    }
}

#[derive(Debug)]
struct Side {
    t1: Color,
    t2: Color,
    t3: Color,
    m1: Color,
    m2: Color,
    m3: Color,
    b1: Color,
    b2: Color,
    b3: Color,
}
impl Side {
    fn update(&mut self, other: &Side, side: &str) {
        match side {
            "top" => {
                self.t1 = other.t1.clone();
                self.t2 = other.t2.clone();
                self.t3 = other.t3.clone();
            }
            "bottom" => {
                self.b1 = other.b1.clone();
                self.b2 = other.b2.clone();
                self.b3 = other.b3.clone();
            }
            "left" => {
                self.t1 = other.t1.clone();
                self.m1 = other.m1.clone();
                self.b1 = other.b1.clone();
            }
            "right" => {
                self.t3 = other.t3.clone();
                self.m3 = other.m3.clone();
                self.b3 = other.b3.clone();
            }
            _ => {
                println!("Invalid argument side: {side}");
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Color {
    Red,
    Blue,
    Yellow,
    White,
    Green,
    Orange,
}
