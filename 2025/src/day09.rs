use aoc;

/// Check if point is inside polygon or on edge using ray casting
fn is_point_in_polygon_or_on_edge(point: (usize, usize), polygon: &[(usize, usize)]) -> bool {
    let (x, y) = (point.0 as f64, point.1 as f64);
    let mut inside = false;
    let n = polygon.len();

    let mut jdx = n - 1;
    for i in 0..n {
        let (xi, yi) = (polygon[i].0 as f64, polygon[i].1 as f64);
        let (xj, yj) = (polygon[jdx].0 as f64, polygon[jdx].1 as f64);

        // check if point is on edge
        if is_point_on_line_segment((x, y), (xi, yi), (xj, yj)) {
            return true;
        }

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        jdx = i;
    }
    inside
}

/// Check if point lies on line segment
fn is_point_on_line_segment(point: (f64, f64), start: (f64, f64), end: (f64, f64)) -> bool {
    let (px, py) = point;
    let (sx, sy) = start;
    let (ex, ey) = end;

    // check if point is collinear with segment
    let cross_product = (py - sy) * (ex - sx) - (px - sx) * (ey - sy);
    if cross_product.abs() > 1e-10 {
        return false; // not collinear
    }

    // check if point is within segment bounds
    let dot_product = (px - sx) * (ex - sx) + (py - sy) * (ey - sy);
    let squared_length = (ex - sx) * (ex - sx) + (ey - sy) * (ey - sy);

    dot_product >= 0.0 && dot_product <= squared_length
}

/// Check if rectangle is fully contained within polygon
fn is_rectangle_in_polygon(
    corner1: (usize, usize),
    corner2: (usize, usize),
    polygon: &[(usize, usize)],
) -> bool {
    // calculate rectangle bounds and corners
    let x_min = corner1.0.min(corner2.0);
    let x_max = corner1.0.max(corner2.0);
    let y_min = corner1.1.min(corner2.1);
    let y_max = corner1.1.max(corner2.1);

    // check all 4 corners - they must all be inside or on edge/vertex
    let corners = [
        (x_min, y_min),
        (x_min, y_max),
        (x_max, y_min),
        (x_max, y_max),
    ];
    if !corners
        .iter()
        .all(|&corner| is_point_in_polygon_or_on_edge(corner, polygon))
    {
        return false;
    }

    // check if rectangle edges cross polygon edges (but not just touch)
    let rect_edges = [
        ((x_min as f64, y_min as f64), (x_max as f64, y_min as f64)), // top
        ((x_max as f64, y_min as f64), (x_max as f64, y_max as f64)), // right
        ((x_max as f64, y_max as f64), (x_min as f64, y_max as f64)), // bottom
        ((x_min as f64, y_max as f64), (x_min as f64, y_min as f64)), // left
    ];

    let n = polygon.len();
    for i in 0..n {
        let poly_edge = (
            (polygon[i].0 as f64, polygon[i].1 as f64),
            (polygon[(i + 1) % n].0 as f64, polygon[(i + 1) % n].1 as f64),
        );

        for &rect_edge in &rect_edges {
            if line_segments_cross_properly(rect_edge.0, rect_edge.1, poly_edge.0, poly_edge.1) {
                return false; // rectangle crosses polygon boundary improperly
            }
        }
    }

    true // all corners valid and no improper crossings
}

/// Check if two line segments cross properly (not just touch at endpoints)
fn line_segments_cross_properly(
    p1: (f64, f64),
    q1: (f64, f64),
    p2: (f64, f64),
    q2: (f64, f64),
) -> bool {
    fn orientation(p: (f64, f64), q: (f64, f64), r: (f64, f64)) -> i32 {
        let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
        if val.abs() < 1e-10 {
            0
        }
        // collinear
        else if val > 0.0 {
            1
        }
        // clockwise
        else {
            2
        } // counterclockwise
    }

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // only return true for proper intersection (not touching at endpoints)
    o1 != o2 && o3 != o4 && o1 != 0 && o2 != 0 && o3 != 0 && o4 != 0
}

/// Movie Theater
/// computational geometry problem
fn part(filename: &str, is_part1: bool) -> usize {
    let lines = aoc::read_lines(filename);
    let mut polygon = Vec::<(usize, usize)>::new();
    for line in lines.iter() {
        let bla = aoc::parse_delimited::<usize>(line, ',');
        polygon.push((bla[0], bla[1]));
    }
    let mut largest_area = 0usize;
    if is_part1 {
        // n choose 2 vertices
        for idx in 0..polygon.len() {
            for jdx in idx + 1..polygon.len() {
                let dx = (polygon[idx].0 as isize - polygon[jdx].0 as isize).abs() + 1;
                let dy = (polygon[idx].1 as isize - polygon[jdx].1 as isize).abs() + 1;
                let area = (dx * dy) as usize;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    } else {
        // n choose 2, but check if rect formed is entirely within polygon
        for idx in 0..polygon.len() {
            for jdx in idx + 1..polygon.len() {
                let dx = (polygon[idx].0 as isize - polygon[jdx].0 as isize).abs() + 1;
                let dy = (polygon[idx].1 as isize - polygon[jdx].1 as isize).abs() + 1;
                let area = (dx * dy) as usize;

                // early exit when area smaller than best
                if area <= largest_area {
                    continue;
                }

                // check if rectangle is fully contained within polygon
                let is_full = is_rectangle_in_polygon(polygon[idx], polygon[jdx], &polygon);
                if is_full {
                    largest_area = area;
                    // println!("{} {} largest_area updated: {}", idx, jdx, largest_area);
                }
            }
        }
    }
    return largest_area;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 50);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 24);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 120+ minutes.");
}
