use std::io;
use rand::Rng;

fn main() {
    game();
}

fn game() {
    let mut main_character = Character::new("Placeholder", 100, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10);

    let mut goblin = Enemy::new("Goblin", 100, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10);

    let mut round = 0;
    while main_character.health > 0 || goblin.health > 0 {
        println!("Round {}", round);
        battle(main_character, goblin);
        round += 1;
    }
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

struct Item {
    name: String,
    description: String,
    value: i32,
    weight: i32,
    effect: Effect::Heal,
    power: i32
}

enum Effect {
    Heal,
    Damage,
    Buff,
    Debuff
}

fn battle(mut player: Character, mut enemy: Enemy, item: Item) {
    let mut turn = 0;
    while player.health > 0 || enemy.health > 0 {
        if turn % 2 == 0 {
            player_turn(player, enemy, item: Item);
        } else {
            enemy_turn(player, enemy);
        }
        turn += 1;
    }
}

fn player_turn(player: Character, enemy: Enemy, item: Item) {
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
        3 => use_item(player, item),
        _ => println!("Please enter a valid number!")
    }
}

fn enemy_turn(player: Character, enemy: Enemy) {
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

}

fn attack(player: Character, mut enemy: Enemy) {
    let mut rng = rand::thread_rng();
    let attack_roll = rng.gen_range(1..20);
    if attack_roll + player.strength > enemy.armor_class {
        let damage_roll = rng.gen_range(1..8);
        enemy.health -= damage_roll + player.strength;
        println!("You hit the enemy for {} damage!", damage_roll + player.strength);
    } else {
        println!("You missed!");
    }
}

fn defend(mut player: Character) {
    println!("You defend!");
    player.armor_class += 2;
}

fn use_item(player: Character, item: Item) {
    println!("You use an item!");
}