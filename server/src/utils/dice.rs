use rand::Rng;

pub fn get_dice_value() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

pub fn throw_dice() -> usize {
    let mut dice_value: usize = 0;
    // player/client sends MessageType::ThrowDice
    // << message exchange >>
    match get_dice_value() {
        6 => {
            dice_value += 6;
            // << message exchange >>;
            match get_dice_value() {
                6 => {
                    dice_value += 6;
                    // << message exchange >>;
                    match get_dice_value() {
                        // if we throw 6 three times, it gets reset
                        6 => {
                            dice_value = 0;
                            // << message exchange >>
                        },
                        n => dice_value += n
                    }
                },
                n => dice_value += n
            }
        },
        n => dice_value += n
    }

    dice_value
}