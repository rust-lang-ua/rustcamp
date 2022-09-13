use step_1_6::*;

trait Command {}

trait CommandHandler< C : Command >
{
  type Context : ?Sized;
  type Result;

  fn handle_command( &self, cmd : &C, ctx : &mut Self::Context ) -> Self::Result;
}

struct Add;

impl Command for Add {}

impl CommandHandler< Add > for User
{
    type Context = dyn UserRepository;

    type Result = Result< (), CanNotAddUser >;

    fn handle_command( &self, _ : &Add, ctx : &mut Self::Context ) -> Self::Result
    {
      ctx.add( self.clone() )
    }
}

struct Get;

impl Command for Get {}

impl CommandHandler< Get > for User
{
    type Context = dyn UserRepository;

    type Result = Option< User >;

    fn handle_command( &self, _ : &Get, ctx : &mut Self::Context ) -> Self::Result
    {
      ctx.get( self.id ).cloned()
    }
}

struct Update;

impl Command for Update {}

impl CommandHandler< Update > for User
{
    type Context = dyn UserRepository;

    type Result = ();

    fn handle_command( &self, _ : &Update, ctx : &mut Self::Context ) -> Self::Result
    {
      ctx.update( self.clone() )
    }
}

struct Remove;

impl Command for Remove {}

impl CommandHandler< Remove > for User
{
    type Context = dyn UserRepository;

    type Result = Option< User >;

    fn handle_command( &self, _ : &Remove, ctx : &mut Self::Context ) -> Self::Result
    {
      ctx.remove( self.id )
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
    let mut user = User{ id : 1, email : "user@gmail.com".into(), activated : true };
    let mut rep = UserRepositoryStatic::new( DB::default() );

    assert_eq!( None, user.handle_command( &Get, &mut rep) );

    assert!( user.handle_command( &Add, &mut rep).is_ok() );
    assert_eq!( Some( user.clone() ), user.handle_command( &Get, &mut rep) );

    user.activated = false;
    user.handle_command( &Update, &mut rep);
    assert_eq!( Some( user.clone() ), user.handle_command( &Get, &mut rep) );

    assert!( user.handle_command( &Remove, &mut rep).is_some() );
    assert_eq!( None, user.handle_command( &Get, &mut rep) );
  }

  #[ test ]
  fn dynamic_repository()
  {
    let mut user = User{ id : 1, email : "user@gmail.com".into(), activated : true };
    let mut rep = UserRepositoryDynamic::new( Box::new( DB::default() ) );

    assert_eq!( None, user.handle_command( &Get, &mut rep) );

    assert!( user.handle_command( &Add, &mut rep).is_ok() );
    assert_eq!( Some( user.clone() ), user.handle_command( &Get, &mut rep) );

    user.activated = false;
    user.handle_command( &Update, &mut rep);
    assert_eq!( Some( user.clone() ), user.handle_command( &Get, &mut rep) );

    assert!( user.handle_command( &Remove, &mut rep).is_some() );
    assert_eq!( None, user.handle_command( &Get, &mut rep) );
  }
}