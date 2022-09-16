use bmp;
use std::io::{stdout, Write};

fn draw_pixel(mut image: bmp::Image) -> bmp::Image {
    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    return image;
}

fn draw_diagonal(mut image: bmp::Image) -> bmp::Image {

    for (x, y) in image.coordinates() {
        if x == y {
            image.set_pixel(x, y, bmp::Pixel::new(255, 255, 255));
        }
    }
    

    return image;
}


fn draw_x(mut image: bmp::Image) -> bmp::Image {

    for (x, y) in image.coordinates() {
        if x == y {
            image.set_pixel(x, y, bmp::Pixel::new(255, 255, 255));
        }
        if image.get_width() - x == y {
            image.set_pixel(y, x, bmp::Pixel::new(255, 255, 255));
        }
    }
    

    return image;
}


fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    let mut image = match bmp::open(path.as_str()) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100)
    };

    image = match op.as_str() {
        "pixel\n" => draw_pixel(image),
        "diagonal\n" => draw_diagonal(image),
        "x\n" => draw_x(image),
        _ =>  {
            panic!("The operation {op} was not recognised!");
        },
    };

    image.save(path).expect("This should save correctly.");
}
