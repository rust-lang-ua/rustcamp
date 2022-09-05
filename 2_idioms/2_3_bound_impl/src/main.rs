use std::
{
  borrow::{ Borrow, BorrowMut },
  num::NonZeroU64,
};

fn main()
{
  println!( "Refactor me!" );
}

/// A projected state built from a series of events.
pub trait Aggregate: Default
{
  /// A static string representing the type of the aggregate.
  ///
  /// Note: This should effectively be a constant value, and should never change.
  fn aggregate_type() -> &'static str;

  /// Consumes the event, applying its effects to the aggregate.
  fn apply< E >( &mut self, event : E )
  where
    E : AggregateEvent< Self >,
  {
    event.apply_to( self );
  }
}

/// An identifier for an aggregate.
pub trait AggregateId< A >
where
  A : Aggregate,
{
  /// Gets the stringified aggregate identifier.
  fn as_str( &self ) -> &str;
}

/// A thing that happened.
pub trait Event
{
  /// A static description of the event.
  fn event_type( &self ) -> &'static str;
}

/// An event that can be applied to an aggregate.
pub trait AggregateEvent< A : Aggregate > : Event
{
  /// Consumes the event, applying its effects to the aggregate.
  fn apply_to( self, aggregate : &mut A );
}

/// Represents an event sequence number, starting at 1
#[ derive( Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord ) ]
pub struct EventNumber( NonZeroU64 );

impl EventNumber
{
  /// The minimum [EventNumber].
  #[ allow( unsafe_code ) ]
  pub const MIN_VALUE : EventNumber =
    // One is absolutely non-zero, and this is required for this to be usable in a `const` context.
    EventNumber( unsafe { NonZeroU64::new_unchecked( 1 ) } );

  /// Increments the event number to the next value.
  #[ inline ]
  pub fn incr( &mut self ) { self.0 = NonZeroU64::new( self.0.get() + 1 ).unwrap(); }
}

/// An aggregate version.
#[ derive( Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord ) ]
pub enum Version
{
  /// The version of an aggregate that has not had any events applied to it.
  Initial,
  /// The version of the last event applied to the aggregate.
  Number( EventNumber ),
}

impl Default for Version
{
  #[ inline ]
  fn default() -> Self { Version::Initial }
}

impl Version
{
  /// Creates a new `Version` from a number.
  ///
  /// The number `0` gets interpreted as being `Version::Initial`, while any other number is interpreted as the
  /// latest event number applied.
  #[ inline ]
  pub fn new( number : u64 ) -> Self
  {
    NonZeroU64::new( number )
    .map( EventNumber )
    .map( Version::Number )
    .unwrap_or( Version::Initial )
  }

  /// Increments the version number to the next in sequence.
  #[ inline ]
  pub fn incr( &mut self )
  {
    match *self
    {
      Version::Initial = > *self = Version::Number( EventNumber::MIN_VALUE ),
      Version::Number( ref mut en ) = > en.incr(),
    }
  }
}

/// An aggregate that has been loaded from a source, which keeps track of the version of its last snapshot and the current version of the aggregate.
#[ derive( Clone, Copy, Debug, Default, Hash, PartialEq, Eq ) ]
pub struct HydratedAggregate< A >
where
  A : Aggregate,
{
  version : Version,
  snapshot_version : Option< Version >,
  state : A,
}

impl< A > HydratedAggregate< A >
where
  A : Aggregate,
{
  /// The current version of the aggregate.
  pub fn version( &self ) -> Version { self.version }

  /// The version of the snapshot from which the aggregate was loaded.
  pub fn snapshot_version( &self ) -> Option< Version > { self.snapshot_version }

  /// Updates the snapshot version. Generally used to indicate that a snapshot was taken.
  pub fn set_snapshot_version( &mut self, new_snapshot_version : Version ) { self.snapshot_version = Some( new_snapshot_version ); }

  /// The actual aggregate.
  pub fn state( &self ) -> &A { &self.state }

  /// Applies a sequence of events to the internal aggregate.
  pub fn apply_events< E : AggregateEvent< A >, I : IntoIterator< Item = E > >( &mut self, events : I )
  {
    for event in events
    {
      self.apply( event );
    }
  }

  /// Applies a single event to the aggregate, keeping track of the new aggregate version.
  pub fn apply< E : AggregateEvent< A > >( &mut self, event : E )
  {
    self.state.apply( event );
    self.version.incr();
  }
}

impl< A > AsRef< A > for HydratedAggregate< A >
where
  A : Aggregate,
{
  fn as_ref( &self ) -> &A { &self.state }
}

impl< A > Borrow< A > for HydratedAggregate< A >
where
  A : Aggregate,
{
  fn borrow( &self ) -> &A { &self.state }
}

/// An identified, specific instance of a hydrated aggregate.
#[ derive( Clone, Copy, Debug, Default, Hash, PartialEq, Eq ) ]
pub struct Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  id : I,
  aggregate : HydratedAggregate< A >,
}

impl< I, A > Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  /// Creates a new entity from an identifier and an associated hydrated aggregate.
  pub fn new( id : I, aggregate : HydratedAggregate< A > ) -> Self { Entity { id, aggregate } }

  /// The entity's identifier.
  pub fn id( &self ) -> &I { &self.id }

  /// An immutable reference to the underlying aggregate.
  pub fn aggregate( &self ) -> &HydratedAggregate< A > { &self.aggregate }

  /// A mutable reference to the underlying aggregate.
  pub fn aggregate_mut( &mut self ) -> &mut HydratedAggregate< A > { &mut self.aggregate }
}

impl< I, A > From< Entity< I, A > > for HydratedAggregate< A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn from( entity : Entity< I, A > ) -> Self { entity.aggregate }
}

impl< I, A > AsRef< HydratedAggregate< A > > for Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn as_ref( &self ) -> &HydratedAggregate< A > { &self.aggregate }
}

impl< I, A > AsMut< HydratedAggregate< A > > for Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn as_mut( &mut self ) -> &mut HydratedAggregate< A > { &mut self.aggregate }
}

impl< I, A > Borrow< HydratedAggregate< A > > for Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn borrow( &self ) -> &HydratedAggregate< A > { &self.aggregate }
}

impl< I, A > Borrow< A > for Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn borrow( &self ) -> &A { self.aggregate.borrow() }
}

impl< I, A > BorrowMut< HydratedAggregate< A > > for Entity< I, A >
where
  A : Aggregate,
  I : AggregateId< A >,
{
  fn borrow_mut( &mut self ) -> &mut HydratedAggregate< A > { &mut self.aggregate }
}
