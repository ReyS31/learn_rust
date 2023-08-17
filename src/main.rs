use rand::Rng;
struct User {
    name: String,
    level: u32,
    atk: i128,
    def: i128,
    hp: i128,
    mp: i128,
}

impl User {
    fn new(player_name: &str) -> Self {
        let mut rng = rand::thread_rng();
        User {
            name: String::from(player_name),
            level: 1,
            atk: rng.gen_range(1..10),
            def: rng.gen_range(1..7),
            hp: rng.gen_range(10..30),
            mp: rng.gen_range(5..20),
        }
    }

    fn level_up(&mut self) {
        let level = self.level + 1;
        let lv_ok: i128 = level.into();
        let mut rng = rand::thread_rng();
        self.level = level;
        self.atk = self.atk + rng.gen_range(lv_ok..10 + lv_ok);
        self.def = self.def + rng.gen_range(lv_ok..7 + lv_ok);
        self.hp = self.hp + rng.gen_range(lv_ok + 10..30 + lv_ok);
        self.mp = self.mp + rng.gen_range(lv_ok + 5..20 + lv_ok);
        println!("{} Leveled up to {}", self.name, self.level);
    }

    fn info(&mut self) {
        println!("=======INFO======");
        println!("name: {}", self.name);
        println!("level: {}", self.level);
        println!("atk: {}", self.atk);
        println!("def: {}", self.def);
        println!("hp: {}", self.hp);
        println!("mp: {}", self.mp);
    }

    fn damaged(&mut self, damage: i128) {
        let total_damage = (self.def - damage).abs();
        self.hp = self.hp - total_damage;
        println!("{} got {} damage", self.name, total_damage);
    }

    fn attack(&mut self, foe: &mut User) {
        foe.damaged(self.atk);
    }

    fn is_dead(&mut self) -> bool {
        self.hp <= 0
    }
}

fn main() {
    let mut a: i32 = 6;
    println!("Nilai dari a adalah {a}");
    a = a.pow(2);

    println!("Nilai dari a adalah {a}");

    let mut user = User::new("Rizma");
    let mut user_den = User::new("Denzo");

    user.info();
    user_den.info();

    let mut loop_breaker = 0;
    loop {
        loop_breaker = loop_breaker + 1;
        if loop_breaker > 9 {
            break;
        }
        user.level_up();
        user_den.level_up();
    }

    user.info();
    user_den.info();

    loop {
        user.attack(&mut user_den);
        user_den.attack(&mut user);
        if user.is_dead() || user_den.is_dead() {
            break;
        }
    }
    println!("GAME OVER");
    user.info();
    user_den.info();

    let winners_name: String = if user.hp < user_den.hp {
        user_den.name
    } else {
        user.name
    };
    println!("{} wins", winners_name);
}
