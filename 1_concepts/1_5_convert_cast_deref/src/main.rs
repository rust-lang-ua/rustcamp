use std::ops::Deref;


#[ derive( Debug, PartialEq ) ]
pub struct InvalidEmail;

#[ derive( Debug, PartialEq ) ]
pub struct EmailString< 'a >( &'a str );

impl< 'a > TryFrom< &'a str > for EmailString< 'a >
{
  type Error = InvalidEmail;

  fn try_from( value : &'a str ) -> Result< Self, Self::Error >
  {
    if validator::validate_email( value )
    {
      Ok( Self( value ) )
    }
    else 
    {
      Err( InvalidEmail )
    }
  }
}

pub struct Random< T >
{
  pub values : [ T; 3 ],
  _iter : std::cell::RefCell< Box< dyn Iterator< Item = usize > > > 
} 

impl< T > Random< T >
{
  pub fn new( first : T, second : T, third : T ) -> Self
  {
    Self 
    {
      values : [ first, second, third ],
      _iter : std::cell::RefCell::new
      (
        Box::new
        ({
          // pseudo random iterator
          let mut seed = 92;
          std::iter::repeat_with
          (
            move | | 
            {
              seed ^= seed << 13;
              seed ^= seed >> 17;
              seed ^= seed << 5;
              seed
            }
          )
        })
      )
    }
  }
}

impl< T > Deref for Random< T >
{
  type Target = T;

  fn deref( &self ) -> &Self::Target {
    &self.values
    [
      self._iter.borrow_mut().next().unwrap() % self.values.len()
    ]
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn valid_email()
  {
    assert_eq!( 
      Ok( EmailString( "a@gmail.com".into() ) ),
      EmailString::try_from( "a@gmail.com" )
    );
    assert_eq!( 
      Ok( EmailString( "some.email_address5@gmail.com".into() ) ),
      EmailString::try_from( "some.email_address5@gmail.com" )
    );
  }

  #[ test ]
  fn invalid_email()
  {
    assert_eq!( 
      Err( InvalidEmail ),
      EmailString::try_from( "agmail.com" )
    );
    assert_eq!( 
      Err( InvalidEmail ),
      EmailString::try_from( "some.email_address5@@gmail.com" )
    );
  }

  // this test works for seed = 92
  #[ test ]
  fn random_gets_all_values()
  {
    let rand = Random::new( 1, 2, 3 );
    assert_eq!( 1, *rand );
    assert_eq!( 3, *rand );
    assert_eq!( 2, *rand );

    let rand = Random::new( "first", "second", "third" );
    assert_eq!( "first", *rand );
    assert_eq!( "third", *rand );
    assert_eq!( "second", *rand );
  }
}