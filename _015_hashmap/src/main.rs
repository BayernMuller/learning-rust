use std::collections::HashMap;

fn main() {
    let mut worldcup_wins = HashMap::new();
    worldcup_wins.insert(String::from("Germany"), 4);
    worldcup_wins.insert(String::from("South Korea"), 0);

    let countries = vec![
        String::from("germany"),
        String::from("france"),
        String::from("spain")
    ];

    let win_count = vec![4, 2, 1];

    let mut ranking: HashMap<_, _> = countries.iter().zip(win_count.iter()).collect();

    let england = String::from("england");
    let england_win_count = 1; // 56 years ago, England is stupid little dumb dumb

    ranking.insert(&england, &england_win_count);
    println!("{}: {}", england, england_win_count);
    println!("{:?}", ranking);

    let mut germany_win_count = ranking.get(&String::from("germany"));
    match germany_win_count {
        Some(i) => println!("germany: {}", i),
        None => ()
    };

    let korea = String::from("south korea");
    ranking.entry(&korea).or_insert(&0);
    for (key, value) in &ranking {
        println!("{}: {}", key, value);
    }
}
