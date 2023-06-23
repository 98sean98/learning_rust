// encapsulation in OOP

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64, // cached value
    // private fields so that they cannot be modified directly
}

impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection{list: vec![], average: 0.0}
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let value = self.list.pop();
        match value {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // private method
    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    c.add(10);
    c.add(20);
    println!("{}", c.average());
    c.remove();
    println!("{}", c.average());
}


// rust doesn't support inheritance and polymorphism in the traditional sense
// instead it offers default trait method implementations to allow a type implementing that trait
// to use default methods (a sort of inheritance of code for re-use)
// another reason for inheritance is polymorphism which is to allow child types and parent types
// be used in the same places
// substitute multiple objects at runtime if they share similar characteristics

// rust uses generics to abstract over different possible types
// and trait bounds to impose constraints on what those types must provide
