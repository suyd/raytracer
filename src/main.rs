struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn dot_product(self, other: Vector3D) -> f64 {
       self.x * other.x + self.y * other.y + self.z * other.z  
    }

    fn cross_product(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Check if you need to use checked_pow
    fn length(self) -> f64 {
        (self.x.powf(2.0) +
         self.y.powf(2.0) +
         self.z.powf(2.0)).sqrt()
    }

    fn unit_vector(self) -> Vector3D{
        let length = self.length();
        Vector3D {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,

        }
    }
}

// Operations for two vectors
impl std::ops::Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.y,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.y,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Div for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x / other.y,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Operations for aplifiers
impl std::ops::Div<f64> for Vector3D {
    type Output = Vector3D;
    
    fn div(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;
    
    fn mul(self, other: f64) -> Vector3D {
        Vector3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
fn main() {
    let mut vector = Vector3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let aditive = Vector3D {
        x: 2.0,
        y: 1.0,
        z: 0.0,
    };

    println!("{} {} {}", vector.x, vector.y, vector.z);
    vector = vector + aditive;

    println!("{}", vector.x);
}
