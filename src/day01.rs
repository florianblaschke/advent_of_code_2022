use std::fs;

pub fn run() {
    let content = match fs::read_to_string("src/day01.input.txt") {
        Ok(s) => s,
        Err(e) => panic!("Couldn't read src/input.txt: {e}"),
    };

    let v: Vec<&str> = content.split("\n").collect();

    let elves = v.split(|line| line.is_empty()).collect::<Vec<&[&str]>>();

    let mut max = 0;
    let mut rations: Vec<i32> = Vec::new();

    for elf in elves {
        let mut calories = 0;
        for ration in elf {
            let rat = match ration.parse::<i32>() {
                Ok(num) => num,
                Err(e) => {
                    panic!("Failed parsing: {}", e)
                }
            };

            calories += rat;
        }
        if calories > max {
            max = calories
        }
        rations.push(calories);
    }

    rations.sort_unstable_by(|a, b| b.cmp(a));
    let top_three: i32 = rations.iter().take(3).sum();

    println!("There is a hungry elf, who carries {} calories", { max });
    println!("Top three calorie loaded elves combined: {}", top_three)
}
