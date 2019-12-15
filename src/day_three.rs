use std::fs::File;
use std::io::{BufRead, BufReader, Error};


#[cfg(test)]
mod trinity {
    use super::{Line, Point, get_min_manhattan, run_1, run_2, get_min_steps};


    #[test]
    fn test_horizontal() {
        let line = Line{ origin: Point{ x: 0, y: 4 }, destination: Point{ x: 3, y: 4 } };

        assert!(
            line.horizontal()
        );
    }


    #[test]
    fn test_line_contains_point() {
        let line = Line{ origin: Point{ x: 0, y: 4 }, destination: Point{ x: 3, y: 4 } };

        assert!(
            line.contains_point(&Point{ x: 1, y: 4 })
        );
    }


    #[test]
    fn test_line_contains_point_negative() {
        let line = Line{ origin: Point{ x: 0, y: 4 }, destination: Point{ x: -3, y: 4 } };

        assert!(
            line.contains_point(&Point{ x: -1, y: 4 })
        );
    }


    #[test]
    fn test_intersection() {
        let l1 = Line{ origin: Point{ x: 1, y: 0 }, destination: Point{ x: 1, y: 10 } };
        let l2 = Line{ origin: Point{ x: 0, y: 4 }, destination: Point{ x: 3, y: 4 } };

        assert_eq!(
            Point{ x: 1, y: 4 }, l1.intersection(&l2).unwrap()
        );
    }


    #[test]
    fn test_manhattan_one() {
        assert_eq!(
            6, get_min_manhattan(&[String::from("R8"), String::from("U5"), String::from("L5"), String::from("D3")],
                                &[String::from("U7"), String::from("R6"), String::from("D4"), String::from("L4")]).unwrap()
        );
    }


    #[test]
    fn test_manhattan_two() {
        assert_eq!(
            6, get_min_manhattan(&[String::from("R8"), String::from("U5"), String::from("L5"), String::from("D3")],
                                &[String::from("U7"), String::from("R6"), String::from("D4"), String::from("L4")]).unwrap()
        );
    }


    #[test]
    fn test_manhattan_three() {
        assert_eq!(
            159, get_min_manhattan(&[String::from("R75"), String::from("D30"), String::from("R83"), String::from("U83"),
                                    String::from("L12"), String::from("D49"), String::from("R71"), String::from("U7"),
                                    String::from("L72")],
                                    &[String::from("U62"), String::from("R66"), String::from("U55"), String::from("R34"),
                                    String::from("D71"), String::from("R55"), String::from("D58"), String::from("R83")]).unwrap()
        );
    }


    #[test]
    fn test_manhattan_four() {
        assert_eq!(
            135, get_min_manhattan(&[String::from("R98"), String::from("U47"), String::from("R26"), String::from("D63"),
                                    String::from("R33"), String::from("U87"), String::from("L62"), String::from("D20"),
                                    String::from("R33"), String::from("U53"), String::from("R51")],
                                    &[String::from("U98"), String::from("R91"), String::from("D20"), String::from("R16"),
                                    String::from("D67"), String::from("R40"), String::from("U7"), String::from("R15"),
                                    String::from("U6"), String::from("R7")]).unwrap()
        );
    }


    #[test]
    fn test_steps_one() {
        assert_eq!(
            30, get_min_steps(&[String::from("R8"), String::from("U5"), String::from("L5"), String::from("D3")],
                              &[String::from("U7"), String::from("R6"), String::from("D4"), String::from("L4")]).unwrap()
        );
    }


    #[test]
    fn test_steps_two() {
        assert_eq!(
            610, get_min_steps(&[String::from("R75"), String::from("D30"), String::from("R83"), String::from("U83"),
                                String::from("L12"), String::from("D49"), String::from("R71"), String::from("U7"),
                                String::from("L72")],
                                &[String::from("U62"), String::from("R66"), String::from("U55"), String::from("R34"),
                                String::from("D71"), String::from("R55"), String::from("D58"), String::from("R83")]).unwrap()
        );
    }


    #[test]
    fn test_steps_three() {
        assert_eq!(
            410, get_min_steps(&[String::from("R98"), String::from("U47"), String::from("R26"), String::from("D63"),
                                String::from("R33"), String::from("U87"), String::from("L62"), String::from("D20"),
                                String::from("R33"), String::from("U53"), String::from("R51")],
                                &[String::from("U98"), String::from("R91"), String::from("D20"), String::from("R16"),
                                String::from("D67"), String::from("R40"), String::from("U7"), String::from("R15"),
                                String::from("U6"), String::from("R7")]).unwrap()
        );
    }

    #[test]
    fn test_distance() {
        let p = Point{ x: 10, y: 55 };
        let p1 = Point{ x: 10, y: 20 };

        assert_eq!(
            p.distance(&p1), 35
        );
    }


    #[test]
    fn test_run_1() {
        assert_eq!(
            run_1(), Some(806)
        );
    }


    #[test]
    fn test_run_2() {
        assert_eq!(
            run_2(), Some(66076)
        );
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}


impl Point {
    fn manhattan_distance(&self, rhs: &Point) -> i64 {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs())
    }

    fn distance(&self, rhs: &Point) -> i64 {
        let x1 = self.x as f64;
        let y1 = self.y as f64;
        let x2 = rhs.x as f64;
        let y2 = rhs.y as f64;

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() as i64
    }
}


#[derive(Debug, Copy, Clone)]
struct Line {
    origin: Point,
    destination: Point,
}


impl Line {
    fn horizontal(&self) -> bool {
        self.origin.y == self.destination.y
    }

    fn contains_point(&self, point: &Point) -> bool {
        if self.horizontal() {
            if self.origin.x <= self.destination.x {
                return point.y == self.origin.y && self.origin.x <= point.x && point.x <= self.destination.x;
            }

            return point.y == self.origin.y && self.origin.x >= point.x && point.x >= self.destination.x;
        }

        if self.origin.y <= self.destination.y {
            return point.x == self.origin.x && self.origin.y <= point.y && point.y <= self.destination.y;
        }

        point.x == self.origin.x && self.origin.y >= point.y && point.y >= self.destination.y
    }

    fn intersection(&self, rhs: &Line) -> Option<Point> {
        let intersect = if self.horizontal() {
            Point{ x: rhs.origin.x, y: self.origin.y}
        } else {
            Point{ x: self.origin.x, y: rhs.origin.y}
        };

        if self.contains_point(&intersect) && rhs.contains_point(&intersect) {
            return Some(intersect);
        }

        None
    }
}


#[derive(Debug, Copy, Clone)]
struct Intersection {
    point: Point,
    distance: i64,
}


pub fn run_1() -> Option<i64> {
    let filename = "input/day_three.txt";
    let layouts = read_wire_layouts(filename).unwrap();
    get_min_manhattan(&layouts[0], &layouts[1])
}


pub fn run_2() ->  Option<i64> {
    let filename = "input/day_three.txt";
    let layouts = read_wire_layouts(filename).unwrap();
    get_min_steps(&layouts[0], &layouts[1])
}


fn read_wire_layouts(filename: &str) -> Result<Vec<Vec<String>>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut layouts = Vec::new();

    for line in reader.lines() {
        layouts.push(
            line?.split(',')
                .map(str::to_string)
                .collect()
        );
    }

    Ok(layouts)
}


fn get_min_manhattan(layouts1: &[String], layouts2: &[String] ) -> Option<i64> {
    let intersections = get_intersections(&layouts1, &layouts2);
    min_manhattan(&intersections)
}


fn get_intersections(wire1_layout: &[String], wire2_layout: &[String]) -> Vec<Intersection> {
    let wire1 = calculate_points(&wire1_layout);
    let wire2 = calculate_points(&wire2_layout);
    let mut intersections = Vec::new();
    let mut distance1 = 0;
    let mut distance2 = 0;

    for segment in wire1 {
        distance1 += segment.origin.distance(&segment.destination);
        for segment2 in wire2.iter() {
            distance2 += segment2.origin.distance(&segment2.destination);
            if let Some(intersection) = segment.intersection(&segment2) {
                let d1 = segment.origin.distance(&segment.destination) - segment.origin.distance(&intersection);
                let d2 = segment2.origin.distance(&segment2.destination) - segment2.origin.distance(&intersection);
                intersections.push(Intersection{ point: intersection, distance: (distance1 - d1 + distance2 - d2) });
            }
        }
        distance2 = 0;
    }

    intersections
}


fn min_manhattan(intersections: &[Intersection]) -> Option<i64> {
    let mut distances = Vec::new();
    let origin = Point{ x: 0, y: 0 };

    for intersection in intersections {
        if intersection.point == origin {
            continue;
        }

        distances.push(origin.manhattan_distance(&intersection.point));
    }

    Some(*distances.iter().min().unwrap())
}


fn calculate_points(wire_paths: &[String]) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut origin = Point{ x: 0, y: 0 };

    for path in wire_paths {
        let mut chars = path.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i64>().unwrap();

        let destination = match direction {    
            'U' => Point{ x: origin.x, y: origin.y + distance },
            'D' => Point{ x: origin.x, y: origin.y - distance },
            'R' => Point{ x: origin.x + distance, y: origin.y },
            'L' => Point{ x: origin.x - distance, y: origin.y },
            _ => panic!("Unexpected direction")
        };

        lines.push(Line{ origin, destination });
        origin = destination;   
    }

    lines
}


fn get_min_steps(layouts1: &[String], layouts2: &[String] ) -> Option<i64> {
    let intersections = get_intersections(&layouts1, &layouts2);
    let origin = Point{ x: 0, y: 0 };
    let mut distances = Vec::new();

    for intersection in intersections {
        if intersection.point == origin {
            continue;
        }
        distances.push(intersection.distance);
    }

    Some(*distances.iter().min().unwrap())
}
