use std::io;
use rand::Rng;

fn main() {
    game();
}

fn game() {
    let potion = Item {
        name: String::from("Potion"),
        description: String::from("A potion that heals 10 HP"),
        value: 10,
        weight: 1,
        effect: Effect::Heal,
        power: 10
    };

    let mut main_character = Character {
        name: String::from("Main Character"),
        health: 50,
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
        armor_class: 10,
        initiative: 10,
        speed: 10,
        proficiency_bonus: 10,
        inventory: vec![potion, potion, potion],
    };

    let mut goblin = Enemy {
        name: String::from("Goblin"),
        health: 10,
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
        armor_class: 10,
        initiative: 10,
        speed: 10,
        proficiency_bonus: 10
    };

    let mut round = 0;
    while main_character.health > 0 || goblin.health > 0 {
        println!("Round {}", round);
        battle(main_character, goblin);
        round += 1;
    }
}
pub struct Character {
    name: String,
    health: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    armor_class: i32,
    initiative: i32,
    speed: i32,
    proficiency_bonus: i32,
    inventory: Vec<Item>
}

#[derive(Clone)]
pub struct Enemy {
    name: String,
    health: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    armor_class: i32,
    initiative: i32,
    speed: i32,
    proficiency_bonus: i32
}

#[derive(Copy, Clone)]
pub struct Item {
    name: String,
    description: String,
    value: i32,
    weight: i32,
    effect: Effect,
    power: i32
}
#[derive(Copy, Clone)]
pub enum Effect {
    Heal,
    Damage,
    Buff,
    Debuff
}

fn battle(mut player: Character, mut enemy: Enemy) {
    let mut turn = 0;
    while player.health > 0 || enemy.health > 0 {
        if turn % 2 == 0 {
            player_turn(player, enemy);
        } else {
            enemy_turn(player, enemy);
        }
        turn += 1;
    }
}

fn player_turn(player: Character, enemy: Enemy) {
    println!("It's your turn!");
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
            1 => if !attack(player, enemy) { continue; } else { break;},
            2 =>  if !defend(player) { continue;},
            3 => if !use_item(player) { continue;},
            _ => println!("Please enter a valid number!")
        }
        break;
    }

}

fn enemy_turn(player: Character, enemy: Enemy) -> bool {
    println!("It's the enemy's turn!");
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..20);
    if attack_roll + enemy.strength > player.armor_class {
        let damage_roll = rng.gen_range(1..8);
        player.health -= damage_roll + enemy.strength;
        println!("The enemy hit you for {} damage!", damage_roll + enemy.strength);
    } else {
        println!("The enemy missed!");
    }
    return true;
}

fn attack(player: Character, mut enemy: Enemy) -> bool {
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..20);
    if attack_roll + player.strength > enemy.armor_class {
        let damage_roll = rng.gen_range(1..8);
        enemy.health -= damage_roll + player.strength;
        println!("You hit the enemy for {} damage!", damage_roll + player.strength);
    } else {
        println!("You missed!");
    }
    return true;
}

fn defend(mut player: Character) -> bool {
    println!("You defend!");
    player.armor_class += 2;
    return true;
}

fn use_item(mut player: Character) -> bool {
    if player.inventory.len() != 0 {
        println!("Which item would you like to use?");
        for (i, item) in player.inventory.iter().enumerate() {
            println!("{}. {}", i + 1, item.name);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");
        if input > player.inventory.len() as u32 {
            println!("Please enter a valid number!");
            return false;
        } else {
            let item = player.inventory[input as usize - 1];
            match item.effect {
                Effect::Heal => {
                    player.health += item.power;
                    println!("You healed for {} HP!", item.power);
                },
                Effect::Buff => {
                    player.strength += item.power;
                    println!("You gained {} strength!", item.power);
                },
            }
            player.inventory.remove(input as usize - 1);
        }
        return true;
    } else {
        println!("You have no items!");
        return false;
    }
}