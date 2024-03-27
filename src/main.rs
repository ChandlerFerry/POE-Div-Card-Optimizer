use itertools::Itertools;
use std::collections::HashSet;
use serde::Deserialize;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

#[derive(Clone, PartialEq, Debug)]
struct AreaEv {
    ssev: f64,
    areas: Vec<String>,
}

impl Eq for AreaEv {}

impl Ord for AreaEv {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ssev.partial_cmp(&other.ssev).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AreaEv {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_top_results(all_cards: &[Card], initial_areas: &[&str], remaining_maps: &[&str], additional_maps: usize) -> Vec<AreaEv> {
    let top_results = Arc::new(Mutex::new(BinaryHeap::new()));
    let total_combinations = binomial(remaining_maps.len(), additional_maps);
    let combinations_per_thread = total_combinations / NUM_THREADS;
    let mut handles = vec![];
    let mut start_index = 0;

    // Convert remaining_maps to Vec<String> here
    let remaining_maps_as_string: Vec<String> = remaining_maps.iter().map(|&s| s.to_string()).collect();

    for i in 0..NUM_THREADS {
        let top_results = Arc::clone(&top_results);
        let all_cards_clone = all_cards.to_owned();
        // Clone the already converted Vec<String>
        let remaining_maps_clone = remaining_maps_as_string.clone();
        let initial_areas_as_string: Vec<String> = initial_areas.iter().map(|&s| s.to_string()).collect();

        let end_index = if i == NUM_THREADS - 1 { total_combinations } else { start_index + combinations_per_thread };

        let handle = thread::spawn(move || {
            remaining_maps_clone.iter().combinations(additional_maps).skip(start_index).take(end_index - start_index).for_each(|combination| {
                let mut sampled_areas = initial_areas_as_string.clone();
                sampled_areas.extend(combination.into_iter().cloned()); // Since combination is already Vec<String>

                let (_, total_stack_scarab_ev) = get_calculated_cards(&sampled_areas.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &all_cards_clone);
                let mut top_results = top_results.lock().unwrap();
                if top_results.len() < 100 {
                    top_results.push(Reverse(AreaEv { ssev: total_stack_scarab_ev, areas: sampled_areas }));
                } else {
                    let min_ev = top_results.peek().unwrap().0.ssev;
                    if total_stack_scarab_ev > min_ev {
                        top_results.pop();
                        top_results.push(Reverse(AreaEv { ssev: total_stack_scarab_ev, areas: sampled_areas }));
                    }
                }
            });
        });

        handles.push(handle);
        start_index = end_index;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let top_results = Arc::try_unwrap(top_results).expect("Lock still has multiple owners");
    let top_results = top_results.into_inner().expect("Mutex cannot be locked");

    top_results.into_sorted_vec().into_iter().map(|rev| rev.0).collect()
}

fn rerun_top_results(all_cards: &[Card], top_results: Vec<AreaEv>, remaining_maps: &[&str], additional_maps: usize) -> Vec<AreaEv> {
    let updated_top_results = Arc::new(Mutex::new(BinaryHeap::new()));
    let total_combinations = top_results.len() * binomial(remaining_maps.len(), additional_maps);
    let combinations_per_thread = total_combinations / NUM_THREADS;
    let remaining_maps_as_string: Vec<String> = remaining_maps.iter().map(|&s| s.to_string()).collect();
    let mut handles = vec![];

    for i in 0..NUM_THREADS {
        let updated_top_results = Arc::clone(&updated_top_results);
        let all_cards_clone = all_cards.to_owned();
        let remaining_maps_clone = remaining_maps_as_string.clone();
        let top_results_clone = top_results.clone();

        let start_index = i * combinations_per_thread;
        let end_index = if i == NUM_THREADS - 1 { total_combinations } else { start_index + combinations_per_thread };

        let handle = thread::spawn(move || {
            for j in start_index..end_index {
                let top_result_index = j / binomial(remaining_maps_clone.len(), additional_maps);
                let combination_index = j % binomial(remaining_maps_clone.len(), additional_maps);

                if top_result_index < top_results_clone.len() {
                    let current_top_result = &top_results_clone[top_result_index];
                    let mut sampled_areas = current_top_result.areas.clone();

                    remaining_maps_clone.iter().combinations(additional_maps).nth(combination_index).map(|combination| {
                        sampled_areas.extend(combination.into_iter().cloned());

                        let (_, total_stack_scarab_ev) = get_calculated_cards(&sampled_areas.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &all_cards_clone);
                        let mut updated_top_results = updated_top_results.lock().unwrap();
                        if updated_top_results.len() < 100 {
                            updated_top_results.push(Reverse(AreaEv { ssev: total_stack_scarab_ev, areas: sampled_areas }));
                        } else {
                            let min_ev = updated_top_results.peek().unwrap().0.ssev;
                            if total_stack_scarab_ev > min_ev {
                                updated_top_results.pop();
                                updated_top_results.push(Reverse(AreaEv { ssev: total_stack_scarab_ev, areas: sampled_areas }));
                            }
                        }
                    });
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let updated_top_results = Arc::try_unwrap(updated_top_results).expect("Lock still has multiple owners");
    let updated_top_results = updated_top_results.into_inner().expect("Mutex cannot be locked");

    updated_top_results.into_sorted_vec().into_iter().map(|rev| rev.0).collect()
}


fn main() {
    let all_cards_data = fs::read_to_string("cards.json").expect("Failed to read cards.json");
    let all_cards: Vec<Card> = serde_json::from_str(&all_cards_data).expect("Failed to parse cards.json");
    
    // First Round with Initial Areas
    let initial_areas: Vec<&str> = vec!["DefiledCathedral", "Phantasmagoria", "Maze", "Cells"];
    let (_initial_raw_ev, initial_stack_scarab_ev) = get_calculated_cards(&initial_areas, &all_cards);
    println!("Initial areas SSEV: {}", initial_stack_scarab_ev);

    // Second Round with 5 Additional Maps
    let remaining_maps: Vec<&str> = ALL_MAPS.iter().filter(|&map| !initial_areas.contains(map)).cloned().collect();
    let additional_maps = 5;
    let top_results = calculate_top_results(&all_cards, &initial_areas, &remaining_maps, additional_maps);

    // Third Round: Rerun Top 100 Results with 3 Additional Maps
    let third_round_additional_maps = 3;
    let final_top_results = rerun_top_results(&all_cards, top_results, &remaining_maps, third_round_additional_maps);

    // Print Final Top Results
    for wrapped_area_ev in final_top_results.iter().take(100) {
        println!("SSEV: {}, Areas: {:?}", &wrapped_area_ev.ssev, &wrapped_area_ev.areas);
    }
}
