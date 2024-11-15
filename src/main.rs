use rand::Rng;

const MAX_ROLLS:u32 = 1_000_000_000;

fn main() {
    let mut numbers:[u8;4] = [0;4];
    let mut rolls:u32 = 0;
    let mut max_ones:u8 = 0;

    while numbers[0] < 177 && rolls < MAX_ROLLS {
        numbers = [0,0,0,0];
        for _ in 0..231 {
            let roll = rand::thread_rng().gen_range(0..3);
            numbers[roll] += 1
        }
        rolls += 1;
        //println!("{rolls}");
        if numbers[0] > max_ones {
            max_ones = numbers[0]
        }
    }
    println!("highest ones roll: {max_ones}");
}
