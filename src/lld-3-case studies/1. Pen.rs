// Define the PenType enum
#[derive(Debug)]
enum PenType {
    Ball,
    Fountain,
    Gel,
}

// Define the ClosureType enum
#[derive(Debug)]
enum ClosureType {
    Click,
    Cap,
    Rotate,
}

// Define the Refillable trait
trait Refillable {
    fn refill(&self);
}

// Define the InkFeatures struct
#[derive(Clone, Debug)]
struct InkFeatures {
    sparkling: bool,
    transparent: bool,
    waterproof: bool,
}

// Define the Ink struct
#[derive(Clone, Debug)]
struct Ink {
    color: String,
    density: f64,
    features: InkFeatures,
}

// Define the Nib struct
#[derive(Clone, Debug)]
struct Nib {
    radius: f64,
}

// Define the Refill struct
#[derive(Clone, Debug)]
struct Refill {
    ink: Ink,
    nib: Option<Nib>,
}

// Define the Pen struct
#[derive(Debug)]
struct Pen {
    name: String,
    brand: String,
    price: f64,
    pen_type: PenType,
    closure_type: ClosureType,
}

impl Pen {
    fn write(&self) {
        // Write logic
        println!("Writing with {} pen", self.name);
    }

    fn close(&self) {
        // Close logic
        println!("Closing {} pen", self.name);
    }

    fn open(&self) {
        // Open logic
        println!("Opening {} pen", self.name);
    }
}

// Define the BallPen struct
#[derive(Debug)]
struct BallPen {
    pen: Pen,
}

// Implement Refillable for BallPen
impl Refillable for BallPen {
    fn refill(&self) {
        // Refill logic for BallPen
        println!("Refilling BallPen");
    }
}

// Define the FountainPen struct
#[derive(Debug)]
struct FountainPen {
    pen: Pen,
    refill: Refill,
}

// Define the GelPen struct
#[derive(Debug)]
struct GelPen {
    pen: Pen,
}

// Implement Refillable for GelPen
impl Refillable for GelPen {
    fn refill(&self) {
        // Refill logic for GelPen
        println!("Refilling GelPen");
    }
}

// Example instantiation
fn main() {
    let ink_features = InkFeatures {
        sparkling: true,
        transparent: false,
        waterproof: true,
    };

    let ink = Ink {
        color: String::from("Blue"),
        density: 0.8,
        features: ink_features,
    };

    let nib = Nib {
        radius: 0.5,
    };

    let refill = Refill {
        ink: ink.clone(),
        nib: Some(nib.clone()),
    };

    let fountain_pen = Pen {
        name: String::from("Fancy Fountain Pen"),
        brand: String::from("LuxuryBrand"),
        price: 25.0,
        pen_type: PenType::Fountain,
        closure_type: ClosureType::Cap,
    };

    let ball_pen = Pen {
        name: String::from("Regular Ball Pen"),
        brand: String::from("StandardBrand"),
        price: 5.0,
        pen_type: PenType::Ball,
        closure_type: ClosureType::Click,
    };

    let gel_pen = Pen {
        name: String::from("Smooth Gel Pen"),
        brand: String::from("ModernBrand"),
        price: 10.0,
        pen_type: PenType::Gel,
        closure_type: ClosureType::Rotate,
    };

    let fountain_pen_instance = FountainPen {
        pen: fountain_pen,
        refill: refill.clone(),
    };

    let ball_pen_instance = BallPen {
        pen: ball_pen,
    };

    let gel_pen_instance = GelPen {
        pen: gel_pen,
    };

    // Using InkFeatures fields
    println!(
        "Ink features: sparkling = {}, transparent = {}, waterproof = {}",
        ink.features.sparkling, ink.features.transparent, ink.features.waterproof
    );

    // Using Pen fields
    println!(
        "Pen details: name = {}, brand = {}, price = {}, pen_type = {:?}, closure_type = {:?}",
        fountain_pen_instance.pen.name,
        fountain_pen_instance.pen.brand,
        fountain_pen_instance.pen.price,
        fountain_pen_instance.pen.pen_type,
        fountain_pen_instance.pen.closure_type
    );

    // Using Nib field
    println!("Nib radius: {}", nib.radius);

    // Using Refill fields
    println!(
        "Refill details: ink color = {}, density = {},nib radius = {}",
        fountain_pen_instance.refill.ink.color,
        fountain_pen_instance.refill.ink.density,
        fountain_pen_instance.refill.nib.as_ref().unwrap().radius
    );

    fountain_pen_instance.pen.write();
    fountain_pen_instance.pen.open();
    fountain_pen_instance.pen.close();

    ball_pen_instance.pen.write();
    ball_pen_instance.pen.open();
    ball_pen_instance.pen.close();
    ball_pen_instance.refill();

    gel_pen_instance.pen.write();
    gel_pen_instance.pen.open();
    gel_pen_instance.pen.close();
    gel_pen_instance.refill();
}
