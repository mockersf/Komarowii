#[derive(Debug, PartialEq)]
pub struct Mortgage {
    pub principal: u64,
    pub interest: f32,
    pub term: u16,
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub credits: u64,
    pub score: u32,
    pub mortgage: Mortgage,
}

#[derive(Debug, PartialEq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, PartialEq)]
pub struct Fleet<'a> {
    pub kind: &'a str,
    pub count: u16,
}

#[derive(Debug, PartialEq)]
pub struct Tribute<'a> {
    pub value: u32,
    pub threshold: u32,
    pub fleet: Fleet<'a>,
}

#[derive(Debug)]
pub struct Start<'a> {
    pub date: Date,
    pub system: &'a str,
    pub planet: &'a str,
    pub account: Account,
    pub set: &'a str,
}

#[derive(Debug)]
pub struct Planet<'a> {
    pub name: &'a str,
    pub description: Vec<&'a str>,
    pub spaceport: Vec<&'a str>,
    pub shipyard: Vec<&'a str>,
    pub outfitter: Vec<&'a str>,
    pub bribe: f32,
    pub security: f32,
    pub tribute: Tribute<'a>,
}

#[derive(Debug)]
pub enum Object<'a> {
    Start(Start<'a>),
    Planet(Planet<'a>),
}
