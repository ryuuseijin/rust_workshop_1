use bmp::{self, Image};
use std::io::{stdout, Write};

fn draw_pixel(mut image: bmp::Image) -> bmp::Image {
    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    return image;
}

fn draw_diagonal(mut image: bmp::Image) -> bmp::Image {
    if image.get_width() != image.get_height() {
        panic!("canvas not square");
    }

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

fn draw_square(mut image: bmp::Image, margin_left: u32, margin_right: u32, margin_top: u32, margin_bottom: u32) -> bmp::Image {

    for (x, y) in image.coordinates() {
        if (x == margin_left || x == margin_right) && y > margin_top && y < margin_bottom {
            image.set_pixel(y, x, bmp::Pixel::new(255, 255, 255));
        }
        if (y == margin_top || y == margin_bottom) && x > margin_left && x < margin_right {
            image.set_pixel(y, x, bmp::Pixel::new(255, 255, 255));
        }
    }

    return image;
}

fn draw_house(image: bmp::Image) -> bmp::Image {
    let width = image.get_width();
    let height = image.get_height();
    return draw_square(image, 10, width - 10, 10, height - 10);
}

fn avg(i1: bmp::Image, i2: bmp::Image) -> bmp::Image {
    if i1.get_width() != i2.get_width() || i1.get_height() != i2.get_height() {
        panic!("images not same size");
    }

    let mut image = bmp::Image::new(i1.get_width(), i1.get_height());

    for (x, y) in image.coordinates() {
        let p1 = i1.get_pixel(x, y);
        let p2 = i2.get_pixel(x, y);

        let r = (p1.r + p2.r) / 2;
        let g = (p1.g + p2.g) / 2;
        let b = (p1.b + p2.b) / 2;

        image.set_pixel(x, y, bmp::Pixel::new(r, g, b));
    }
    return image;
}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let path2 = std::env::args().nth(2);
    let mut image = match bmp::open(path.as_str()) {
        Ok(i) => i,
        Err(_) => bmp::Image::new(100, 100),
    };
    let image2: Option<Image> = match path2 {
        Some(s) => match bmp::open(s.as_str()) {
            Ok(i) => Some(i),
            Err(_) => Some(bmp::Image::new(100, 100)),
        },
        None => None,
    };

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    image = match op.as_str() {
        "pixel\n" => draw_pixel(image),
        "diagonal\n" => draw_diagonal(image),
        "x\n" => draw_x(image),
        "avg\n" => match image2 {
            Some(i) => avg(image, i),
            None => panic!("no second image"),
        },
        "house\n" => draw_house(image),
        _ =>  {
            panic!("The operation {op} was not recognized!");
        },
    };

    image.save(path).expect("This should save correctly.");
}
