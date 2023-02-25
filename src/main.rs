use clap::Parser;
use image::DynamicImage;

#[derive(Parser, Default, Debug)]
#[clap(author = "Dan", version, about = "Image manipulator")]

struct Args {
    in_file: String,
    out_file: String,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    filters: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse() as Args;
    println!("{:?}", args);
    if !Option::is_some(&args.filters) {
        print_usage_and_exit();
    }
    if let Some(filters) = args.filters {
        let infile = String::from(&args.in_file);
        let outfile = String::from(&args.out_file);
        let mut final_img = image::open(infile).expect("Failed to open INFILE.");

        for filter in filters.into_iter() {
            match filter.as_str() {
                // EXAMPLE FOR CONVERSION OPERATIONS
                "blur" => final_img = blur(&mut final_img),

                "brighten" => final_img = brighten(&mut final_img),

                "crop" => final_img = crop(&mut final_img),

                "rotate" => final_img = rotate(&mut final_img),

                "invert" => invert(&mut final_img),

                "grayscale" => final_img = grayscale(&mut final_img),

                _ => {
                    print_usage_and_exit();
                }
            }
        }
        final_img.save(outfile);
    };
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
}

fn blur(img: &mut DynamicImage) -> DynamicImage {
    img.blur(2.0)
}

fn brighten(img: &mut DynamicImage) -> DynamicImage {
    img.brighten(50)
}

fn crop(img: &mut DynamicImage) -> DynamicImage {
    img.crop(10, 20, 100, 100)
}

fn rotate(img: &mut DynamicImage) -> DynamicImage {
    img.rotate90()
}

fn invert(img: &mut DynamicImage) {
    img.invert()
}

fn grayscale(img: &mut DynamicImage) -> DynamicImage {
    img.grayscale()
}
