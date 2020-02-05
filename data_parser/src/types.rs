/// Mortgage owned by a player
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mortgage {
    /// amount of mortgage
    pub principal: u64,
    /// interest rate
    pub interest: f32,
    /// term by which mortgage is due
    pub term: u16,
}

/// Account of a player
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Account {
    /// how much he currently has
    pub credits: u64,
    /// his credit score
    pub score: u32,
    /// his current mortgage
    pub mortgage: Mortgage,
}
/// A date
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Date {
    /// the year
    pub year: u16,
    /// the month
    pub month: u8,
    /// the day
    pub day: u8,
}

/// A fleet
#[derive(Debug, PartialEq)]
pub struct Fleet<'a> {
    /// kind of the fleet
    pub kind: &'a str,
    /// count of ships in the fleet
    pub count: u16,
}

/// Tribute given by a planet
#[derive(Debug, PartialEq)]
pub struct Tribute<'a> {
    /// amount given for tribute
    pub value: u32,
    /// menace threshold at which planet will reply for tribute
    pub threshold: u32,
    /// fleet that will protect planet
    pub fleet: Fleet<'a>,
}

/// Start point for the player
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Planet<'a> {
    /// name of the planet
    pub name: &'a str,
    /// description of the planet, each &str is a line
    pub description: Vec<&'a str>,
    /// description of the spaceport, each &str is a line
    pub spaceport: Vec<&'a str>,
    /// shipyard, each &str is a set of ships sold
    pub shipyard: Vec<&'a str>,
    /// outfitter, each &str is a set of outfits sold
    pub outfitter: Vec<&'a str>,
    /// factor for bribe (?)
    pub bribe: f32,
    /// security of the planet (?)
    pub security: f32,
    /// tribute for this planet
    pub tribute: Tribute<'a>,
}

/// list of top level objects that can be parsed
#[derive(Debug)]
pub enum Object<'a> {
    /// player start
    Start(Start<'a>),
    /// a planet
    Planet(Planet<'a>),
}
