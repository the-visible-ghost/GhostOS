use crate::buffer::Buffer;

pub fn circle(buffer: &mut Buffer, center: (isize, isize), radius: isize, fill: (u8, u8, u8)) {
    for x in center.0 - radius..=center.0 + radius {
        for y in center.1 - radius..=center.1 + radius {
            let d: (isize, isize) = (x - center.0, y - center.1);
            if d.0 * d.0 + d.1 * d.1 < radius * radius {
                let pixel = buffer.pixel(x as usize, y as usize).unwrap();
                pixel.red = fill.0;
                pixel.green = fill.1;
                pixel.blue = fill.2;
            }
        }
    }
}
