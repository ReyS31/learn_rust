mod player;
use std::time::Duration;

use player::Player;
fn main() {
    royal_rumbel();
}

fn royal_rumbel() {
    let users_a: &mut [Player; 3] = &mut [
        Player::new("Rizma"),
        Player::new("Denzo"),
        Player::new("Palkimov"),
    ];

    let users_b: &mut [Player; 3] = &mut [
        Player::new("Akhrad"),
        Player::new("Bonnaque"),
        Player::new("Centrone"),
    ];

    users_a.iter_mut().for_each(|user| {
        user.info();
    });
    users_b.iter_mut().for_each(|user| {
        user.info();
    });

    let mut loop_breaker = 0;
    loop {
        loop_breaker = loop_breaker + 1;
        if loop_breaker > 9 {
            break;
        }
        users_a.iter_mut().for_each(|user| {
            user.level_up();
        });
        users_b.iter_mut().for_each(|user| {
            user.level_up();
        });
    }

    users_a.iter_mut().for_each(|user: &mut Player| {
        user.info();
    });
    users_b.iter_mut().for_each(|user: &mut Player| {
        user.info();
    });

    println!("=======ROUND START======");
    let mut a_won = false;
    loop_breaker = 1;
    loop {
        let total_dead_a = users_a
            .iter_mut()
            .fold(0, |acc: i8, user| acc + user.is_dead_num());
        let total_dead_b = users_b
            .iter_mut()
            .fold(0, |acc: i8, user| acc + user.is_dead_num());
        if total_dead_a >= 3 || total_dead_b >= 3 {
            if total_dead_b >= 3 {
                a_won = true;
            }
            break;
        }

        std::thread::sleep(Duration::from_secs(1));
        println!("=======ROUND {}======", loop_breaker);

        let user_a_atk = player::get_alive_players(users_a);
        let user_a_def = player::get_alive_players(users_a);
        let user_b_atk = player::get_alive_players(users_b);
        let user_b_def = player::get_alive_players(users_b);

        users_a[user_a_atk].attack(&mut users_b[user_b_def]);
        users_b[user_b_atk].attack(&mut users_a[user_a_def]);

        loop_breaker = loop_breaker + 1;
    }

    println!("GAME OVER");
    users_a.iter_mut().for_each(|user: &mut Player| {
        user.info();
    });
    users_b.iter_mut().for_each(|user: &mut Player| {
        user.info();
    });
    let total_dead_a = users_a
        .iter_mut()
        .fold(0, |acc: i8, user| acc + user.is_dead_num());
    let total_dead_b = users_b
        .iter_mut()
        .fold(0, |acc: i8, user| acc + user.is_dead_num());
    if a_won {
        println!("THE WINNER IS A");
    } else {
        println!("THE WINNER IS B");
    }
    println!("TOTAL ROUNDS: {}", loop_breaker);
    println!("TOTAL A DEATHS: {}", total_dead_a);
    println!("TOTAL B DEATHS: {}", total_dead_b);
}
