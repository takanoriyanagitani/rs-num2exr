use std::io::{Seek, Write};
use std::path::Path;

use exr::error::Error;
use exr::prelude::{f16, Image, SpecificChannels, WritableImage};

use crate::source::pos2val1d;

pub fn sz2height(sz: usize, width: usize) -> usize {
    let d: usize = sz / width;
    let mul: usize = d * width;
    let dlt: usize = sz - mul;
    let add: usize = match 0 < dlt {
        true => 1,
        false => 0,
    };
    add + d
}

macro_rules! slice2wtr {
    ($fname: ident, $typ: ty) => {
        pub fn $fname<W>(
            s: &[$typ],
            width: usize,
            wtr: W,
            chname: &str,
            alt: $typ,
        ) -> Result<(), Error>
        where
            W: Write + Seek,
        {
            let h: usize = sz2height(s.len(), width);
            let pixels = SpecificChannels::build()
                .with_channel(chname)
                .with_pixel_fn(|pos| {
                    let f: $typ = pos2val1d(&|ix: usize| s.get(ix).copied(), width, alt, pos);
                    (f,)
                });
            let img = Image::from_channels((width, h), pixels);
            img.write().to_buffered(wtr)
        }
    };
}

slice2wtr!(slice2wtr32f, f32);
slice2wtr!(slice2wtr16f, f16);
slice2wtr!(slice2wtr32u, u32);

macro_rules! slice2path {
    ($fname: ident, $typ: ty) => {
        pub fn $fname<P>(
            s: &[$typ],
            width: usize,
            filepath: P,
            chname: &str,
            alt: $typ,
        ) -> Result<(), Error>
        where
            P: AsRef<Path>,
        {
            let h: usize = sz2height(s.len(), width);
            let pixels = SpecificChannels::build()
                .with_channel(chname)
                .with_pixel_fn(|pos| {
                    let f: $typ = pos2val1d(&|ix: usize| s.get(ix).copied(), width, alt, pos);
                    (f,)
                });
            let img = Image::from_channels((width, h), pixels);
            img.write().to_file(filepath)
        }
    };
}

slice2path!(slice2path32f, f32);
slice2path!(slice2path16f, f16);
slice2path!(slice2path32u, u32);

#[cfg(test)]
mod test_img {

    mod sz2height {

        #[test]
        fn empty1() {
            assert_eq!(crate::img::sz2height(0, 1), 0);
        }

        #[test]
        fn empty2() {
            assert_eq!(crate::img::sz2height(0, 2), 0);
        }

        #[test]
        fn empty3() {
            assert_eq!(crate::img::sz2height(0, 3), 0);
        }

        #[test]
        fn single1() {
            assert_eq!(crate::img::sz2height(1, 1), 1);
        }

        #[test]
        fn single2() {
            assert_eq!(crate::img::sz2height(1, 2), 1);
        }

        #[test]
        fn single3() {
            assert_eq!(crate::img::sz2height(1, 3), 1);
        }

        #[test]
        fn double1() {
            assert_eq!(crate::img::sz2height(2, 1), 2);
        }

        #[test]
        fn double2() {
            assert_eq!(crate::img::sz2height(2, 2), 1);
        }

        #[test]
        fn double3() {
            assert_eq!(crate::img::sz2height(2, 3), 1);
        }

        #[test]
        fn triple1() {
            assert_eq!(crate::img::sz2height(3, 1), 3);
        }

        #[test]
        fn triple2() {
            assert_eq!(crate::img::sz2height(3, 2), 2);
        }

        #[test]
        fn triple3() {
            assert_eq!(crate::img::sz2height(3, 3), 1);
        }

        #[test]
        fn iv1() {
            assert_eq!(crate::img::sz2height(4, 1), 4);
        }

        #[test]
        fn iv2() {
            assert_eq!(crate::img::sz2height(4, 2), 2);
        }

        #[test]
        fn iv3() {
            assert_eq!(crate::img::sz2height(4, 3), 2);
        }

        #[test]
        fn iv4() {
            assert_eq!(crate::img::sz2height(4, 4), 1);
        }

        #[test]
        fn sky25() {
            assert_eq!(crate::img::sz2height(634, 25), 26);
        }

        #[test]
        fn sky42() {
            assert_eq!(crate::img::sz2height(634, 42), 16);
        }

        #[test]
        fn run205() {
            assert_eq!(crate::img::sz2height(42195, 205), 206);
        }

        #[test]
        fn fuji599() {
            assert_eq!(crate::img::sz2height(3776, 599), 7);
        }

        #[test]
        fn atm333() {
            assert_eq!(crate::img::sz2height(101325, 333), 305);
        }

        #[test]
        fn c16383() {
            assert_eq!(crate::img::sz2height(299792458, 16383), 18299);
        }
    }
}
