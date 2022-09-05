fn main()
{
  println!( "Implement me!" );
}

fn parse( input : &str ) -> ( Option< Sign >, Option< usize >, Option< Precision > ) { unimplemented!() }

#[ derive( Debug, PartialEq ) ]
enum Sign
{
  Plus,
  Minus,
}

#[ derive( Debug, PartialEq ) ]
enum Precision
{
  Integer( usize ),
  Argument( usize ),
  Asterisk,
}

#[ cfg( test ) ]
mod spec
{
  use super::*;

  #[ test ]
  fn parses_sign()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", None ),
      ( ">+8.*", Some( Sign::Plus ) ),
      ( "-.1$x", Some( Sign::Minus ) ),
      ( "a^#043.8?", None ),
    ]
    {
      let ( sign, .. ) = parse( input );
      assert_eq!( sign, expected );
    }
  }

  #[ test ]
  fn parses_width()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", Some( 8 ) ),
      ( ">+8.*", Some( 8 ) ),
      ( "-.1$x", None ),
      ( "a^#043.8?", Some( 43 ) ),
    ]
    {
      let ( _, width, _ ) = parse( input );
      assert_eq!( width, expected );
    }
  }

  #[ test ]
  fn parses_precision()
  {
    for ( input, expected ) in vec!
    [
      ( "", None ),
      ( ">8.*", Some( Precision::Asterisk ) ),
      ( ">+8.*", Some( Precision::Asterisk ) ),
      ( "-.1$x", Some( Precision::Argument( 1 ) ) ),
      ( "a^#043.8?", Some( Precision::Integer( 8 ) ) ),
    ]
    {
      let ( _, _, precision ) = parse( input );
      assert_eq!( precision, expected );
    }
  }
}
