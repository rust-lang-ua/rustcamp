use std::pin::Pin;

trait MutMeSomehow 
{
  fn mut_me_somehow( self: Pin< &mut Self > );
}

trait SayHi: std::fmt::Debug 
{
  fn say_hi( self: Pin< &Self > ) 
  {
    println!( "Hi from {:?}", self )
  }
}

impl< T : Default > MutMeSomehow for Box< T >
{
  fn mut_me_somehow( mut self: Pin< &mut Self > ) 
  {
    self.set( Box::new( T::default() ) );
  }
}

impl< T : Default > MutMeSomehow for Vec< T >
{
  fn mut_me_somehow( mut self: Pin< &mut Self > ) 
  {
    self.set( vec![ T::default() ] );
  }
}

impl MutMeSomehow for String
{
  fn mut_me_somehow( mut self: Pin< &mut Self > ) 
  {
    self.set( String::from( "Hello" ) );
  }
}

impl< T : std::fmt::Debug > SayHi for Box< T > {}
impl< T : std::fmt::Debug > SayHi for Vec< T > {}
impl SayHi for String {}


fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn mut_pinned_box()
  {
    let mut data = Box::new( "Hi" );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( *data, "" );
  }

  #[ test ]
  fn mut_pinned_vec()
  {
    let mut data = vec![ 1, 2, 3];
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( data, vec![ 0 ] );
  }

  #[ test ]
  fn mut_pinned_string()
  {
    let mut data = String::from( "World" );
    Pin::new( &mut data ).mut_me_somehow();
    assert_eq!( &data, "Hello" );
  }

  #[ test ]
  fn say_hi_box()
  {
    let data = Box::new( "Hi" );
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_vec()
  {
    let data = vec![ 1, 2, 3];
    Pin::new( &data ).say_hi();
  }

  #[ test ]
  fn say_hi_string()
  {
    let data = String::new();
    Pin::new( &data ).say_hi();
  }
}
