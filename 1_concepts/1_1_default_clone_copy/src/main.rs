#[ derive( Debug, Default, Copy, Clone, PartialEq ) ]
struct Point
{
  x : f32,
  y : f32,
}

#[ derive( Debug, Clone, PartialEq ) ]
struct Polyline( Vec< Point >);

fn main()
{
  println!( "Implement me!" );
}

#[ cfg( test ) ]
mod tests {
  use super::*;

  #[ test ]
  fn default_point()
  {
    let point : Point = Default::default();
    assert_eq!( Point{ x : 0.0, y : 0.0 }, point );
  }

  #[ test ]
  fn copy_point() 
  {
    let point = Point{ x : 16.0, y : -2.9 };
    let copy_point = point;
    assert_eq!( point, copy_point );
  }

  #[ test ]
  fn copy_polyline()
  {
    let polyline = Polyline( 
      vec![ Point::default(), Point::default() ]
    );
    let copy_polyline = polyline.clone();
    assert_eq!( polyline, copy_polyline );
  }

  #[ test ]
  fn non_default_polyline()
  {
    // fails at compile time cuz we don't want to use the Default
    // let polyline = Polyline::default();
  }
}