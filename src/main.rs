use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
}

fn game() {
    let mut main_character = Character::new("Placeholder", 100, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10);

    let mut goblin = Enemy::new("Goblin", 100, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10);



}

struct Character {
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

struct Enemy {
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

fn battle(player: Character, enemy: Enemy) {
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
    println!("4. Run");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    match input {
        1 => attack(player, enemy),
        2 => defend(player),
        3 => use_item(player),
        4 => run(player, enemy),
        _ => println!("Please enter a valid number!")
    }
}

fn attack(player: Character, mut enemy: Enemy) {
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1, 20);
    if attack_roll + player.strength > enemy.armor_class {
        let damage_roll = rng.gen_range(1, 8);
        enemy.health -= damage_roll + player.strength;
        println!("You hit the enemy for {} damage!", damage_roll + player.strength);
    } else {
        println!("You missed!");
    }
}