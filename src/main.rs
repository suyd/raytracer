pub mod vector;
use crate::vector::Vector3D;


fn main() {

    let mut vector = Vector3D;

    vector.new(1.0,2.0,3.0);


//    println!("{} {} {}", vector.x, vector.y, vector.z);
//    vector = vector + aditive;

    println!("{}", vector.get_x());
}
