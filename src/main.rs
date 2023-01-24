fn main() {
    let rocket = Rocket::default();
    println!("{rocket}");
    let mut rocket = rocket.launch();
    rocket.accelerate();
    rocket.decelerate();
    println!("{rocket}");
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

type Kilograms = usize;

#[derive(Debug)]
struct Grounded;

#[derive(Debug)]
struct Launched;

#[derive(Debug)]
struct Rocket<Stage = Grounded> {
    weight: Kilograms,
    stage: std::marker::PhantomData<Stage>,
}

impl Default for Rocket<Grounded> {
    fn default() -> Self {
        Self {
            weight: 100_000,
            stage: std::marker::PhantomData::default(),
        }
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket {
            weight: ((self.weight as f32) * 0.9) as Kilograms,
            stage: std::marker::PhantomData::default(),
        }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {}
    pub fn decelerate(&mut self) {}
}

impl<Stage> Rocket<Stage> {
    pub fn color(&self) -> Color {
        Color::White
    }
    pub fn weight(&self) -> Kilograms {
        self.weight
    }
}

impl<Stage> std::fmt::Display for Rocket<Stage> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Rocket ({:?}kg, {:?})", self.weight(), self.color())
    }
}
