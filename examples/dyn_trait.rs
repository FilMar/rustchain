trait Trait {
    fn demo(&self) {println!("static");}
}

struct S;

impl dyn Trait {
    fn demo2(&self) {println!("dyn");}
}

impl Trait for S {}

fn main() {
    let a = S;
    a.demo();
    // a.demo2();     
    let b : &dyn Trait = &a;
    b.demo();
    b.demo2();
}

