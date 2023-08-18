use rand::Rng;

pub struct Player {
    name: String,
    level: u32,
    atk: i64,
    def: i64,
    hp: i64,
    mp: i64,
    rng: rand::rngs::ThreadRng,
}

impl Player {
    pub fn new(player_name: &str) -> Self {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        Player {
            name: String::from(player_name),
            level: 1,
            atk: rng.gen_range(1..=10),
            def: rng.gen_range(1..=7),
            hp: rng.gen_range(10..=30),
            mp: rng.gen_range(5..=20),
            rng: rng,
        }
    }

    pub fn new_custom(
        player_name: &str,
        level: Option<u32>,
        atk: i64,
        def: i64,
        hp: i64,
        mp: i64,
    ) -> Self {
        Player {
            name: String::from(player_name),
            level: level.unwrap_or(1),
            atk,
            def,
            hp,
            mp,
            rng: rand::thread_rng(),
        }
    }

    pub fn level_up(&mut self) {
        let level = self.level + 1;
        let lv_ok: i64 = level.into();
        self.level = level;
        self.atk = self.atk + self.rng.gen_range(lv_ok..=10 + lv_ok);
        self.def = self.def + self.rng.gen_range(lv_ok..=7 + lv_ok);
        self.hp = self.hp + self.rng.gen_range(lv_ok + 10..=30 + lv_ok);
        self.mp = self.mp + self.rng.gen_range(lv_ok + 5..=20 + lv_ok);
        println!("{} Leveled up to {}", self.name, self.level);
    }

    pub fn info(&mut self) {
        println!("=======INFO======");
        println!("name: {}", self.name);
        println!("level: {}", self.level);
        println!("atk: {}", self.atk);
        println!("def: {}", self.def);
        println!("hp: {}", self.hp);
        println!("mp: {}", self.mp);
    }

    fn damaged(&mut self, damage: i64, attacker: &str) {
        if self.is_dead() {
            println!("{} attaked {}'s dead body", attacker, self.name);
            return;
        }
        let fully_defend: i8 = self.rng.gen_range(0..=10);
        if fully_defend == 10 {
            println!("SUCCESSFULLY EVADED");
            println!(
                "{} attacked {}, {} got {} damage",
                attacker, self.name, self.name, 0
            );
            return;
        }

        let mut true_dmg = damage;
        let crit_atk: i8 = self.rng.gen_range(0..=10);

        if crit_atk == 10 {
            println!("CRITICAL");
            true_dmg = damage + ((self.def - damage).abs() / 2);
        }

        let total_damage = if self.def > true_dmg {
            0
        } else {
            (self.def - true_dmg).abs()
        };
        self.hp = if self.hp < total_damage {
            0
        } else {
            self.hp - total_damage
        };
        println!(
            "{} attacked {}, {} got {} damage",
            attacker, self.name, self.name, total_damage,
        );

        if self.hp > 0 && self.rng.gen_bool(3.0 / 20.0) {
            println!("{} awakened true potential by damaged", self.name);
            self.buff();
        }

        if self.rng.gen_bool(3.0 / 20.0) {
            self.heal();
        }
    }

    pub fn attack(&mut self, foe: &mut Player) {
        if self.is_dead() {
            println!("{} already dead, can't attack", self.name);
            return;
        }
        foe.damaged(self.atk, &self.name);
    }

    pub fn is_dead(&mut self) -> bool {
        self.hp <= 0
    }

    pub fn is_dead_num(&mut self) -> i8 {
        if self.hp <= 0 {
            1
        } else {
            0
        }
    }

    fn buff(&mut self) {
        println!("BUFFS");
        let buff_rate = self.rng.gen_range(0.1..1.0);
        // ATK buff
        if self.rng.gen_bool(buff_rate) {
            let atk_added = (self.atk as f64 * self.rng.gen_range(0.1..0.5)).floor() as i64;
            println!("{} attack buffed by {}", self.name, atk_added);
            self.atk = self.atk + atk_added;
        }
        if self.rng.gen_bool(buff_rate) {
            let def_added = (self.def as f64 * self.rng.gen_range(0.1..0.5)).floor() as i64;
            println!("{} def buffed by {}", self.name, def_added);
            self.def = self.def + def_added;
        }
        if self.rng.gen_bool(buff_rate) {
            let free_heal = (self.hp as f64 * self.rng.gen_range(0.1..0.5)).floor() as i64;
            println!("{} healed {}hp", self.name, free_heal);
            self.hp = self.hp + free_heal;
        }
    }

    fn heal(&mut self) {
        if self.mp > 0 {
            println!("HEALING");
            let mana_used = if self.mp == 1 {
                1
            } else {
                self.rng.gen_range(1..self.mp)
            };
            let total_heal = mana_used * self.rng.gen_range(1..4);
            self.hp = self.hp + total_heal;
            self.mp = self.mp - mana_used;
            println!(
                "{} got {}hp from healing by {}mp",
                self.name, total_heal, mana_used
            );
        }
    }
}

pub fn get_alive_players(players: &mut [Player]) -> usize {
    let mut indexes = vec![];
    let mut rng = rand::thread_rng();
    for (index, player) in players.iter_mut().enumerate() {
        if !player.is_dead() {
            indexes.push(index);
        }
    }

    if indexes.len() > 1 {
        indexes[rng.gen_range(0..indexes.len())]
    } else {
        indexes[0]
    }
}
