use alloc::{vec::Vec, sync::Arc};
use embedded_graphics::{
    image::Image,
    prelude::{Point, Size}, pixelcolor::Rgb888, Drawable,
};
use tinybmp::Bmp;

use crate::{
    drivers::{BLOCK_DEVICE, GPU_DEVICE},
    sync::UPSafeCell,
};

use super::{Graphics, Component};

pub struct ImageComp {
    
    inner: UPSafeCell<ImageInner>,
}

pub struct ImageInner {
    image: &'static [u8],
    graphic: Graphics,
    parent: Option<Arc<dyn Component>>
}

impl ImageComp {
    pub fn new(size: Size, point: Point, v: &'static [u8],parent: Option<Arc<dyn Component>>) -> Self {
        unsafe {
            ImageComp {
                inner: UPSafeCell::new(ImageInner {
                    parent,
                    image: v,
                    graphic: Graphics {
                        size,
                        point,
                        drv: GPU_DEVICE.clone(),
                    },
                }),
            }
        }
    }
}

impl Component for ImageComp {
    fn paint(&self) {
        let mut inner = self.inner.try_exclusive_access().unwrap();
        let b = unsafe { 
            let len = inner.image.len();
            let ptr = inner.image
            .as_ptr() as *const u8;
            core::slice::from_raw_parts(ptr, len)
        };
        let bmp = Bmp::<Rgb888>::from_slice(b).unwrap();
        let point = match &inner.parent {
            Some(parent) => {
                let (_, point) = parent.bound();
                Point::new(point.x + inner.graphic.point.x, point.y + inner.graphic.point.y)
            }
            None => inner.graphic.point,
        };
        Image::new(&bmp, point,).draw(&mut inner.graphic);
    }

    fn add(&self, comp: alloc::sync::Arc<dyn Component>) {
        todo!()
    }

    fn bound(&self) -> (Size, Point) {
        let inner = self.inner.try_exclusive_access().unwrap();
        (inner.graphic.size, inner.graphic.point)
    }
}