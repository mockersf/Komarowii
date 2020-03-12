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
    pub year: i32,
    /// the month
    pub month: u32,
    /// the day
    pub day: u32,
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
#[derive(Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
pub struct Planet<'a> {
    /// name of the planet
    pub name: &'a str,
    /// attributes of the planet
    #[builder(default)]
    pub attributes: Vec<&'a str>,
    /// landscape to display for the planet
    #[builder(default)]
    pub landscape: Option<&'a str>,
    /// government of the planet, if different from the parent system
    #[builder(default)]
    pub government: Option<&'a str>,
    /// music to play on landing
    #[builder(default)]
    pub music: Option<&'a str>,
    /// description of the planet, each &str is a line
    pub description: Vec<&'a str>,
    /// description of the spaceport, each &str is a line
    #[builder(default)]
    pub spaceport: Vec<&'a str>,
    /// shipyard, each &str is a set of ships sold
    #[builder(default)]
    pub shipyard: Vec<&'a str>,
    /// outfitter, each &str is a set of outfits sold
    #[builder(default)]
    pub outfitter: Vec<&'a str>,
    /// factor for bribe (?)
    #[builder(default)]
    pub bribe: Option<f32>,
    /// security of the planet (?)
    #[builder(default)]
    pub security: Option<f32>,
    /// tribute for this planet
    #[builder(default)]
    pub tribute: Option<Tribute<'a>>,
    /// required reputation with planet faction to land
    #[builder(default)]
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

/// weapon of a ship (?)
#[derive(Debug, PartialEq, Clone, Copy, Builder)]
pub struct ShipWeapon {
    /// it's blast radius
    pub blast_radius: u32,
    /// it's shield damage
    pub shield_damage: u32,
    /// it's hull damage
    pub hull_damage: u32,
    /// it's hit force
    pub hit_force: u32,
}

/// Attributes of a ship
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct ShipAttributes<'a> {
    /// licences needed to pilot this ship
    #[builder(default)]
    pub licenses: Vec<&'a str>,
    /// it's category
    pub category: &'a str,
    /// it's cost
    pub cost: u32,
    /// it's shield
    #[builder(default)]
    pub shields: u32,
    /// it's hull strength
    pub hull: u32,
    /// is it an automaton
    #[builder(default)]
    pub automaton: bool,
    /// it's required crew count
    #[builder(default)]
    pub required_crew: u32,
    /// it's bunk count
    #[builder(default)]
    pub bunks: u32,
    /// it's mass
    pub mass: u32,
    /// it's drag
    pub drag: f32,
    /// it's heat dissipation
    pub heat_dissipation: f32,
    /// it's fuel capacity
    #[builder(default)]
    pub fuel_capacity: u32,
    /// it's cargo space
    #[builder(default)]
    pub cargo_space: u32,
    /// it's outfit space
    pub outfit_space: u32,
    /// it's weapon capacity
    #[builder(default)]
    pub weapon_capacity: u32,
    /// it's engine capacity
    pub engine_capacity: u32,
    /// it's weapon (?)
    pub weapon: ShipWeapon,
}

/// a sprite
#[derive(Debug, PartialEq, Clone)]
pub enum Sprite<'a> {
    /// Complex sprite with multiple frames
    Sprite {
        /// name of the sprite
        name: &'a str,
        /// (?)
        frame_time: Option<u32>,
        /// (?)
        delay: Option<u32>,
        /// (?)
        random_start_frame: bool,
        /// (?)
        no_repeat: bool,
        /// (?)
        frame_rate: Option<f32>,
    },
    /// Simple sprite
    Simple(&'a str),
}

/// A ship
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Ship<'a> {
    /// name of the ship
    pub name: &'a str,
    /// subclass of the ship
    pub subclass: Option<&'a str>,
    /// plural form of the name
    #[builder(setter(into), default)]
    pub plural: Option<&'a str>,
    /// sprite of the ship
    pub sprite: Sprite<'a>,
    /// thumbnail of the ship
    pub thumbnail: &'a str,
    /// attributes of the ship
    pub attributes: ShipAttributes<'a>,
    /// outfits of the ship
    pub outfits: Vec<(&'a str, u32)>,
    /// engine locations and (?)
    pub engine: Vec<(f32, f32, Option<f32>)>,
    /// gun mount locations and what they hold
    #[builder(default)]
    pub gun: Vec<(f32, f32, Option<&'a str>)>,
    /// turret mount locations and what they hold
    #[builder(default)]
    pub turret: Vec<(f32, f32, Option<&'a str>)>,
    /// fighter mount locations and wherethey are
    #[builder(default)]
    pub fighter: Vec<(f32, f32, Option<&'a str>)>,
    /// drone mount locations and where they are
    #[builder(default)]
    pub drone: Vec<(f32, f32, Option<&'a str>)>,
    /// leaks (?)
    #[builder(default)]
    pub leak: Vec<(&'a str, u32, u32)>,
    /// explosion on death and tiling (?)
    pub explode: Vec<(&'a str, u32)>,
    /// final explosion
    #[builder(setter(into), default)]
    pub final_explode: Option<&'a str>,
    /// description
    pub description: Vec<&'a str>,
}

/// An outfit
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Outfit<'a> {
    /// name of the outfit
    pub name: &'a str,
    /// plural form of the name
    #[builder(setter(into), default)]
    pub plural: Option<&'a str>,
    /// category of the ouftit
    #[builder(setter(into), default)]
    pub category: Option<&'a str>,
    /// cost of the outfit
    #[builder(default)]
    pub cost: u32,
    /// is the outfit unplunderable
    #[builder(default)]
    pub unplunderable: u32,
    /// thumbnail of the outfit
    #[builder(setter(into), default)]
    pub thumbnail: Option<&'a str>,
    /// mass of the outfit
    #[builder(default)]
    pub mass: f32,
    /// outfit space used by the outfit
    #[builder(setter(into), default)]
    pub outfit_space: f32,
    /// cargo space used by the outfit
    #[builder(setter(into), default)]
    pub cargo_space: Option<f32>,
    /// cooling of the outfit
    #[builder(setter(into), default)]
    pub cooling: Option<f32>,
    /// cooling inefficiency of the outfit
    #[builder(setter(into), default)]
    pub cooling_inefficiency: Option<f32>,
    /// heat dissipation of the outfit
    #[builder(setter(into), default)]
    pub heat_dissipation: Option<f32>,
    /// shield generation of the outfit
    #[builder(setter(into), default)]
    pub shield_generation: Option<f32>,
    /// shield energy of the outfit
    #[builder(setter(into), default)]
    pub shield_energy: Option<f32>,
    /// energy consumption of the outfit
    #[builder(setter(into), default)]
    pub energy_consumption: Option<f32>,
    /// heat generation of the outfit
    #[builder(setter(into), default)]
    pub heat_generation: Option<f32>,
    /// radar jamming of the outfit
    #[builder(setter(into), default)]
    pub radar_jamming: Option<f32>,
    /// ramscoop of the outfit
    #[builder(setter(into), default)]
    pub ramscoop: Option<f32>,
    /// jump speed of the outfit
    #[builder(setter(into), default)]
    pub jump_speed: Option<f32>,
    /// jump fuel of the outfit
    #[builder(setter(into), default)]
    pub jump_fuel: Option<f32>,
    /// hyperdrive of the outfit
    #[builder(setter(into), default)]
    pub hyperdrive: Option<f32>,
    /// scram drive of the outfit
    #[builder(setter(into), default)]
    pub scram_drive: Option<f32>,
    /// jump drive of the outfit
    #[builder(setter(into), default)]
    pub jump_drive: Option<f32>,
    /// cargo scan power of the outfit
    #[builder(setter(into), default)]
    pub cargo_scan_power: Option<f32>,
    /// cargo scan speed of the outfit
    #[builder(setter(into), default)]
    pub cargo_scan_speed: Option<f32>,
    /// outfit scan power of the outfit
    #[builder(setter(into), default)]
    pub outfit_scan_power: Option<f32>,
    /// outfit scan speed of the outfit
    #[builder(setter(into), default)]
    pub outfit_scan_speed: Option<f32>,
    /// asteroid scan power of the outfit
    #[builder(setter(into), default)]
    pub asteroid_scan_power: Option<f32>,
    /// tactical scan power of the outfit
    #[builder(setter(into), default)]
    pub tactical_scan_power: Option<f32>,
    /// atmosphere scan of the outfit
    #[builder(setter(into), default)]
    pub atmosphere_scan: Option<f32>,
    /// cloak of the outfit
    #[builder(setter(into), default)]
    pub cloak: Option<f32>,
    /// cloaking energy of the outfit
    #[builder(setter(into), default)]
    pub cloaking_energy: Option<f32>,
    /// cloaking fuel of the outfit
    #[builder(setter(into), default)]
    pub cloaking_fuel: Option<f32>,
    /// bunks added by the outfit
    #[builder(setter(into), default)]
    pub bunks: Option<f32>,
    /// required crew for the outfit
    #[builder(setter(into), default)]
    pub required_crew: Option<f32>,
    /// fuel capacity it will add to the ship
    #[builder(setter(into), default)]
    pub fuel_capacity: Option<f32>,
    /// scan interference
    #[builder(setter(into), default)]
    pub scan_interference: Option<f32>,
    /// capture attack
    #[builder(setter(into), default)]
    pub capture_attack: Option<f32>,
    /// capture defense
    #[builder(setter(into), default)]
    pub capture_defense: Option<f32>,
    /// illegal
    #[builder(setter(into), default)]
    pub illegal: Option<f32>,
    /// map
    #[builder(setter(into), default)]
    pub map: Option<f32>,
    /// weapon capacity
    #[builder(setter(into), default)]
    pub weapon_capacity: Option<f32>,
    /// engine capacity
    #[builder(setter(into), default)]
    pub engine_capacity: Option<f32>,
    /// afterburner thrust
    #[builder(setter(into), default)]
    pub afterburner_thrust: Option<f32>,
    /// afterburner fuel
    #[builder(setter(into), default)]
    pub afterburner_fuel: Option<f32>,
    /// afterburner energy
    #[builder(setter(into), default)]
    pub afterburner_energy: Option<f32>,
    /// afterburner heat
    #[builder(setter(into), default)]
    pub afterburner_heat: Option<f32>,
    /// afterburner effect
    #[builder(setter(into), default)]
    pub afterburner_effect: Option<&'a str>,
    /// turn power
    #[builder(setter(into), default)]
    pub turn: Option<f32>,
    /// turning energy consumption
    #[builder(setter(into), default)]
    pub turning_energy: Option<f32>,
    /// thurning heat production
    #[builder(setter(into), default)]
    pub turning_heat: Option<f32>,
    /// thrust power
    #[builder(setter(into), default)]
    pub thrust: Option<f32>,
    /// thrusting energy consumption
    #[builder(setter(into), default)]
    pub thrusting_energy: Option<f32>,
    /// thrusting heat generation
    #[builder(setter(into), default)]
    pub thrusting_heat: Option<f32>,
    /// reverse thrust power
    #[builder(setter(into), default)]
    pub reverse_thrust: Option<f32>,
    /// reverse thrusting energy consumption
    #[builder(setter(into), default)]
    pub reverse_thrusting_energy: Option<f32>,
    /// reverse thrusting heat generation
    #[builder(setter(into), default)]
    pub reverse_thrusting_heat: Option<f32>,
    /// energy capacity
    #[builder(setter(into), default)]
    pub energy_capacity: Option<u32>,
    /// solar collection
    #[builder(setter(into), default)]
    pub solar_collection: Option<f32>,
    /// energy generation
    #[builder(setter(into), default)]
    pub energy_generation: Option<f32>,
    /// flare sprite
    #[builder(setter(into), default)]
    pub flare_sprite: Option<Sprite<'a>>,
    /// flare sound
    #[builder(setter(into), default)]
    pub flare_sound: Option<&'a str>,
    /// gun ports provided / occupied
    #[builder(setter(into), default)]
    pub gun_ports: Option<f32>,
    /// turret mounts provided / occupied
    #[builder(setter(into), default)]
    pub turret_mounts: Option<i32>,
    /// weapon details
    #[builder(setter(into), default)]
    pub weapon: Option<Weapon<'a>>,
    /// ammo it can contains
    #[builder(setter(into), default)]
    pub ammo: Option<&'a str>,
    /// gatling round capacity
    #[builder(setter(into), default)]
    pub gatling_round_capacity: Option<i32>,
    /// javelin capacity
    #[builder(setter(into), default)]
    pub javelin_capacity: Option<i32>,
    /// meteor capacity
    #[builder(setter(into), default)]
    pub meteor_capacity: Option<i32>,
    /// rocket capacity
    #[builder(setter(into), default)]
    pub rocket_capacity: Option<i32>,
    /// sidewinder capacity
    #[builder(setter(into), default)]
    pub sidewinder_capacity: Option<i32>,
    /// torpedo capacity
    #[builder(setter(into), default)]
    pub torpedo_capacity: Option<i32>,
    /// typhoon capacity
    #[builder(setter(into), default)]
    pub typhoon_capacity: Option<i32>,
    /// description
    #[builder(default)]
    pub description: Vec<&'a str>,
}

/// A weapon
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Weapon<'a> {
    /// sprite of the weapon
    #[builder(setter(into), default)]
    pub sprite: Option<Sprite<'a>>,
    /// hardpoint (?) sprite of the weapon
    #[builder(setter(into), default)]
    pub hardpoint_sprite: Option<Sprite<'a>>,
    /// hardpoint offset
    #[builder(setter(into), default)]
    hardpoint_offset: Option<Vec<f32>>,
    /// sound of the weapon
    #[builder(setter(into), default)]
    sound: Option<&'a str>,
    /// ammo used by the weapon
    #[builder(setter(into), default)]
    ammo: Option<&'a str>,
    /// icon
    #[builder(setter(into), default)]
    pub icon: Option<&'a str>,
    /// hit effect of the weapon
    #[builder(setter(into), default)]
    hit_effect: Option<(&'a str, Option<i32>)>,
    /// fire effect of the weapon
    #[builder(setter(into), default)]
    fire_effect: Option<(&'a str, Option<i32>)>,
    /// die effect of the weapon
    #[builder(setter(into), default)]
    die_effect: Option<(&'a str, Option<i32>)>,
    /// submunition of the weapon
    #[builder(setter(into), default)]
    submunition: Option<(&'a str, Option<i32>)>,
    /// anti-missile
    #[builder(setter(into), default)]
    anti_missile: Option<f32>,
    /// inaccuracy
    #[builder(setter(into), default)]
    inaccuracy: Option<f32>,
    /// turret turn
    #[builder(setter(into), default)]
    turret_turn: Option<f32>,
    /// velocity
    #[builder(setter(into), default)]
    velocity: Option<f32>,
    /// lifetime
    #[builder(setter(into), default)]
    lifetime: Option<f32>,
    /// random velocity
    #[builder(setter(into), default)]
    random_velocity: Option<f32>,
    /// random lifetime
    #[builder(setter(into), default)]
    random_lifetime: Option<f32>,
    /// firing energy
    #[builder(setter(into), default)]
    reload: Option<f32>,
    /// firing energy
    #[builder(setter(into), default)]
    firing_energy: Option<f32>,
    /// firing force
    #[builder(setter(into), default)]
    firing_force: Option<f32>,
    /// firing fuel
    #[builder(setter(into), default)]
    firing_fuel: Option<f32>,
    /// firing heat
    #[builder(setter(into), default)]
    firing_heat: Option<f32>,
    /// hit force
    #[builder(setter(into), default)]
    hit_force: Option<f32>,
    /// shield damage
    #[builder(setter(into), default)]
    shield_damage: Option<f32>,
    /// hull damage
    #[builder(setter(into), default)]
    hull_damage: Option<f32>,
    /// heat damage
    #[builder(setter(into), default)]
    heat_damage: Option<f32>,
    /// acceleration
    #[builder(setter(into), default)]
    pub acceleration: Option<f32>,
    /// drag
    #[builder(setter(into), default)]
    pub drag: Option<f32>,
    /// turn
    #[builder(setter(into), default)]
    pub turn: Option<f32>,
    /// homing
    #[builder(setter(into), default)]
    pub homing: Option<f32>,
    /// infrared tracking
    #[builder(setter(into), default)]
    pub infrared_tracking: Option<f32>,
    /// radar tracking
    #[builder(setter(into), default)]
    pub radar_tracking: Option<f32>,
    /// optical tracking
    #[builder(setter(into), default)]
    pub optical_tracking: Option<f32>,
    /// trigger radius
    #[builder(setter(into), default)]
    pub trigger_radius: Option<f32>,
    /// blast radius
    #[builder(setter(into), default)]
    pub blast_radius: Option<f32>,
    /// missile strength
    #[builder(setter(into), default)]
    pub missile_strength: Option<f32>,
    /// stream
    #[builder(setter(into), default)]
    pub stream: bool,
    /// clustre
    #[builder(setter(into), default)]
    pub cluster: bool,
    /// burst count
    #[builder(setter(into), default)]
    pub burst_count: Option<u32>,
    /// burst reload
    #[builder(setter(into), default)]
    pub burst_reload: Option<u32>,
}

/// An effect
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct Effect<'a> {
    /// name of the outfit
    pub name: &'a str,
    /// sprite of the effect
    pub sprite: Sprite<'a>,
    /// sound of the effect
    #[builder(setter(into), default)]
    sound: Option<&'a str>,
    /// lifetime of the effect
    #[builder(setter(into), default)]
    lifetime: Option<f32>,
    /// angle of the effect
    #[builder(setter(into), default)]
    random_angle: Option<f32>,
    /// spin of the effect
    #[builder(setter(into), default)]
    random_spin: Option<f32>,
    /// frame rate of the effect
    #[builder(setter(into), default)]
    random_frame_rate: Option<f32>,
    /// velocity of the effect
    #[builder(setter(into), default)]
    random_velocity: Option<f32>,
    /// scale of the effect
    #[builder(setter(into), default)]
    velocity_scale: Option<f32>,
}

/// list of top level objects that can be parsed
#[allow(clippy::large_enum_variant)]
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
    /// a ship
    Ship(Ship<'a>),
    /// an outfit
    Outfit(Outfit<'a>),
    /// an effect
    Effect(Effect<'a>),
}
