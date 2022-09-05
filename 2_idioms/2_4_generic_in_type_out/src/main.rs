use std::net::{ IpAddr, SocketAddr };

fn main()
{
  println!( "Refactor me!" );

  let mut err = Error::new( "NO_USER".to_string() );
  err.status( 404 ).message( "User not found".to_string() );
}

#[ derive( Debug ) ]
pub struct Error
{
  code : String,
  status : u16,
  message : String,
}

impl Default for Error
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      code : "UNKNOWN".to_string(),
      status : 500,
      message : "Unknown error has happened.".to_string(),
    }
  }
}

impl Error
{
  pub fn new( code : String ) -> Self
  {
    let mut err = Self::default();
    err.code = code;
    err
  }

  pub fn status( &mut self, s : u16 ) -> &mut Self
  {
    self.status = s;
    self
  }

  pub fn message( &mut self, m : String ) -> &mut Self
  {
    self.message = m;
    self
  }
}

#[ derive( Debug, Default ) ]
pub struct Server( Option< SocketAddr > );

impl Server
{
  pub fn bind( &mut self, ip : IpAddr, port : u16 ) { self.0 = Some( SocketAddr::new( ip, port ) ) }
}

#[ cfg( test ) ]
mod server_spec
{
  use super::*;

  mod bind
  {
    use std::net::Ipv4Addr;

    use super::*;

    #[ test ]
    fn sets_provided_address_to_server()
    {
      let mut server = Server::default();

      server.bind( IpAddr::V4( Ipv4Addr::new( 127, 0, 0, 1 ) ), 8080 );
      assert_eq!( format!( "{}", server.0.unwrap() ), "127.0.0.1:8080" );

      server.bind( "::1".parse().unwrap(), 9911 );
      assert_eq!( format!( "{}", server.0.unwrap() ), "[::1]:9911" );
    }
  }
}
