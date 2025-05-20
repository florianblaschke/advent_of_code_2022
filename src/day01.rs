use std::fs;

/// Our run fn now returns a result. The ? aborts early with an error.
/// In a real application you would use something like anyhow or eyre, but this works as well.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Day1Result { max, top_three } = day1()?;
    println!("There is a hungry elf, who carries {} calories", max);
    println!("Top three calorie loaded elves combined: {}", top_three);
    Ok(())
}

/// The result that we can work with in our test.
/// We could easily return a tuple (i32, i32), but named is almost always better.
struct Day1Result {
    max: i32,
    top_three: i32,
}

/// This function contains most of our logic, but doesn't print anything.
/// It's called both from run() and our test.
fn day1() -> Result<Day1Result, Box<dyn std::error::Error>> {
    let mut rations = fs::read_to_string("src/day01.input.txt")?
        .lines()
        .collect::<Vec<&str>>() // Need to collect lines before we split.
        .split(|line| line.is_empty())
        .map(|elve| {
            elve.into_iter().try_fold(0, |sum, ration| {
                let ration = ration.parse::<i32>()?;
                Ok(ration + sum)
            })
        })
        .collect::<Result<Vec<i32>, Box<dyn std::error::Error>>>()?;
    rations.sort_unstable_by(|a, b| b.cmp(a));

    // Our first item is the largest, if we don't have one, max = 0.
    let max: i32 = *rations.first().unwrap_or(&0);
    let top_three: i32 = rations.iter().take(3).sum();
    Ok(Day1Result { max, top_three })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() -> Result<(), Box<dyn std::error::Error>> {
        let Day1Result { max, top_three } = day1()?;
        assert_eq!(max, 70509);
        assert_eq!(top_three, 208567);
        Ok(())
    }
}
