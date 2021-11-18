// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    // unimplemented!("implement `fn divmod`");
    (dividend/divisor, dividend%divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    // std::iter::empty()
    // let ev: Vec<T> = Vec::new();
    // let od: Vec<T> = Vec::new();
    // // &out.iter()
    // for i in iter {
    //     // if i % 2 == 0 {
    //     //     ev.push(i)
    //     // }else if i % 2 == 1 {
    //     //     od.push(i)
    //     // }
    //     match i % 2 {
    //         0 => ev.push(i)
    //         _ => od.push(i)
    //     }
    // }
    // ev
    
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // unimplemented!("implement `fn manhattan`")
        self.0.abs()+self.1.abs()
    }
}
