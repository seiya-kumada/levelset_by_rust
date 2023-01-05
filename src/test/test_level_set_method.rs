use crate::core::grid::{Grid2d, Grid3d};
use crate::core::indexer::{Indexer2d, Indexer3d, IndexerMethod};
use crate::core::initial_front::{InitialFront2d, InitialFront3d};
use crate::core::inside_estimator::{InsideEstimator2d, InsideEstimator3d, InsideEstimatorMethod};
use crate::core::level_set_method::{LevelSetMethod2d, LevelSetMethod3d};
use crate::core::parameters::Parameters;
use crate::core::point::{Point2d, Point3d};
use crate::core::space_size::{SpaceSize2d, SpaceSize3d};
use crate::core::status::Status;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use crate::core::{
        grid_range::GridRangeMethod, initial_front, inside_estimator::InsideEstimator3d, parameters,
    };

    use super::*;
    #[test]
    fn front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();

        initial_front.vertices[0] = Point3d::<i32>::new(0, 0, 0);
        initial_front.vertices[1] = Point3d::<i32>::new(2, 2, 2);

        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);

        let front = lsm.get_front();
        assert_eq!(front.borrow().len(), 26);
    }

    #[test]
    fn initialize_along_front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();

        initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
        initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

        let size = Rc::new(SpaceSize3d::new(101, 143, 131));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);

        let phi = lsm.get_phi();
        let statuses = lsm.get_statuses();

        let left = initial_front.vertices[0].x;
        let top = initial_front.vertices[0].y;
        let front_ = initial_front.vertices[0].z;
        let right = initial_front.vertices[1].x;
        let bottom = initial_front.vertices[1].y;
        let back = initial_front.vertices[1].z;

        let indexer = Indexer3d::new(&size);

        for j in top..bottom {
            for i in left..right {
                let p = Point3d::new(i, j, front_);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(i, j, back);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }

        for k in front_..back {
            for i in left..right {
                let p = Point3d::new(i, top, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(i, bottom, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }

        for j in top..bottom {
            for k in front_..back {
                let p = Point3d::new(left, j, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
                let p = Point3d::<i32>::new(right, j, k);
                let index = indexer.get(&p) as usize;
                assert_eq!(phi.borrow()[index], 0.0);
                assert_eq!(statuses.borrow()[index], Status::Front);
            }
        }
    }

    #[test]
    fn initialize_over_all_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront3d::new();
        initial_front.vertices[0] = Point3d::<i32>::new(10, 15, 32);
        initial_front.vertices[1] = Point3d::<i32>::new(82, 74, 61);

        let size = Rc::new(SpaceSize3d::new(101, 143, 131));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let mut lsm = LevelSetMethod3d::new(params.clone(), Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);
        lsm.initialize_over_all(&initial_front);

        let phi = lsm.get_phi();
        let width = size.width;
        let height = size.height;
        let depth = size.depth;
        let indexer = lsm.get_indexer();
        let statuses = lsm.get_statuses();
        let mut insider = InsideEstimator3d::new();
        let grid = lsm.get_grid();
        insider.set_grid(&grid);
        for k in 0..depth {
            for j in 0..height {
                for i in 0..width {
                    let p = Point3d::<i32>::new(i, j, k);
                    let index = indexer.get(&p) as usize;
                    if statuses.borrow()[index] != Status::Front {
                        if insider.is_inside(&p) {
                            assert_eq!(phi.borrow()[index], -params.wband as f64);
                        } else {
                            assert_eq!(phi.borrow()[index], params.wband as f64);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn initialize_over_all_2d() {
        let mut params = Parameters::new();
        params.wband = 3;
        let mut initial_front = InitialFront2d::new();
        initial_front.vertices[0] = Point2d::<i32>::new(10, 15);
        initial_front.vertices[1] = Point2d::<i32>::new(82, 74);

        let size = Rc::new(SpaceSize2d::new(101, 143));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let mut lsm = LevelSetMethod2d::new(params.clone(), Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);
        lsm.initialize_over_all(&initial_front);

        let phi = lsm.get_phi();
        let width = size.width;
        let height = size.height;
        let grid = lsm.get_grid();

        let indexer = lsm.get_indexer();
        let statuses = lsm.get_statuses();
        let mut insider = InsideEstimator2d::new();
        insider.set_grid(&grid);
        for j in 0..height {
            for i in 0..width {
                let p = Point2d::<i32>::new(i, j);
                let index = indexer.get(&p) as usize;
                if statuses.borrow()[index] != Status::Front {
                    if insider.is_inside(&p) {
                        assert_eq!(phi.borrow()[index], -params.wband as f64);
                    } else {
                        assert_eq!(phi.borrow()[index], params.wband as f64);
                    }
                }
            }
        }
    }

    fn make_input_gray(size: &SpaceSize3d, front: &InitialFront3d) -> Rc<RefCell<Vec<u8>>> {
        let mut gray = vec![1u8; size.total as usize];
        let left = front.vertices[0].x;
        let top = front.vertices[0].y;
        let front_ = front.vertices[0].z;
        let right = front.vertices[1].x;
        let bottom = front.vertices[1].y;
        let back = front.vertices[1].z;

        let indexer = Indexer3d::new(&size);

        for j in top..bottom {
            for i in left..right {
                let p = Point3d::<i32>::new(i, j, front_);
                let q = Point3d::<i32>::new(i, j, back);
                let p_index = indexer.get(&p) as usize;
                let q_index = indexer.get(&q) as usize;
                gray[p_index] = 0u8;
                gray[q_index] = 0u8;
            }
        }

        for j in top..bottom {
            for k in front_..back {
                let p = Point3d::<i32>::new(left, j, k);
                let q = Point3d::<i32>::new(right, j, k);
                let p_index = indexer.get(&p) as usize;
                let q_index = indexer.get(&q) as usize;
                gray[p_index] = 0u8;
                gray[q_index] = 0u8;
            }
        }

        for i in left..right {
            for k in front_..back {
                let p = Point3d::<i32>::new(i, top, k);
                let q = Point3d::<i32>::new(i, bottom, k);
                let p_index = indexer.get(&p) as usize;
                let q_index = indexer.get(&q) as usize;
                gray[p_index] = 0u8;
                gray[q_index] = 0u8;
            }
        }
        Rc::new(RefCell::new(gray))
    }

    #[test]
    fn set_speed_on_front_3d() {
        let mut params = Parameters::new();
        params.wband = 1;
        params.constant_speed = 1.0;
        params.gain = 2.0;

        let mut initial_front = InitialFront3d::new();
        let left = 2;
        let top = 3;
        let right = 8;
        let bottom = 7;
        let front = 3;
        let back = 7;

        initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
        initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

        let size = Rc::new(SpaceSize3d::new(11, 11, 11));
        let gray = make_input_gray(&size, &initial_front);

        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);
        lsm.initialize_over_all(&initial_front);

        lsm.calculate_speed_factors();
        lsm.initialize_narrow_band();

        let fs = lsm.set_speed_on_front();
        assert!(fs != 0.0);

        let width = size.width;
        let height = size.height;
        let area = width * height;
        let depth = size.depth;

        let speed = lsm.get_speed();
        for k in 0..depth {
            let ak = area * k;
            for j in 0..height {
                let wj = ak + width * j;
                for i in 0..width {
                    let index = (wj + i) as usize;
                    if (left <= i && i <= right && top <= j && j <= bottom)
                        && (k == front || k == back)
                    {
                        assert!(0.0 != speed.borrow()[index]);
                    } else if (left <= i && i <= right && front <= k && k <= back)
                        && (j == top || j == bottom)
                    {
                        assert!(0.0 != speed.borrow()[index]);
                    } else if ((top <= j && j <= bottom && front <= k && k <= back)
                        && (i == left || i == right))
                    {
                        assert!(0.0 != speed.borrow()[index]);
                    } else {
                        assert!(0.0 == speed.borrow()[index]);
                    }
                }
            }
        }
    }

    fn set_narrow_band(narrow_band: &mut Vec<Point3d<i32>>) {
        for k in 2..9 {
            for j in 2..9 {
                for i in 1..10 {
                    if (j == 5 && k == 5 && (i == 4 || i == 5 || i == 6)) {
                        //
                    } else {
                        narrow_band.push(Point3d::<i32>::new(i, j, k));
                    }
                }
            }
        }
    }

    #[test]
    fn clear_speed_within_narrow_band_3d() {
        let mut params = Parameters::new();
        params.wband = 1;
        params.constant_speed = 1.0;
        params.gain = 2.0;

        let mut initial_front = InitialFront3d::new();
        let left = 2;
        let top = 3;
        let right = 8;
        let bottom = 7;
        let front = 3;
        let back = 7;
        initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
        initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

        let size = Rc::new(SpaceSize3d::new(11, 11, 11));
        let gray = make_input_gray(&size, &initial_front);
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);
        lsm.initialize_over_all(&initial_front);

        // initialize narrow band
        let mut narrow_bands = lsm.get_narrow_bands();
        set_narrow_band(narrow_bands);

        // initialize phi
        let mut dphi = lsm.get_dphi();
        let s = dphi.borrow().len();
        for i in 0..s {
            dphi.borrow_mut()[i] = 1.0;
        }

        // speed
        let mut speed = lsm.get_speed();
        let s = speed.borrow().len();
        for i in 0..s {
            speed.borrow_mut()[i] = 1.0;
        }

        lsm.clear_speed_within_narrow_band(true);
        check_buffer(speed, Rc::clone(&size));
        check_buffer(dphi, Rc::clone(&size));
    }

    fn is_within_narrow_band(p: &Point3d<i32>) -> bool {
        let i = p.x;
        let j = p.y;
        let k = p.z;
        if (2 <= k && k <= 8) {
            if (2 <= j && j <= 8) {
                if (1 <= i && i <= 9) {
                    if (j == 5 && k == 5 && (i == 4 || i == 5 || i == 6)) {
                    } else {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn check_buffer(buffer: Rc<RefCell<Vec<f64>>>, size: Rc<SpaceSize3d>) {
        let w = size.width;
        let h = size.height;
        let d = size.depth;
        let a = w * h;

        for k in 0..d {
            let ak = a * k;
            for j in 0..h {
                let wj = ak + w * j;
                for i in 0..w {
                    let p = buffer.borrow()[(wj + i) as usize];
                    let q = Point3d::<i32>::new(i, j, k);
                    if is_within_narrow_band(&q) {
                        assert!(p == 0.0);
                    } else {
                        assert!(p == 1.0);
                    }
                }
            }
        }
    }

    #[test]
    fn set_speed_function_3d() {
        let mut params = Parameters::new();
        params.wband = 1;
        params.constant_speed = 1.0;
        params.gain = 2.0;

        let mut initial_front = InitialFront3d::new();
        let left = 2;
        let top = 3;
        let right = 8;
        let bottom = 7;
        let front = 3;
        let back = 7;

        initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
        initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

        let size = Rc::new(SpaceSize3d::new(11, 11, 11));
        let gray = make_input_gray(&size, &initial_front);
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
        lsm.initialize_along_front(&initial_front);
        lsm.initialize_over_all(&initial_front);

        lsm.calculate_speed_factors();
        lsm.initialize_narrow_band();

        let resets = true;
        let is_stopped = lsm.set_speed_function(resets);
        assert!(!is_stopped);

        let mut speed = lsm.get_speed();
        let width = size.width;
        let height = size.height;
        let area = width * height;
        let depth = size.depth;

        for k in 0..depth {
            let ak = area * k;
            for j in 0..height {
                let wj = ak + width * j;
                for i in 0..width {
                    let p = speed.borrow()[(wj + i) as usize];
                    if ((left <= i && i <= right && top <= j && j <= bottom)
                        && (k == front || k == back))
                    {
                        assert!(p != 0.0);
                    } else if ((left <= i && i <= right && front <= k && k <= back)
                        && (j == top || j == bottom))
                    {
                        assert!(p != 0.0);
                    } else if ((top <= j && j <= bottom && front <= k && k <= back)
                        && (i == left || i == right))
                    {
                        assert!(p != 0.0);
                    } else {
                        assert!(p == 0.0);
                    }
                }
            }
        }
    }
    /*
        #[test]
        fn copy_nearest_speed_to_narrow_band_3d() {
            let mut params = Parameters::new();
            params.wband = 3;
            params.constant_speed = 1.0;
            params.gain = 2.0;
            params.wreset = 1;

            let mut initial_front = InitialFront3d::new();
            let left = 2;
            let top = 3;
            let right = 8;
            let bottom = 7;
            let front = 3;
            let back = 7;

            initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
            initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

            let size = Rc::new(SpaceSize3d::new(11, 11, 11));
            let gray = make_input_gray(&size, &initial_front);
            let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));
            lsm.initialize_distance_map();
            lsm.initialize_along_front(&initial_front);
            lsm.initialize_over_all(&initial_front);
            lsm.calculate_speed_factors();

            let resets = true;

            lsm.clear_speed_within_narrow_band(resets);
            lsm.set_speed_on_front();

            lsm.copy_nearest_speed_to_narrow_band(resets);

            let mut status_map: HashMap<usize, Status> = HashMap::new();
            status_map.insert(0, Status::Farway);
            status_map.insert(1, Status::Band);
            status_map.insert(2, Status::ResetBand);
            status_map.insert(3, Status::Front);

            let status_answers: Vec<usize> = vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1,
                2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1,
                1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
                2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1,
                1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1,
                1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3,
                3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
                1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3,
                3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1,
                1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
                1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1,
                1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 2, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1,
                3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3,
                1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3,
                3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 3, 3,
                3, 3, 3, 3, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2,
                0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1,
                2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1,
                1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0,
                0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let statuses = lsm.get_statuses();

            for (i, status) in statuses.borrow().iter().enumerate() {
                let a = status_answers[i];
                let b = status_map[&a];
                let c = *status;
                assert_eq!(b, c);
                if i == 22 {
                    //i<=22までは良い。
                    break;
                }
            }
        }
    */
    #[test]
    fn register_to_narrow_band_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        params.constant_speed = 1.0;
        params.gain = 2.0;
        params.wreset = 1;

        let size = SpaceSize3d::new(11, 11, 11);
        let mut initial_front = InitialFront3d::new();
        let left = 2;
        let top = 3;
        let right = 8;
        let bottom = 7;
        let front = 3;
        let back = 7;
        initial_front.vertices[0] = Point3d::<i32>::new(left, top, front);
        initial_front.vertices[1] = Point3d::<i32>::new(right, bottom, back);

        let size = Rc::new(SpaceSize3d::new(11, 11, 11));
        let gray = make_input_gray(&size, &initial_front);
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));

        let mut statuses = lsm.get_statuses();

        statuses.borrow_mut()[0] = Status::Band;
        statuses.borrow_mut()[1] = Status::ResetBand;
        statuses.borrow_mut()[2] = Status::Front;

        lsm.register_to_narrow_band_();

        let narrow_band = lsm.get_narrow_bands();

        assert_eq!(narrow_band.len(), 3);
        assert_eq!(narrow_band[0], Point3d::<i32>::new(0, 0, 0));
        assert_eq!(narrow_band[1], Point3d::<i32>::new(1, 0, 0));
        assert_eq!(narrow_band[2], Point3d::<i32>::new(2, 0, 0));
    }

    #[test]
    fn propagate_front_3d() {
        let mut params = Parameters::new();
        params.wband = 3;
        params.constant_speed = 1.0;
        params.gain = 2.0;
        params.wreset = 1;
        params.time_step = 1.0;

        let size = Rc::new(SpaceSize3d::new(3, 3, 3));
        let gray = Rc::new(RefCell::new(vec![0u8]));
        let mut lsm = LevelSetMethod3d::new(params, Rc::clone(&size), Rc::clone(&gray));

        let phi = lsm.get_phi();
        let speed = lsm.get_speed();
        let narrow_band = lsm.get_narrow_bands();

        let p = Point3d::<i32>::new(1, 1, 1);
        narrow_band.push(p);
        speed.borrow_mut()[13] = 3.0; // positive

        let sphi = vec![
            0.0, 0.0, 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 4.0, 2.0, 6.0, 0.0, 5.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0,
        ];

        for i in 0..sphi.len() {
            phi.borrow_mut()[i] = sphi[i];
        }
        assert!(phi.borrow()[13] == 2.0);
        speed.borrow_mut()[13] = -3.0;
        lsm.propagate_front();
        assert_eq!(phi.borrow()[13], 2.0 + 3.0 * 91.0_f64.sqrt());
    }
}
