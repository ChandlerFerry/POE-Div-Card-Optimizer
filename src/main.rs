use itertools::Itertools;
use std::collections::HashSet;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

const NUM_THREADS: usize = 18;

#[derive(Debug, Deserialize, Clone)]
struct Card {
    name: String,
    drop: Option<DropInfo>,
    price: f64,
    weight: Option<f64>,
    stack: f64,
}

#[derive(Debug, Deserialize, Clone)]
struct DropInfo {
    #[serde(default = "default_max_level")]
    max_level: Option<u32>,
    areas: Vec<String>,
}

fn default_max_level() -> Option<u32> {
    Some(100)
}

const ALL_MAPS: [&str; 116] = [
    "ForbiddenWoods",
    "ColdRiver_",
    "FrozenCabins",
    "Iceberg",
    "Scriptorium",
    "Museum",
    "Academy",
    "DrySea",
    "Dunes",
    "DesertSpring",
    "MineralPools",
    "CrystalOre",
    "TortureChamber",
    "Grotto",
    "Wharf",
    "FloodedMine",
    "Waterways",
    "Vault",
    "Peninsula",
    "Geode",
    "AshenWood",
    "SpiderForest",
    "Lair",
    "Thicket",
    "TropicalIsland",
    "Stagnation",
    "JungleValley",
    "BrambleValley",
    "SunkenCity",
    "VaalPyramid",
    "Alleyways",
    "Arcade",
    "Port",
    "MoonTemple",
    "Factory",
    "Excavation",
    "Orchard",
    "Plaza",
    "Conservatory",
    "Temple",
    "Cemetery",
    "GraveTrough",
    "Graveyard",
    "LavaChamber",
    "BoneCrypt",
    "Bog",
    "Marshes",
    "Basilica",
    "Residence",
    "Arsenal",
    "Promenade",
    "OvergrownShrine",
    "Courtyard",
    "Terrace",
    "Gardens",
    "Strand",
    "Shore",
    "LavaLake",
    "Estuary",
    "Volcano",
    "Foundry",
    "UndergroundSea",
    "CursedCrypt",
    "Necropolis",
    "AcidLakes",
    "ArachnidNest",
    "ArachnidTomb",
    "AridLake",
    "Armoury",
    "Atoll",
    "Barrows",
    "Beach",
    "Belfry",
    "BurialChambers", // Died on 90 choose 3 (6-3) (9-3)
    "Cage",
    "Canyon",
    "CastleRuins",
    "Cells",
    "Channel",
    "CitySquare",
    "CoralRuins",
    "Courthouse",
    "CrimsonTemple",
    "CrimsonTownship",
    "DefiledCathedral",
    "Dungeon",
    "Fields",
    "ForkingRiver",
    "Springs",
    "HauntedMansion",
    "Leyline",
    "Lighthouse",
    "Lookout",
    "Malformation",
    "Mausoleum",
    "Maze",
    "Palace",
    "Park",
    "Pen",
    "Phantasmagoria",
    "Pier",
    "Pit",
    "Plateau",
    "PrimordialPool",
    "Reef",
    "Shipyard",
    "Shrine",
    "Siege",
    "Silo",
    "SulphurVents",
    "Tower",
    "ToxicSewer",
    "UndergroundRiver",
    "Villa",
    "WastePool",
    "Wasteland",
];

const CARD_PRICE_FLOOR_FILTER: f64 = 6.0;
const CARD_WEIGHT_FLOOR_FILTER: f64 = 0.0;
const GLOBAL_DROP_RATE: f64 = 7954753.0;
const AREA_LEVEL: u32 = 83;

const USE_FORCE_REMOVE_FILTER: bool = true;
const USE_FORCE_SHOW_FILTER: bool = true;
lazy_static! {
    static ref FORCE_REMOVE_V_FILTER: HashSet<String> = HashSet::new();
}

lazy_static! {
    static ref FORCE_REMOVE_FILTER: HashSet<String> = {
        let mut set = HashSet::new();
        set.extend(vec![
            "The Easy Stroll",
            "The Lunaris Priestess",
            "The Explorer",
            "The Mountain",
            "Boundless Realms",
            "Azure Rage",
            "Left to Fate",
            "Might is Right",
            "Scholar of the Seas",
            "Grave Knowledge",
            "The Wolverine",
            "Blind Venture",
            "Hunter's Resolve",
            "Alivia's Grace",
            "The Admirer",
            "The Surgeon",
            "The Wolf's Shadow",
            "Shard of Fate",
            "Jack in the Box",
            "Last Hope",
            "Mitts",
            "The Battle Born",
            "The Sun",
            "The Demoness",
            "The Sigil",
            "The Twins",
            "The Inoculated",
            "The Army of Blood",
            "The Visionary",
            "The Gladiator",
            "Gemcutter's Promise",
            "The Web",
            "The Sword King's Salute",
            "Boon of Justice",
            "The Penitent",
            "The Warden",
            "The Cache",
            "Lysah's Respite",
            "The Fathomless Depths",
            "The Harvester",
            "The Fox",
            "Volatile Power",
            "The Endurance",
            "The Wolf",
            "Time-Lost Relic",
            "The Rite of Elements",
            "Gift of the Gemling Queen",
            "The Standoff",
            "Prosperity",
            "Heterochromia",
            "The Insatiable",
            "The Incantation",
            "The Betrayal",
            "The Pack Leader",
            "The Oath",
            "Vile Power",
            "The Surveyor",
            "Thunderous Skies",
            "The Tower",
            "The Stormcaller",
            "The Opulent",
            "The Blazing Fire",
            "The Journalist",
            "The Jeweller's Boon",
            "The Survivalist",
            "Glimmer of Hope",
            "Destined to Crumble",
            "The Scholar",
            "Thirst for Knowledge",
            "Rain of Chaos",
            "Emperor's Luck",
            "Loyalty",
            "A Sea of Blue",
            "The Lover",
            "The King's Blade",
            "The Catalyst",
            "Lantador's Lost Love",
            "The Scarred Meadow",
            "Rats",
            "The Witch",
            "Three Voices",
        ].iter().map(|s| s.to_string()));
        set
    };
}

lazy_static! {
    static ref FORCE_SHOW_FILTER: HashSet<String> = {
        let mut set = HashSet::new();
        set.extend(vec![
            "Acclimatisation",
            "The Primordial",
            "Three Faces in the Dark",
            "The Coming Storm",
            "The Porcupine",
            "Coveted Possession",
            "The Chains that Bind",
            "The Union",
            "Lucky Connections",
            "The Innocent",
            "Vinia's Token",
            "The Wrath",
            "No Traces",
            "Cursed Words",
            "Lingering Remnants",
            "Bowyer's Dream",
            "Draped in Dreams",
            "Emperor of Purity",
            "Immortal Resolve",
            "Imperial Legacy",
            "The Celestial Justicar",
            "The Dapper Prodigy",
            "The Dark Mage",
            "The Porcupine",
            "The Warlord",
        ].iter().map(|s| s.to_string()));
        set
    };
}

const REAL_CARD_RATE: (f64, u32) = (47.0, 100);

const PINNED_DPI: f64 = 75231.43071987167;

fn is_card_in_area(card: &Card, areas: &[&str]) -> bool {
    areas.iter().any(|&area| {
        let formatted_area = format!("MapWorlds{}", area);
        if let Some(drop_info) = &card.drop {
            drop_info.max_level >= Some(AREA_LEVEL) && drop_info.areas.iter().any(|area_name| area_name.eq_ignore_ascii_case(&formatted_area))
        } else {
            false
        }
    })
}

fn calculate_card_ev(stack: f64, card_count: f64, price: f64, use_stack_scarab: bool) -> (f64, f64) {
    let mut ev = 0.0;
    let mut drops = card_count;
    if use_stack_scarab {
        ev = card_count * 0.2 * stack * price + card_count * 0.8 * price;
        drops = card_count * 0.2 * stack + card_count * 0.8;
    } else {
        ev = card_count * price;
    }
    (ev, drops)
}

fn get_calculated_cards(areas: &[&str], all_cards: &[Card]) -> (f64, f64) {
    let mut total_raw_ev = 0.0;
    let mut total_stack_scarab_ev = 0.0;

    let map_cards: Vec<&Card> = all_cards.iter().filter(|&card| is_card_in_area(card, areas)).collect();
    let map_total_weight: f64 = map_cards.iter().map(|&card| card.weight.unwrap_or(0.0)).sum();
    let card_weight_baseline = all_cards.iter().find(|&card| card.name == "The Union").unwrap().weight.unwrap_or(0.0);
    let current_total_weight = map_total_weight + GLOBAL_DROP_RATE;
    let drop_pool_items = 1.0 / (card_weight_baseline / current_total_weight) * REAL_CARD_RATE.1 as f64;
    let dpi_multiplier = PINNED_DPI / drop_pool_items;

    let filtered_cards: Vec<&Card> = map_cards.iter().filter(|&card| {
        (card.price >= CARD_PRICE_FLOOR_FILTER && 
        !FORCE_REMOVE_V_FILTER.contains(&card.name) && 
        (!FORCE_REMOVE_FILTER.contains(&card.name) || !USE_FORCE_REMOVE_FILTER) && 
        card.weight.unwrap_or(0.0) > CARD_WEIGHT_FLOOR_FILTER) || 
        (FORCE_SHOW_FILTER.contains(&card.name) && USE_FORCE_SHOW_FILTER)
    }).cloned().collect();

    filtered_cards.iter().for_each(|&card| {
        if let Some(drop_info) = &card.drop {
            let individual_drop_rate = (card.weight.unwrap_or(0.0) / current_total_weight) * drop_pool_items;
            let drops_per_map = individual_drop_rate * dpi_multiplier;
            let (raw_ev, _) = calculate_card_ev(card.stack, drops_per_map, card.price, false);
            total_raw_ev += raw_ev;
            let (ss_ev, _) = calculate_card_ev(card.stack, drops_per_map, card.price, true);
            total_stack_scarab_ev += ss_ev;
        }
    });

    (total_raw_ev, total_stack_scarab_ev)
}

fn binomial(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

fn main() {
    let all_cards_data = fs::read_to_string("cards.json").expect("Failed to read cards.json");
    let all_cards: Vec<Card> = serde_json::from_str(&all_cards_data).expect("Failed to parse cards.json");
    let optimal_stack_scarab_ev = Arc::new(Mutex::new(0.0));
    let mut handles = vec![];

    // Updated initial_areas to a vector of vectors
    let initial_areas: Vec<Vec<&str>> = vec![
        // Current Strategy: manual 4 + 5 + 3
        // Current Lockouts: 9 @ 9900 | 12 @ 10569

        // Manual
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells"],

        // 18 on 115? choose 3
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Graveyard", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Graveyard", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Graveyard", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Graveyard", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Graveyard", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Graveyard", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "Mausoleum", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "Mausoleum", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "Mausoleum", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Graveyard", "Arsenal", "Mausoleum", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Graveyard", "Arsenal", "Mausoleum", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Graveyard", "Arsenal", "Mausoleum", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Graveyard", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Graveyard", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Graveyard", "Arsenal", "OvergrownShrine", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Wharf", "GraveTrough", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Wharf", "GraveTrough", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Wharf", "GraveTrough", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Wharf", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Wharf", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Wharf", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "MoonTemple", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "MoonTemple", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "MoonTemple", "Cemetery", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Stagnation", "GraveTrough", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Stagnation", "GraveTrough", "Arsenal", "Shrine"],
        vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Stagnation", "GraveTrough", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Stagnation", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Stagnation", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Stagnation", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Wharf", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Wharf", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Wharf", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "MoonTemple", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "MoonTemple", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "MoonTemple", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "JungleValley", "Cemetery", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "GraveTrough", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "GraveTrough", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Stagnation", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Stagnation", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "GraveTrough", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Stagnation", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "JungleValley", "Cemetery", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "DrySea", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "DesertSpring", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "Dunes", "Cemetery", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "JungleValley", "Graveyard", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "Belfry", "Shrine", "SulphurVents"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Graveyard", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Graveyard", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Graveyard", "Arsenal", "Shipyard", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Residence", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Residence", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "TropicalIsland", "Cemetery", "Arsenal", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "JungleValley", "Graveyard", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Residence", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "GraveTrough", "Arsenal", "Belfry", "Leyline", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "Belfry", "Leyline", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "Belfry", "Canyon", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "AridLake", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "LavaLake", "Shrine", "SulphurVents"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "DrySea", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "DesertSpring", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "ColdRiver_", "Dunes", "Graveyard", "Arsenal", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "Pier", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "Pier", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "Pier", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DrySea", "Cemetery", "Arsenal", "Shrine", "Siege"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "DesertSpring", "Cemetery", "Arsenal", "Shrine", "Siege"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "Terrace", "Belfry", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "TropicalIsland", "Cemetery", "Arsenal", "LavaLake", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Dunes", "Cemetery", "Arsenal", "Shrine", "Siege"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "GraveTrough", "Arsenal", "LavaLake", "Leyline", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Graveyard", "Arsenal", "Belfry", "Shrine", "SulphurVents"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "LavaLake", "Leyline", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "LavaLake", "Canyon", "Shrine"],
        // vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells", "Cemetery", "Arsenal", "LavaLake", "AridLake", "Shrine"],
    ];

    for initial_group in initial_areas.iter().map(|group| group.to_vec()) {
        let remaining_maps: Vec<&str> = ALL_MAPS.iter().filter(|&map| !initial_group.contains(map)).cloned().collect();
    
        let additional_maps = 3; // Number of maps to add to the initial areas
        let total_combinations = binomial(remaining_maps.len(), additional_maps);
        let combinations_per_thread = total_combinations / (NUM_THREADS / initial_areas.len());
        let mut start_index = 0;

        for i in 0..(NUM_THREADS / initial_areas.len()) {
            let optimal_stack_scarab_ev = Arc::clone(&optimal_stack_scarab_ev);
            let all_cards_clone = all_cards.clone();
            let thread_remaining_maps = remaining_maps.clone();
    
            // Clone initial_group for use in the thread
            let initial_group_clone = initial_group.clone();
    
            let end_index = if i == NUM_THREADS - 1 {
                total_combinations
            } else {
                start_index + combinations_per_thread
            };

            let handle = thread::spawn(move || {
                thread_remaining_maps.iter().combinations(additional_maps).skip(start_index).take(end_index - start_index).for_each(|combination| {
                    let mut sampled_areas = initial_group_clone.to_vec();
                    sampled_areas.extend(combination.iter().map(|&&map| map));

                    let (total_raw_ev, total_stack_scarab_ev) = get_calculated_cards(&sampled_areas, &all_cards_clone);

                    let mut optimal_stack_scarab_ev = optimal_stack_scarab_ev.lock().unwrap();
                        // if total_stack_scarab_ev > 9900.0 {
                    if total_stack_scarab_ev > 10569.0 {
                        println!("[ev:{}|ssev:{}]: {:?}", total_raw_ev, total_stack_scarab_ev, sampled_areas);
                    } else if total_stack_scarab_ev > *optimal_stack_scarab_ev {
                        *optimal_stack_scarab_ev = total_stack_scarab_ev;
                        // println!("[{}]: {:?}", *optimal_stack_scarab_ev, sampled_areas);
                    }
                });
            });

            handles.push(handle);
            start_index = end_index;
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
