fn spirale(circle: usize) {
    // Init array :
    let size = circle * 2;
    let mut spirale = vec![vec![0; size]; size]; // [ row, column]
    let mut number_to_display: u32 = 1;

    // Lets compute index :
    for i in 0..circle {
        let length = size - 2 * i;
        for j in 0..length {
            spirale[i][j + i] = number_to_display;
            number_to_display = number_to_display + 1;
        }
        for j in 1..length {
            spirale[i + 1 + (j - 1)][size - i - 1] = number_to_display;
            number_to_display = number_to_display + 1;
        }
        for j in 1..length {
            spirale[size - 1 - i][size - 1 - 1 - i - (j - 1)] = number_to_display;
            number_to_display = number_to_display + 1;
        }
        for j in 1..length - 1 {
            spirale[size - 1 - 1 - i - (j - 1)][i] = number_to_display;
            number_to_display = number_to_display + 1;
        }
    }

    // Print :
    for i in 0..size {
        for j in 0..size {
            print!("{}\t", spirale[i][j]);
        }
        println!("");
    }
}

fn main() {
    spirale(3);
}
