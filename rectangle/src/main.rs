struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_contain(&self, rect: &Rectangle) -> bool {
        (self.width >= rect.width) && (self.height >= rect.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}.", rect.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_test() {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        assert_eq!(1500, rect.area());
    }

    #[test]
    fn contain_test() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 20,
            height: 40,
        };

        assert_eq!(true, rect1.can_contain(&rect1));
        assert_eq!(true, rect1.can_contain(&rect2));
        assert_eq!(false, rect2.can_contain(&rect1));
    }
}
