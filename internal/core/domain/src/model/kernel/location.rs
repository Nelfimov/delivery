use rand::Rng;
use rand::rng;

#[derive(PartialEq, Debug)]
pub struct Location {
    x: u8,
    y: u8,
}

impl Location {
    pub fn new(x: u8, y: u8) -> Result<Self, String> {
        if x * y == 0 {
            return Err(format!(
                "coordinates cannot be less than 1. x: {}, y: {}",
                x, y
            ));
        }

        if x > 10 || y > 10 {
            return Err(format!(
                "coordinates cannot be more than 10. x: {}, y: {}",
                x, y
            ));
        }

        Ok(Self { x, y })
    }

    pub fn new_random() -> Location {
        let mut rand = rng();

        let x = rand.random_range(1..=10);
        let y = rand.random_range(1..=10);

        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }

    pub fn get_distance(&self, other: &Location) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
