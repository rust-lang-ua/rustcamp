use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64
}

#[derive(Clone)]
pub struct Polyline {
    points: Vec<Point>
}

impl Polyline {
    pub fn new() -> Self {
        Self {
            points: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn add(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn remove(&mut self, point: &Point) -> Option<Point> {
        self.points.iter()
            .position(|&p| p == *point)
            .map(|idx| self.points.remove(idx))
    }

    pub fn point(&self, point: &Point) -> Option<&Point> {
        self.points.iter().find(|&p| p == point)
    }

    pub fn point_mut(&mut self, point: &Point) -> Option<&mut Point> {
        self.points.iter_mut().find(|p| *p == point)
    }

    pub fn contains(&self, p: &Point) -> bool {
        self.point(p).is_some()
    }
}

impl Index<usize> for Polyline {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Polyline {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_len_is_empty_when_created() {
        let line = Polyline::new();
        assert_eq!(line.points.len(), 0);
    }

    #[test]
    fn test_add_two_points_should_return_len_equals_two() {
        let mut line = Polyline::new();
        line.add(Point::default());
        line.add(Point::default());
        assert_eq!(line.points.len(), 2);
    }

    #[test]
    fn test_remove_should_return_the_same_instance_as_was_added() {
        let p_in = Point { x: 12.0, y: 42.0 };
        let mut line = Polyline::new();
        line.add(p_in);
        let p_out = line.remove(&p_in).unwrap();
        assert!(p_in == p_out);
    }

    #[test]
    fn test_get_at_index_1_should_return_expected_res_when_called_for_triangle() {
        let p0 = Point { x: 1.0, y: 1.0 };
        let p1 = Point { x: 2.0, y: 2.0 };
        let p2 = Point { x: 3.0, y: 3.0 };
        let mut line = Polyline::new();
        line.add(p0);
        line.add(p1);
        line.add(p2);
        assert_eq!(line[1], p1);
    }

    #[test]
    fn test_contains() {
        let p0 = Point { x: 1.0, y: 1.0 };
        let p1 = Point { x: 2.0, y: 2.0 };
        let p2 = Point { x: 3.0, y: 3.0 };
        let mut line = Polyline::new();
        line.add(p0);
        line.add(p1);
        line.add(p2);
        assert!(line.contains(&p0));
        assert!(!line.contains(&Point { x: 0.0, y: 0.0 }));
    }
}

