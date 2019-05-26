const SIZE_X: usize = 8;
const SIZE_Y: usize = 8;

mod test;

fn show_field(field: &[[i32; SIZE_X];SIZE_Y]) {
    println!("");
    print!("   ");
    let c = SIZE_X + 65;
    let c = c as u8;
    for len in 65u8..c {
        print!("{} ", len as char);
    }
    println!("");

    for i in 0..SIZE_Y {
        print!("{}: ", i+1);
        for j in 0..SIZE_X {
            match field[i][j]{
                0 => print!(" |"),
                1 => print!("X|"),
                2 => print!("O|"),
                _ => panic!("Field contains invalid characters!"),
            }
        }
        println!("");
    }
}

fn winner(field: &[[i32; SIZE_X];SIZE_Y]) -> i32 {
    let mut count_p1 = 0;
    let mut count_p2 = 0;

    for i in 0..SIZE_Y {
        for j in 0..SIZE_X {
            match field[i][j]{
                1 => count_p1 += 1,
                2 => count_p2 += 1,
                _ => (),
            }
        }
    }
    if count_p1 == count_p2 {
        0
    } else if count_p1 > count_p2 {
        1
    } else {
        2
    }

}

fn check_valid_boundaries(rev_x: i32, rev_y: i32) -> bool {
    if rev_x >= 0 {
        if rev_x < (SIZE_X as i32) {
            if rev_y >= 0 {
                if rev_y < (SIZE_Y as i32) {
                    return true
                }
            }
        }
    }
    return false
}

fn valid_turn(field: &[[i32; SIZE_X];SIZE_Y], player: i32, x_koord: usize, y_koord: usize) -> bool {
    if x_koord > SIZE_X || y_koord > SIZE_Y {
        println!("ERROR, not a valid turn");
        return false
    } else {
        let opponent = 3 - player;
        if field[x_koord][y_koord] != 0 {
            println!("ERROR, not a valid turn");
            return false
        }
        let mut i: i32 = -1;
        let mut j: i32 = -1;
        while i <= 1 {
            let help_y = (y_koord as i32) + i;

            while j <= 1 {
                let help_x = (x_koord as i32) + j;

                if field[help_y as usize][help_x as usize] != opponent {
                    j += 1;
                    continue;
                }

                let mut rev_y = y_koord as i32;
                let mut rev_x = x_koord as i32;
                rev_y += i;
                rev_x += j;
                while check_valid_boundaries(rev_x, rev_y) {
                //((((rev_x as i32) += j) >= 0) && ((rev_x as i32) < (SIZE_X as i32)) && (((rev_y as i32) += i) >= 0) && ((rev_y as i32) < (SIZE_Y as i32)) {
                    if field[rev_y as usize][rev_x as usize] == player {
                        return true
                    } else if field[rev_y as usize][rev_x as usize] == 0 {
                        break;
                    }
                    rev_y += i;
                    rev_x += j;
                }
                j += 1;
            }
            i += 1;
        }
        return false
    }
}

fn execute_turn(field: &mut [[i32; SIZE_X];SIZE_Y], player :i32, x_koord: usize, y_koord: usize) -> &mut [[i32; SIZE_X];SIZE_Y] {
    let opponent = 3 - player;
    let mut i: i32 = -1;
    let mut j: i32 = -1;

    while i <= 1 {
        let mut help_y = (y_koord as i32) + i;

        while j <= 1 {
            let mut help_x = (x_koord as i32) + j;

            if field[help_y as usize][help_x as usize] != opponent {
                j += 1;
                continue;
            }

            //let mut temp_y = y_koord + i;
            //let mut temp_x = x_koord + j;
            while check_valid_boundaries(help_x, help_y) {
                if field[help_y as usize][help_x as usize] == opponent {
                    help_y += i;
                    help_x += j;
                    continue;
                } else if field[help_y as usize][help_x as usize] == player {
                    while help_y != (y_koord as i32) || help_x != (x_koord as i32) {
                        help_y -= i;
                        help_x -= j;
                        field[help_y as usize][help_x as usize] = player;
                    }
                    break;
                } else {
                    break;
                }
            }
            j += 1;
        }
        i += 1;
    }
    field
}

fn possible_turns(field: &[[i32; SIZE_X];SIZE_Y], player: i32) -> i32 {
    let mut possibilities = 0;
    for y_koord in 0..SIZE_Y {
        for x_koord in 0..SIZE_X {
            if valid_turn(&field, player, x_koord, y_koord) == true {
                possibilities += 1;
            }
        }
    }
    possibilities
}

// fn player_turn(player: i32) -> [[i32; SIZE_X];SIZE_Y]{
//     if possible_turns(field, player) != 0 {
//         let mut x_koord = 0;
//         let mut y_koord = 0;
//         loop {
//             //TODO: input
//             break;
//         }
//     } else {
//         println!("No turns possible for player {}", player);
//     }
// }


fn main() {
    let mut field = [[0; SIZE_X]; SIZE_Y];
    show_field(&field);
}
