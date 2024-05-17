fn main() {

    #[derive(Debug)]
    struct StemSize {
        reach: u8,
        rise: u8,
        oversized: bool
    }

    #[derive(Debug)]
    enum StemType {
        TopLoad(StemSize),
        FrontLoad(StemSize),
        Unknown(String),
        Integrated,
        Elementary {reach: u8, size: u8},
    }

    impl StemType {
        fn slip(&self) {
            println!("slipped {:?}", &self);
        }
    }

    struct ElementaryProps {
        reach: u8,
        rise: u8,
    }

    let stem = StemType::TopLoad(StemSize { reach: 53, rise: 34, oversized: false });
    stem.slip();
    println!("{:?}", stem);
    let stem = StemType::FrontLoad(StemSize { reach: 48, rise: 0, oversized: true });
    println!("{:?}", stem);
    let stem = StemType::Unknown(String::from("awesome stem"));
    println!("{:?}", stem);
    let stem = StemType::Integrated;
    println!("{:?}", stem);
}