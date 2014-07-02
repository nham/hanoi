enum TowerType {
    A,
    B,
    C,
}

impl TowerType {
    fn other(type1: TowerType, type2: TowerType) -> TowerType {
        match (type1, type2) {
            (A, B) => C,
            (B, A) => C,
            (A, C) => B,
            (C, A) => B,
            (B, C) => A,
            (C, B) => A,
            _      => fail!("TowerTypes must be different in other()"),
        }
    }
}

#[deriving(Show)]
struct Towers {
    a: Vec<uint>,
    b: Vec<uint>,
    c: Vec<uint>,
}

impl Towers {
    fn new() -> Towers {
        Towers { a: vec!(), b: vec!(), c: vec!() }
    }

    fn pop(&mut self, t: TowerType) -> Option<uint> {
        match t {
            A => self.a.pop(),
            B => self.b.pop(),
            C => self.c.pop(),
        }
    }

    fn push(&mut self, x: uint, t: TowerType) {
        match t {
            A => self.a.push(x),
            B => self.b.push(x),
            C => self.c.push(x),
        }
    }

    fn move(&mut self, src: TowerType, dest: TowerType) -> bool {
        match self.pop(src) {
            None => false,
            Some(x) => { self.push(x, dest); true },
        }
    }
}

fn main() {
    let mut towers = Towers::new();
    let n = 10u;
    for i in range(0, n) {
        towers.push(n - i - 1, A);
    }

    println!("{}", towers);
    move_rec(&mut towers, A, B, n);
    println!("{}", towers);

}

fn move_rec(towers: &mut Towers, src: TowerType, dest: TowerType, size: uint) {
    if size == 0 {
        return;
    } else {
        let other = TowerType::other(src, dest);
        move_rec(towers, src, other, size - 1);
        towers.move(src, dest);
        move_rec(towers, other, dest, size - 1);
    }
}
