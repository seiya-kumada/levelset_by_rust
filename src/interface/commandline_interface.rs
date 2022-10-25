//use crate::core::dimension_types as dim;
//use crate::core::initial_front::InitialFront_;
use crate::core::parameters::Parameters;
use crate::core::space_size;
//use crate::core::space_size::SpaceSize;
use clap::Parser;
use image::GenericImageView;
use image::ImageFormat;
use opencv as cv;
use opencv::prelude::*;
#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandlineArguments {
    #[arg(long)]
    dim: i32,

    #[arg(long)]
    verbose: bool,

    #[arg(long)]
    input_path: std::path::PathBuf,

    #[arg(long)]
    wband: i32,

    #[arg(long)]
    wreset: i32,

    #[arg(long)]
    time_step: f64,

    #[arg(long)]
    gain: f64,

    #[arg(long)]
    constant_speed: f64,

    #[arg(long)]
    speed_threshold: f64,

    #[arg(long)]
    left: i32,

    #[arg(long)]
    top: i32,

    #[arg(long)]
    right: i32,

    #[arg(long)]
    bottom: i32,

    #[arg(long)]
    front: Option<i32>,

    #[arg(long)]
    back: Option<i32>,
}

//pub fn load_input_image(
//    input_path: &std::path::PathBuf,
//) -> Option<(SpaceSize<TwoDimension>, Vec<u8>)> {
//    let gray = cv::imgcodecs::imread(
//        input_path.to_str().unwrap(),
//        cv::imgcodecs::IMREAD_GRAYSCALE,
//    )
//    .unwrap();
//
//    if gray.empty() {
//        return None;
//    }
//    let space_size = SpaceSize::<TwoDimension>::new(gray.cols(), gray.rows());
//    let image: Vec<u8> = gray.data_typed::<u8>().unwrap().iter().cloned().collect();
//    Some((space_size, image))
//}

fn make_parameters(args: &CommandlineArguments) -> Parameters {
    Parameters {
        wband: args.wband,
        wreset: args.wreset,
        time_step: args.time_step,
        gain: args.gain,
        constant_speed: args.constant_speed,
        speed_threshold: args.speed_threshold,
    }
}

fn execute_level_set_method_in_2d(args: &CommandlineArguments, params: &Parameters) {
    // set an initial front
    let left = args.left;
    let top = args.top;
    let right = args.right;
    let bottom = args.bottom;

    let lt = [left, top];
    let rb = [right, bottom];
    //let inital_front = InitialFront_::<{ dim::TWO }> { vertices: [lt, rb] };

    // load an input image
    //let (space_size, image) = load_input_image(&args.input_path).unwrap();

    // Viewer2d
}

fn execute_level_set_method_in_3d(args: &CommandlineArguments, params: &Parameters) {}

fn print_args(args: &CommandlineArguments) {
    println!("dim: {}", args.dim);
    println!("verbose: {:?}", args.verbose);
    println!("input_path: {:?}", args.input_path);
    println!("wband: {}", args.wband);
    println!("wreset: {}", args.wreset);
    println!("time_step: {}", args.time_step);
    println!("gain: {}", args.gain);
    println!("constant_speed: {}", args.constant_speed);
    println!("speed_threshold: {}", args.speed_threshold);
    println!("left: {}", args.left);
    println!("top: {}", args.top);
    println!("right: {}", args.right);
    println!("bottom: {}", args.bottom);
    match args.front {
        Some(v) => println!("front: {}", v),
        None => println!("front: no used"),
    }
    match args.back {
        Some(v) => println!("back: {}", v),
        None => println!("back: no used"),
    }
}

pub fn execute_level_set_method(args: &CommandlineArguments) {
    print_args(&args);
    let params = make_parameters(&args);
    match args.dim {
        2 => execute_level_set_method_in_2d(&args, &params),
        3 => execute_level_set_method_in_3d(&args, &params),
        _ => println!("unsupported dimension!"),
    }
}
