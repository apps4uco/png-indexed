use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{ColorType, ImageBuffer, Luma, PixelWithColorType};
use image::codecs::png::PngEncoder;
use pngindexed::PngIndexed;

pub fn pngindexed_benchmark(c: &mut Criterion) {
    c.bench_function("pngindexed", |b| b.iter(|| {
        let mut png=PngIndexed::new(black_box(1000),black_box(1000));
        for y in 0..png.height() {
            for x in (0..png.width()).step_by(2) {
                png.put_pixel(x,y,1);
            }
        }
        png.to_png();
    }));
}

pub fn image_benchmark(c: &mut Criterion) {
    c.bench_function("pngindexed", |b| b.iter(|| {
        // let mut image=ImageBuffer::new(black_box(1000),black_box(1000));
        // image.put_pixel(0,0,Luma([0u8]));

        let image = ImageBuffer::from_fn(1000, 1000, |x, y| {
            if x % 2 == 0 {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        });

        let mut buf = Vec::new();
        let encoder = PngEncoder::new(&mut buf);
        encoder.encode(&image, image.width(), image.height(),ColorType::L8).unwrap();

    }));
}



criterion_group!(benches, pngindexed_benchmark, image_benchmark);
criterion_main!(benches);