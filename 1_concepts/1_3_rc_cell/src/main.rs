use std::sync::{Arc, Mutex};

#[ derive( Debug, Clone ) ]
struct GlobalStack< T >( Arc< Mutex< Vec< T > > > );

impl< T > GlobalStack< T >
{
  fn new() -> Self
  {
    Self( Arc::new( Mutex::new( vec![] ) ) ) 
  }

  fn push( &self, value : T )
  {
    let mut handler = self.0.lock().unwrap();
    ( *handler ).push( value );
  }
}

impl < T : Clone > GlobalStack< T > 
{
  fn get( &self ) -> Vec< T >
  {
    let catch = self.0.lock().unwrap();
    catch.to_vec()
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn basic()
  {
    let stack = GlobalStack::new();
    stack.push( "Element" );
    assert_eq!( [ "Element" ], stack.get().as_slice() );
  }

  #[ test ]
  fn multiple_shared_references()
  {
    let stack = GlobalStack::new();
    let stack_ref = &stack;
    let stack_ref_2 = &stack_ref;

    stack.push( 1 );
    assert_eq!( [ 1 ], stack.get().as_slice() );
    stack_ref.push( 2 );
    assert_eq!( [ 1, 2 ], stack.get().as_slice() );
    stack_ref_2.push( 3 );
    assert_eq!( [ 1, 2, 3 ], stack.get().as_slice() );
  }

  #[ test ]
  fn cloning_doesnt_clone_data()
  {
    let stack = GlobalStack::new();
    let stack_clone = stack.clone();
    
    stack.push( "Is it cloning?" );
    assert_eq!( stack.get(), stack_clone.get() );
  }
}