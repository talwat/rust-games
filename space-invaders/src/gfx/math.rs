/// I'm a software engineer not a computer scientist :(
/// https://www.roguebasin.com/index.php/Bresenham's_Line_Algorithm#Rust
pub fn bresenham(x1: usize, y1: usize, x2: usize, y2: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::<(usize, usize)>::new();
    let mut x1 = x1 as i32;
    let mut y1 = y1 as i32;
    let mut x2 = x2 as i32;
    let mut y2 = y2 as i32;

    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }

    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }

    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2 + 1) {
        if is_steep {
            points.push((y as usize, x as usize));
        } else {
            points.push((x as usize, y as usize));
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len() / 2) {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }
    points
}
