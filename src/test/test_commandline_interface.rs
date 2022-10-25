use crate::interface::commandline_interface as cm;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_input_image() {
        let path = std::path::PathBuf::from("/Users/kumada/Data/levelset/dreamworks.png");
        let r = cm::load_input_image(&path);
        match r {
            Some(v) => {
                let space_size = v.0;
                let image = v.1;
                assert_eq!(image.len(), 240 * 254);
            }
            None => (),
        }
    }
}
