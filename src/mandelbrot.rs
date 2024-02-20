use std::usize;

use num::Complex;


fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    //keep running the loop up to the limit to determine whether z is stable or unstable
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            //i in this case is how long it took to fly into infinity.
            //z is unstable, return i which is how many iterations it took to fly out to infinity -> become unstable.
            //i is the number of the iteration at which z left the circle of radius 2.
            return Some(i);
        }
        z = z * z + c;
    }

    //it doesnt fly out to infity -> it is stable. c is a member in a mandelbrot set
    //key here is probably be a member as it is a limited number of iterations
    None
}

/*
    pixel_to_point is a map from a pixel on the image to a point on the complex plane
    This point on the complex plane(c) is what we will check whether it belongs in a mandelbrot set or not.

    what about upper_left & lower_right?
    These are the 'bounds' of our square, this are the points we want to check. This is because outside this,
    the point are unstable so no need to check. we are mapping from our image bounds to this tiny area and then
    checking whether this complex point is stable, if stable we color accordingly on our image.

    upper_left = Complex { re: -1.0, im: 1.0 },
    lower_right = Complex { re: 1.0, im: -1.0 }),
*/
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let pixel: (usize, usize) = (col, row);
            let point = pixel_to_point(bounds, pixel, upper_left, lower_right);
            let pixel_index = row * bounds.0 + col; //multiply the row & add the column to get the specifix pixel
            pixels[pixel_index] = match escape_time(point, 255) {
                None => 0, //shade black,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
