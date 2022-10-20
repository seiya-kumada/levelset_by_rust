use opencv as cv;
use opencv::prelude::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_input_image() {
        let img = cv::imgcodecs::imread(
            "/Users/kumada/Data/levelset/dreamworks.png",
            cv::imgcodecs::IMREAD_COLOR,
        )
        .unwrap();
        let s = img.size().unwrap();
        let w = s.width;
        let h = s.height;
        assert_eq!(254, w);
        assert_eq!(240, h);

        println!("{},{}", w, h);
        //let path = std::path::PathBuf::from("/Users/kumada/Data/levelset/dreamworks.png");
        //let image = image::open(path).unwrap().into_bytes();
        //assert_eq!(image.width(), 254);
        //assert_eq!(image.height(), 240);
        //let data = image.into_bytes();
        //assert_eq!(254 * 240, image.len()); // ???
    }
}
