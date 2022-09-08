use std::borrow::Cow;


fn conf_path() -> Result< Cow< 'static, str >, String >
{
  match ( 
    std::env::args().collect::< Vec< String > >().as_slice(),
    std::env::var( "APP_CONF" )
  )
  {
    ( [ _, cmd ], _ ) if cmd == "--conf" => Err( "Path is empty".into() ),
    ( [ _, cmd, path ], _ ) if cmd == "--conf" => Ok( format!( "{}", path ).into() ),
    ( _, Ok( path ) ) if !path.is_empty() => Ok ( path.into() ),
    _ => Ok( "/etc/app/app.conf".into() )
  }
}

fn main()
{
  println!( "{:?}", conf_path() );
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn default()
  {
    assert_eq!( "/etc/app/app.conf", conf_path().unwrap() );
  }
  
  #[ test ]
  fn app_conf_is_empty()
  {
    std::env::set_var("APP_CONF", "");
    assert_eq!( "/etc/app/app.conf", conf_path().unwrap() );
  }

  #[ test ]
  fn app_conf_is_not_empty()
  {
    std::env::set_var("APP_CONF", "/home/user/app.conf");
    assert_eq!( "/home/user/app.conf", conf_path().unwrap() );
  }
}
