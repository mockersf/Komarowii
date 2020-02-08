use derive_builder::Builder;

/// Mortgage owned by a player
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Mortgage {
    /// amount of mortgage
    pub principal: u64,
    /// interest rate
    pub interest: f32,
    /// term by which mortgage is due
    pub term: u16,
}

/// Account of a player
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Account {
    /// how much he currently has
    pub credits: u64,
    /// his credit score
    pub score: u32,
    /// his current mortgage
    pub mortgage: Mortgage,
}

/// A date
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
#[builder(setter(into))]
pub struct Date {
    /// the year
    pub year: u16,
    /// the month
    pub month: u8,
    /// the day
    pub day: u8,
}

/// A fleet
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Fleet<'a> {
    /// kind of the fleet
    pub kind: &'a str,
    /// count of ships in the fleet
    pub count: u16,
}

/// Tribute given by a planet
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Tribute<'a> {
    /// amount given for tribute
    pub value: u32,
    /// menace threshold at which planet will reply for tribute
    pub threshold: u32,
    /// fleet that will protect planet
    pub fleet: Fleet<'a>,
}

/// Start point for the player
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Start<'a> {
    /// start date
    pub date: Date,
    /// start system
    pub system: &'a str,
    /// start planet
    pub planet: &'a str,
    /// start account
    pub account: Account,
    /// start set
    pub set: &'a str,
}

/// A planet
#[derive(Debug, PartialEq, Clone, Builder, Default)]
#[builder(setter(into), default)]
pub struct Planet<'a> {
    /// name of the planet
    pub name: &'a str,
    /// attributes of the planet
    pub attributes: Vec<&'a str>,
    /// landscape to display for the planet
    pub landscape: Option<&'a str>,
    /// government of the planet, if different from the parent system
    pub government: Option<&'a str>,
    /// music to play on landing
    pub music: Option<&'a str>,
    /// description of the planet, each &str is a line
    pub description: Vec<&'a str>,
    /// description of the spaceport, each &str is a line
    pub spaceport: Vec<&'a str>,
    /// shipyard, each &str is a set of ships sold
    pub shipyard: Vec<&'a str>,
    /// outfitter, each &str is a set of outfits sold
    pub outfitter: Vec<&'a str>,
    /// factor for bribe (?)
    pub bribe: Option<f32>,
    /// security of the planet (?)
    pub security: Option<f32>,
    /// tribute for this planet
    pub tribute: Option<Tribute<'a>>,
    /// required reputation with planet faction to land
    pub required_reputation: Option<f32>,
}

/// A position
#[derive(Debug, Clone, Copy, PartialEq, Builder)]
#[builder(setter(into))]
pub struct Position {
    /// x pos
    pub x: f64,
    /// y pos
    pub y: f64,
}

/// A galaxy
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Galaxy<'a> {
    /// it's position
    pub pos: Position,
    /// it's name
    pub name: &'a str,
    /// it's sprite
    pub sprite: Option<&'a str>,
}

/// An asteroid
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Asteroids<'a> {
    /// it's name
    pub name: &'a str,
    /// (?)
    pub first_value: u32,
    /// (?)
    pub second_value: f32,
}

/// A minable
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Minables<'a> {
    /// it's name
    pub name: &'a str,
    /// (?)
    pub first_value: u32,
    /// (?)
    pub second_value: f32,
}

/// A trade good with a price
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Trade<'a> {
    /// it's name
    pub name: &'a str,
    /// price
    pub price: u32,
}

/// An object in a system
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct SystemObject<'a> {
    /// it's name
    pub name: Option<&'a str>,
    /// it's sprite
    pub sprite: Option<&'a str>,
    /// distance
    pub distance: Option<f32>,
    /// period
    pub period: f32,
    /// offset
    pub offset: Option<f32>,
    /// related objects
    pub objects: Vec<SystemObject<'a>>,
}

/// A system
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct System<'a> {
    /// it's name
    pub name: &'a str,
    /// it's position
    pub pos: Position,
    /// it's government
    pub government: &'a str,
    /// habitable (?)
    pub habitable: f32,
    /// belt (?)
    pub belt: Option<u32>,
    /// haze type
    pub haze: Option<&'a str>,
    /// links to other systems
    pub links: Vec<&'a str>,
    /// asteroids present in the system
    pub asteroids: Vec<Asteroids<'a>>,
    /// minables present in the system
    pub minables: Vec<Minables<'a>>,
    /// trade goods that are sold here
    pub trades: Vec<Trade<'a>>,
    /// fleets present in the system
    pub fleets: Vec<Fleet<'a>>,
    /// objects present in the system
    pub objects: Vec<SystemObject<'a>>,
}

/// list of top level objects that can be parsed
#[derive(Debug)]
pub enum Object<'a> {
    /// player start
    Start(Start<'a>),
    /// a planet
    Planet(Planet<'a>),
    /// a galaxy
    Galaxy(Galaxy<'a>),
    /// a system
    System(System<'a>),
}
