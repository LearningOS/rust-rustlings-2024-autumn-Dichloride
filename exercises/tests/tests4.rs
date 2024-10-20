// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    fn negative_width() {
        let result = std::panic::catch_unwind(|| {
            Rectangle::new(-10, 10);
        });
        assert!(result.is_err());
    }

    #[test]
    fn negative_height() {
        let result = std::panic::catch_unwind(|| {
            Rectangle::new(10, -10);
        });
        assert!(result.is_err());
    }
}
