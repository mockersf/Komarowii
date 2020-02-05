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
pub struct Fleet {
    pub kind: String,
    pub count: u16,
}

#[derive(Debug, PartialEq)]
pub struct Tribute {
    pub value: u32,
    pub threshold: u32,
    pub fleet: Fleet,
}

#[derive(Debug)]
pub struct Start {
    pub date: Date,
    pub system: String,
    pub planet: String,
    pub account: Account,
    pub set: String,
}

#[derive(Debug)]
pub struct Planet {
    pub name: String,
    pub description: String,
    pub spaceport: String,
    pub shipyard: Vec<String>,
    pub outfitter: Vec<String>,
    pub bribe: f32,
    pub security: f32,
    pub tribute: Tribute,
}

#[derive(Debug)]
pub enum Object {
    Start(Start),
    Planet(Planet),
}
