// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

const INFILE_ERROR: &str = "failed to write INFILE";
const OUTFILE_ERROR: &str = "failed to write OUTFILE";

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0).parse::<f32>().expect("amount is not a number");


            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile, amount);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0).parse::<i32>().expect("amount is not a number");

            brighten(infile, outfile, amount);
        }

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x = args.remove(0).parse::<u32>().expect("x is not a number");
            let y = args.remove(0).parse::<u32>().expect("y is not a number");
            let width = args.remove(0).parse::<u32>().expect("width is not a number");
            let height = args.remove(0).parse::<u32>().expect("height is not a number");

            crop(infile, outfile, x, y, width, height);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount = args.remove(0).parse::<u16>().expect("amount is not a number");

            rotate(infile, outfile, amount)
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        "generate" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let w = args.remove(0).parse::<u32>().expect("WIDTH is not a valid number 0-255");
            let h = args.remove(0).parse::<u32>().expect("HEIGHT is not a valid number 0-255");
            let red = args.remove(0).parse::<u8>().expect("RED is not a valid number 0-255");
            let green = args.remove(0).parse::<u8>().expect("GREEN is not a valid number 0-255");
            let blue = args.remove(0).parse::<u8>().expect("BLUE is not a valid number 0-255");

            generate(outfile, w, h, Color(red, green, blue));
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE AMOUNT");
    println!("fractal OUTFILE");
    println!("crop INFILE OUTFILE X Y WIDTH HEIGHT");
    println!("rotate INFILE OUTFILE (90|180|270)");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("generate OUTFILE WIDTH HEIGHT RED GREEN BLUE");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, amount: f32) {
    let img = image::open(infile).expect(INFILE_ERROR);
    let img2 = img.blur(amount);
    img2.save(outfile).expect(OUTFILE_ERROR);
}

fn brighten(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect(INFILE_ERROR);

    let img2 = img.brighten(amount);
    img2.save(outfile).expect(OUTFILE_ERROR);
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    image::open(infile).expect(INFILE_ERROR)
        .crop(x, y, width, height)
        .save(outfile).expect(OUTFILE_ERROR);
}


fn rotate(infile: String, outfile: String, amount: u16) {
    let img = image::open(infile).expect(INFILE_ERROR);

    let img = match amount {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("invalid rotation amount {}", amount)
    };

    img.save(outfile).expect(OUTFILE_ERROR);
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect(INFILE_ERROR);
    img.invert();
    img.save(outfile).expect(OUTFILE_ERROR);
}

fn grayscale(infile: String, outfile: String) {
    image::open(infile).expect(INFILE_ERROR)
        .grayscale()
        .save(outfile).expect(OUTFILE_ERROR);
}

// represents an RGB color
struct Color (u8, u8, u8);

fn generate(outfile: String, w: u32, h: u32, clr: Color) {
    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([clr.0, clr.1, clr.2]);
    }

    imgbuf.save(outfile).expect(OUTFILE_ERROR);
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
