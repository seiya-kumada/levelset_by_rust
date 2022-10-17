use crate::core::initial_front::IntInitialFront2D;
use clap::Parser;
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
fn execute_level_set_method_in_2d(args: &CommandlineArguments) {
    // set an initial front
    let left = args.left;
    let top = args.top;
    let right = args.right;
    let bottom = args.bottom;

    let inital_front = IntInitialFront2D::new(left, top, right, bottom);
}
fn execute_level_set_method_in_3d(args: &CommandlineArguments) {}
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

    match args.dim {
        2 => execute_level_set_method_in_2d(&args),
        3 => execute_level_set_method_in_3d(&args),
        _ => println!("unsupported dimension!"),
    }
}
