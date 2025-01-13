#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt() // The formula for Euclidean distance in two dimensions
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(initial_point: Point) -> Self {
        Polyline {
            points: vec![initial_point],
        }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn remove_point(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }
    
    //custom methods
    fn length(&self) -> f64 { 
        self.points
            .windows(2)
            .map(|window| window[0].distance_to(&window[1]))
            .sum()
    }

    fn clear(&mut self) {
        self.points.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_point() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        polyline.add_point(Point::new(1.0, 1.0));
        assert_eq!(polyline.points.len(), 2);
        assert_eq!(polyline.get_point(1), Some(&Point::new(1.0, 1.0)));
    }

    #[test]
    fn test_remove_point() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        polyline.add_point(Point::new(1.0, 1.0));
        let removed = polyline.remove_point(1);
        assert_eq!(removed, Some(Point::new(1.0, 1.0)));
        assert_eq!(polyline.points.len(), 1);
    }

    #[test]
    fn test_remove_point_out_of_bounds() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        let removed = polyline.remove_point(1);
        assert_eq!(removed, None);
    }

    #[test]
    fn test_get_point() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        polyline.add_point(Point::new(1.0, 1.0));
        assert_eq!(polyline.get_point(0), Some(&Point::new(0.0, 0.0)));
        assert_eq!(polyline.get_point(1), Some(&Point::new(1.0, 1.0)));
    }

    #[test]
    fn test_length() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        polyline.add_point(Point::new(3.0, 4.0));
        assert_eq!(polyline.length(), 5.0);
    }

    #[test]
    fn test_clear() {
        let mut polyline = Polyline::new(Point::new(0.0, 0.0));
        polyline.add_point(Point::new(1.0, 1.0));
        polyline.clear();
        assert!(polyline.points.is_empty());
    }
}
