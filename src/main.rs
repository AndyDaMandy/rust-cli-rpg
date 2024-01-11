use std::io;
use rand::Rng;

fn main() {
    game();
}

fn game() {
    let potion = Item {
        name: String::from("Potion"),
        description: String::from("A potion that heals 10 HP"),
        effect: Effect::Heal,
        power: 10
    };

    let mut player_inventory = vec![potion.clone(), potion.clone(), potion.clone()];

    let mut main_character = Character {
        name: String::from("Main Character"),
        health: 50,
        strength: 10,
        armor: 10,
        speed: 10
    };

    let mut slime = Enemy {
        name: String::from("Slime"),
        health: 10,
        strength: 10,
        armor: 10,
        speed: 10
    };
    let mut main_character_health = main_character.health;
    let mut slime_health = slime.health;
    let mut round = 0;
    while main_character_health > 0 || slime_health > 0 {
        println!("Round {}", round);
        battle(&mut main_character, main_character_health, slime.clone(), slime_health, &mut player_inventory);
        round += 1;
    }
}

#[derive(Clone)]
pub struct Character {
    name: String,
    health: i32,
    strength: i32,
    armor: i32,
    speed: i32,
}

#[derive(Clone)]
pub struct Enemy {
    name: String,
    health: i32,
    strength: i32,
    armor: i32,
    speed: i32,
}

#[derive(Clone)]
pub struct Item {
    name: String,
    description: String,
    effect: Effect,
    power: i32
}

#[derive(Clone)]
pub enum Effect {
    Heal,
    Damage,
    Buff,
    Debuff
}

fn battle(player: &mut Character, mut player_health: i32, enemy: Enemy, mut enemy_health: i32, inventory: &mut Vec<Item>) {
    let mut turn = 0;
    while player_health > 0 || enemy_health > 0 {
        if turn % 2 == 0 {
            player_turn(player, player_health, enemy.clone(), enemy_health, inventory);
        } else {
            enemy_turn(player.clone(), player_health, enemy.clone());
        }
        turn += 1;
    }
}

fn player_turn(player: &mut Character, mut player_health: i32, enemy: Enemy, mut enemy_health: i32, inventory: &mut Vec<Item>) {
    println!("You've been attacked by {}!", enemy.name);
    println!("It's your turn {}!", player.name);
    println!("What would you like to do?");
    println!("1. Attack");
    println!("2. Defend");
    println!("3. Use Item");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    // if an action returns true, it's valid, else it's invalid and the turn repeats
    loop {
        match input {
            1 => if !attack(player, enemy.clone(), enemy_health) { continue; } else { break;},
            2 =>  if !defend(player) { continue;},
            3 => if !use_item(player, player_health, inventory) { continue;},
            _ => println!("Please enter a valid number!")
        }
        break;
    }

}

fn enemy_turn(mut player: Character, mut player_health: i32, mut enemy: Enemy) -> bool {
    println!("It's the enemy's turn!");
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..20);
    if attack_roll + enemy.strength > player.armor {
        let damage_roll = rng.gen_range(1..8);
        player_health -= damage_roll + enemy.strength;
        println!("The enemy hit you for {} damage! You have {} / {} Health remaining", damage_roll + enemy.strength, player_health, player.health);
    } else {
        println!("The enemy missed!");
    }
    return true;
}

fn attack(player: &mut Character, enemy: Enemy, mut enemy_health: i32) -> bool {
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..20);
    if attack_roll + player.strength > enemy.armor {
        let damage_roll = rng.gen_range(1..8);
        enemy_health -= damage_roll + player.strength;
        println!("You hit the enemy for {} damage!", damage_roll + player.strength);
    } else {
        println!("You missed!");
    }
    return true;
}

fn defend(player: &mut Character) -> bool {
    println!("You defend!");
    player.armor += 2;
    return true;
}

fn use_item(player: &mut Character, mut player_health: i32, inventory: &mut Vec<Item>) -> bool {
    if inventory.len() != 0 {
        println!("Which item would you like to use?");
        for (i, item) in inventory.iter().enumerate() {
            println!("{}. {}", i + 1, item.name);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");
        if input > inventory.len() as u32 {
            println!("Please enter a valid number!");
            return false;
        } else {
            let item = inventory[input as usize - 1].clone();
            match item.effect {
                Effect::Heal => {
                    player_health += item.power;
                    println!("You healed for {} HP!", item.power);
                },
                Effect::Buff => {
                    player.strength += item.power;
                    println!("You gained {} strength!", item.power);
                },
                _ => {}
            }
            inventory.remove(input as usize - 1);
        }
        return true;
    } else {
        println!("You have no items!");
        return false;
    }
}