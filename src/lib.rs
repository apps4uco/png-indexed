#![deny(missing_docs)]
//! Creates a png image based on a palette.
//! Useful to minimize the file size of images for storage or transmission.
//!
//! - [x] Png Support 1 bit per pixel - 2 colors
//! - [ ] Png Support 2 bits per pixel - 4 colors
//! - [ ] Png Support 4 bits per pixel - 16 colors
//! - [ ] Png Support 8 bits per pixel - 256 colors
//!
// use bit_vec::BitVec;
use bitvec::prelude::*;
use png::{BitDepth, Compression};

/// Image Buffer for indexed png images
pub struct PngIndexed {
    /// width of image
    width:u32,
    /// height of image
    height:u32,
    /// stride of image, calculated as width rounded up to next highest multiple of 8
    stride:u32,
    /// data as a bit vector
    data: BitVec<u8,Msb0>
    // data:BitVec
}

impl PngIndexed {
    /// Create a new Image Buffer for indexed png images
    pub fn new(width:u32, height:u32)->Self {
        let stride=((width+7)/8)*8;
        let total_bits=(stride*height) as usize;
        //TODO make the background palette index configurable
        let background=0;
        let data = bitvec![u8, Msb0; background; total_bits];
        //let mut data=BitVec::from_elem(total_bits,false);
        Self {
            width,
            height,
            stride,
            data
        }
    }

    /// Obtain the width of the image
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Obtain the height of the image
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Put a pixel at x,y
    /// 0,0 is at the top left of the image
    /// pixel is the index of the palette item numbered from 0
    pub fn put_pixel(&mut self,x:u32,y:u32,pixel:u8) {
        let index=(y*self.stride+x) as usize;
        let value=pixel !=0;
        self.data.set(index,value);
    }

    /// Obtains the data as a png image
    pub fn to_png(&self) -> Vec<u8> {
        let palette:Vec<u8>=vec![255,255,255,0,0,0];
        // let PALETTE:Vec<u8>=vec![0,0,0,255,255,255];

        let mut w=Vec::new();

        let mut encoder = png::Encoder::new(&mut w, self.width, self.height); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_palette(&palette);
        // encoder.set_compression(Compression::Best);
        //TODO encoder.set_depth(self.bit_depth);
        encoder.set_depth(BitDepth::One);

        //encoder.set_trns(vec![0xFFu8,0x00u8]);

        let mut writer = encoder.write_header().unwrap();
        let data=self.data.as_raw_slice();
        //let data=&self.data.to_bytes();
        writer.write_image_data(data).unwrap();
        writer.finish().unwrap();
        w
    }
}


#[cfg(test)]
mod tests {
    use image::GenericImageView;
    use super::*;

    #[test]
    fn it_works() {
        let width=2;
        let height=2;
        let mut png=PngIndexed::new(width,height);
        png.put_pixel(0,0,1);
        png.put_pixel(1,1,1);
        let res=png.to_png();
        std::fs::write("/tmp/binary.png",res).unwrap();
    }

    #[test]
    fn load_image()-> anyhow::Result<()> {
        use image::io::Reader as ImageReader;

        let img = ImageReader::open("./assets/rustacean-flat-noshadow.png")?.decode()?;
        let mut png=PngIndexed::new(img.width(),img.height());
        img.pixels().for_each(|(x,y,p)|{
            let value=if p.0[0]>128 {1} else {0};

            png.put_pixel(x,y,value);
        });
        let res=png.to_png();
        std::fs::write("/tmp/ferris.png",res).unwrap();
        Ok(())
    }
}
