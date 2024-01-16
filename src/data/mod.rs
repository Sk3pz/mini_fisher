use crate::data::fish::{FishData, FishRarity, FishType};
use crate::data::rods::{BaseRod, RodData, RodModifier};
use crate::data::shop::RodRarity;

pub mod fish;
pub mod rods;
pub mod shop;
pub mod userfile;

pub fn fish_data() -> FishData {
    FishData {
        weight_factor: 0.8,
        fish: vec![
            FishType {
                name: "Old Boot".to_string(),
                depth: 1,
                value: 1,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Tin Can".to_string(),
                depth: 1,
                value: 1,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Seaweed".to_string(),
                depth: 1,
                value: 2,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Plastic Bag".to_string(),
                depth: 1,
                value: 1,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Plastic Bottle".to_string(),
                depth: 1,
                value: 1,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Tire".to_string(),
                depth: 1,
                value: 5,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Trout".to_string(),
                depth: 10,
                value: 8,
                min_weight: 15,
                avg_weight: 20,
                max_weight: 25,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Rainbow Trout".to_string(),
                depth: 10,
                value: 30,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 5,
                min_rarity: FishRarity::Elusive,
            },
            FishType {
                name: "Bass".to_string(),
                depth: 25,
                value: 6,
                min_weight: 5,
                avg_weight: 12,
                max_weight: 20,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Salmon".to_string(),
                depth: 20,
                value: 12,
                min_weight: 5,
                avg_weight: 10,
                max_weight: 30,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Tuna".to_string(),
                depth: 30,
                value: 75,
                min_weight: 40,
                avg_weight: 300,
                max_weight: 500,
                min_rarity: FishRarity::Rare,
            },
            FishType {
                name: "Marlin".to_string(),
                depth: 45,
                value: 30,
                min_weight: 200,
                avg_weight: 210,
                max_weight: 400,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Perch".to_string(),
                depth: 8,
                value: 5,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 4,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Catfish".to_string(),
                depth: 25,
                value: 30,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 4,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Swordfish".to_string(),
                depth: 40,
                value: 80,
                min_weight: 50,
                avg_weight: 180,
                max_weight: 500,
                min_rarity: FishRarity::Rare,
            },
            FishType {
                name: "Pike".to_string(),
                depth: 18,
                value: 8,
                min_weight: 20,
                avg_weight: 28,
                max_weight: 40,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Cod".to_string(),
                depth: 20,
                value: 8,
                min_weight: 6,
                avg_weight: 9,
                max_weight: 15,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Herring".to_string(),
                depth: 10,
                value: 5,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Mackerel".to_string(),
                depth: 15,
                value: 4,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 4,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Sardine".to_string(),
                depth: 5,
                value: 2,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Eel".to_string(),
                depth: 20,
                value: 12,
                min_weight: 10,
                avg_weight: 15,
                max_weight: 30,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Shark".to_string(),
                depth: 60,
                value: 50,
                min_weight: 200,
                avg_weight: 300,
                max_weight: 500,
                min_rarity: FishRarity::Rare,
            },
            FishType {
                name: "Tilapia".to_string(),
                depth: 10,
                value: 5,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Carp".to_string(),
                depth: 10,
                value: 5,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Guppy".to_string(),
                depth: 5,
                value: 2,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Jellyfish".to_string(),
                depth: 5,
                value: 12,
                min_weight: 1,
                avg_weight: 2,
                max_weight: 3,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Dogfish".to_string(),
                depth: 15,
                value: 15,
                min_weight: 4,
                avg_weight: 8,
                max_weight: 21,
                min_rarity: FishRarity::Rare,
            },
            FishType {
                name: "Stingray".to_string(),
                depth: 5,
                value: 50,
                min_weight: 31,
                avg_weight: 40,
                max_weight: 75,
                min_rarity: FishRarity::Elusive,
            },
            FishType {
                name: "Barramundi".to_string(),
                depth: 8,
                value: 10,
                min_weight: 1,
                avg_weight: 13,
                max_weight: 110,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Anglerfish".to_string(),
                depth: 100,
                value: 75,
                min_weight: 60,
                avg_weight: 70,
                max_weight: 110,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Lanternfish".to_string(),
                depth: 110,
                value: 50,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Common,
            },
            FishType {
                name: "Fangtooth".to_string(),
                depth: 110,
                value: 150,
                min_weight: 60,
                avg_weight: 70,
                max_weight: 110,
                min_rarity: FishRarity::Elusive,
            },
            FishType {
                name: "Viperfish".to_string(),
                depth: 110,
                value: 75,
                min_weight: 1,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Daggertooth".to_string(),
                depth: 110,
                value: 75,
                min_weight: 1,
                avg_weight: 4,
                max_weight: 6,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Barracudina".to_string(),
                depth: 110,
                value: 50,
                min_weight: 0,
                avg_weight: 1,
                max_weight: 2,
                min_rarity: FishRarity::Uncommon,
            },
            FishType {
                name: "Antarctic Toothfish".to_string(),
                depth: 110,
                value: 150,
                min_weight: 50,
                avg_weight: 97,
                max_weight: 330,
                min_rarity: FishRarity::Rare,
            },
            FishType {
                name: "Loch Ness Monster".to_string(),
                depth: 150,
                value: 1000,
                min_weight: 800,
                avg_weight: 1000,
                max_weight: 1200,
                min_rarity: FishRarity::Mythical,
            },
        ],
    }
}

pub fn rod_data() -> RodData {
    RodData {
        rods: vec![
            BaseRod {
                name: "Stick with String".to_string(),
                description: "A stick with a string tied to it. It gets the job done.".to_string(),
                catch_chance: 0.32,
                catch_rate: 24.0,
                depth: 15,
                weight_limit: 20,
                cost: 0.0,
                rarity: RodRarity::Unobtainable,
            },
            BaseRod {
                name: "Fiberglass Casting Rod".to_string(),
                catch_chance: 0.35,
                catch_rate: 20.0,
                depth: 20,
                weight_limit: 40,
                cost: 200.0,
                rarity: RodRarity::Common,
                description: "Faster to reel but lower chance of catching fish.".to_string(),
            },
            BaseRod {
                name: "Composite Casting Rod".to_string(),
                catch_chance: 0.38,
                catch_rate: 16.5,
                depth: 25,
                weight_limit: 45,
                cost: 500.0,
                rarity: RodRarity::Uncommon,
                description: "Faster to reel but lower chance of catching fish.".to_string(),
            },
            BaseRod {
                name: "Bamboo Casting Rod".to_string(),
                catch_chance: 0.42,
                catch_rate: 15.0,
                depth: 30,
                weight_limit: 50,
                cost: 1000.0,
                rarity: RodRarity::Uncommon,
                description: "Faster to reel but lower chance of catching fish.".to_string(),
            },
            // Second set of rods
            BaseRod {
                name: "Fiberglass Spinning Rod".to_string(),
                catch_chance: 0.4,
                catch_rate: 18.0,
                depth: 20,
                weight_limit: 40,
                cost: 250.0,
                rarity: RodRarity::Common,
                description: "Slow to reel but higher chance of catching fish.".to_string(),
            },
            BaseRod {
                name: "Composite Spinning Rod".to_string(),
                catch_chance: 0.45,
                catch_rate: 17.5,
                depth: 25,
                weight_limit: 45,
                cost: 500.0,
                rarity: RodRarity::Uncommon,
                description: "Slow to reel but higher chance of catching fish.".to_string(),
            },
            BaseRod {
                name: "Bamboo Spinning Rod".to_string(),
                catch_chance: 0.5,
                catch_rate: 16.0,
                depth: 30,
                weight_limit: 50,
                cost: 1000.0,
                rarity: RodRarity::Uncommon,
                description: "Slow to reel but higher chance of catching fish.".to_string(),
            },
            // Third set of rods
            BaseRod {
                name: "Composite Overhead Rod".to_string(),
                catch_chance: 0.65,
                catch_rate: 15.0,
                depth: 80,
                weight_limit: 300,
                cost: 4000.0,
                rarity: RodRarity::Rare,
                description: "Slower to reel but can catch the deepest fish with the best catch rates.".to_string(),
            },
            BaseRod {
                name: "Graphite Overhead Rod".to_string(),
                catch_chance: 0.7,
                catch_rate: 12.0,
                depth: 100,
                weight_limit: 600,
                cost: 5000.0,
                rarity: RodRarity::Rare,
                description: "Slower to reel but can catch the deepest fish with the best catch rates.".to_string(),
            },
            BaseRod {
                name: "Titanium Overhead Rod".to_string(),
                catch_chance: 0.75,
                catch_rate: 10.0,
                depth: 150,
                weight_limit: 1000,
                cost: 8000.0,
                rarity: RodRarity::Epic,
                description: "Slower to reel but can catch the deepest fish with the best catch rates.".to_string(),
            },
            // Fourth set of rods
            BaseRod {
                name: "Composite Fly Rod".to_string(),
                catch_chance: 0.5,
                catch_rate: 8.0,
                depth: 45,
                weight_limit: 250,
                cost: 4000.0,
                rarity: RodRarity::Rare,
                description: "Incredibly quick catch rates but can't reach deeper waters.".to_string(),
            },
            BaseRod {
                name: "Bamboo Fly Rod".to_string(),
                catch_chance: 0.55,
                catch_rate: 5.0,
                depth: 50,
                weight_limit: 320,
                cost: 5000.0,
                rarity: RodRarity::Rare,
                description: "Incredibly quick catch rates but can't reach deeper waters.".to_string(),
            },
            BaseRod {
                name: "Graphite Fly Rod".to_string(),
                catch_chance: 0.6,
                catch_rate: 3.0,
                depth: 80,
                weight_limit: 450,
                cost: 8000.0,
                rarity: RodRarity::Epic,
                description: "Incredibly quick catch rates but can't reach deeper waters.".to_string(),
            },
        ],
        modifiers: vec![
            RodModifier {
                name: "Old".to_string(),
                catch_rate: 1.5,
                catch_chance: -0.02,
                depth: -5,
                weight_limit: -5,
            },
            RodModifier {
                name: "Better".to_string(),
                catch_rate: -1.5,
                catch_chance: 0.02,
                depth: 0,
                weight_limit: 0,
            },
            RodModifier {
                name: "Upgraded".to_string(),
                catch_rate: -2.5,
                catch_chance: 0.2,
                depth: 20,
                weight_limit: 20,
            },
        ],
    }
}