use std::{ borrow::Cow, collections::HashMap };

pub trait Storage< K, V >
{
  fn set( &mut self, key : K, val : V );
  fn get( &self, key : &K ) -> Option< &V >;
  fn remove( &mut self, key : &K ) -> Option< V >;
}

#[ derive( Debug ) ]
pub struct CanNotAddUser;

pub trait UserRepository
{
  fn add( &mut self, user : User ) -> Result< (), CanNotAddUser >;
  fn get( &self, key : u64 ) -> Option< &User >;
  fn update( &mut self, user : User );
  fn remove( &mut self, key : u64 ) -> Option< User >;
}

#[ derive( Debug, Default, Clone, PartialEq ) ]
pub struct User
{
  pub id : u64,
  pub email : Cow< 'static, str >,
  pub activated : bool,
}

#[ derive( Default ) ]
pub struct DB< K, V >
{
  storage : HashMap< K, V >,
}

impl< K : Eq + std::hash::Hash, V > Storage< K, V > for DB< K, V >
{
  fn set( &mut self, key : K, val : V )
  {
    self.storage.insert( key, val );
  }

  fn get( &self, key : &K ) -> Option< &V > {
    self.storage.get( key )
  }

  fn remove( &mut self, key : &K ) -> Option< V > {
    self.storage.remove( key )
  }
}

pub struct UserRepositoryStatic< S : Storage< u64, User > >
{
  storage : S
}

impl< S > UserRepositoryStatic< S >
where
  S : Storage< u64, User >
{
  pub fn new( storage : S ) -> Self
  {
    Self { storage }
  }
}

impl< S > UserRepository for UserRepositoryStatic< S >
where
  S : Storage< u64, User >
{
  fn add( &mut self, user : User ) -> Result< (), CanNotAddUser >
  {
    if self.storage.get( &user.id ).is_some()
    {
      return Err( CanNotAddUser );
    }
    Ok( self.storage.set( user.id, user ) )
  }

  fn get( &self, key : u64 ) -> Option< &User >
  {
    self.storage.get( &key )
  }

  fn update( &mut self, user : User )
  {
    self.storage.set( user.id, user )
  }

  fn remove( &mut self, key : u64 ) -> Option< User >
  {
    self.storage.remove( &key )
  }
}

pub struct UserRepositoryDynamic
{
  storage : Box< dyn Storage< u64, User > >
}

impl UserRepositoryDynamic
{
  pub fn new( storage : Box< dyn Storage< u64, User > > ) -> Self
  {
    Self { storage }
  }
}

// it must be created with derive macro
impl UserRepository for UserRepositoryDynamic
{
  fn add( &mut self, user : User ) -> Result< (), CanNotAddUser >
  {
    if self.storage.get( &user.id ).is_some()
    {
      return Err( CanNotAddUser );
    }
    Ok( self.storage.set( user.id, user ) )
  }

  fn get( &self, key : u64 ) -> Option< &User >
  {
    self.storage.get( &key )
  }

  fn update( &mut self, user : User )
  {
    self.storage.set( user.id, user )
  }

  fn remove( &mut self, key : u64 ) -> Option< User >
  {
    self.storage.remove( &key )
  }
}

fn main() {}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn static_repository()
  {
    let mut rep = UserRepositoryStatic{ storage : DB::default() };

    let mut user = User{ id : 1, email : "user@gmail.com".into(), activated : true };
    assert!( rep.add( user.clone() ).is_ok() );

    assert_eq!( 1, rep.storage.storage.len() );
    assert_eq!( Some( &user ), rep.get( 1 ) );

    user.activated = false;
    rep.update( user.clone() );

    assert_eq!( Some( &user ), rep.get( 1 ) );

    assert_eq!( Some( user ), rep.remove( 1 ) );
    assert_eq!( 0, rep.storage.storage.len() );

    assert_eq!( None, rep.get( 100 ) );
    assert_eq!( None, rep.remove( 100 ) );
  }

  #[ test ]
  fn dynamic_repository()
  {
    let mut rep = UserRepositoryDynamic{ storage : Box::new( DB::default() ) };

    let mut user = User{ id : 1, email : "user@gmail.com".into(), activated : true };
    assert!( rep.add( user.clone() ).is_ok() );

    assert_eq!( Some( &user ), rep.get( 1 ) );

    user.activated = false;
    rep.update( user.clone() );

    assert_eq!( Some( &user ), rep.get( 1 ) );
    assert_eq!( Some( user ), rep.remove( 1 ) );

    assert_eq!( None, rep.get( 100 ) );
    assert_eq!( None, rep.remove( 100 ) );
  }
}
