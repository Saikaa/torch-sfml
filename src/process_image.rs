use image;
use tch::{Kind, Tensor};

const IMAGE_WIDTH: usize = 28;
const IMAGE_HEIGHT: usize = 28;
const PIXELS: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

pub fn array_to_tensor(image_data: &[f32; 784]) -> Tensor {
    let m_data: *const [f32] = &normalize(image_data);
    Tensor::of_data_size(
        unsafe { &*(m_data as *const [u8]) },
        &[1, IMAGE_WIDTH as i64, IMAGE_HEIGHT as i64],
        Kind::Float,
    )
}

fn normalize(image_data: &[f32; PIXELS]) -> [f32; PIXELS] {
    let mean = 0.5;
    let std_dev = 0.5;
    image_data
        .iter()
        .map(|&x| (x - mean) / std_dev)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn load_image(path: &str) -> Tensor {
    let image = image::open(path).unwrap().to_luma32f();
    let mut data = [0f32; 784];

    let mut i = 0;
    for x in 0..28 {
        for y in 0..28 {
            data[i] = image.get_pixel(y, x).0[0];

            i += 1;
        }
    }

    array_to_tensor(&data)
}
