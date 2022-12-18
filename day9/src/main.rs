use std::fs;

fn main() {
    // let filename = "input.txt";
    let filename = "input_t.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // println!("{}", contents);

    
    let lines = contents.lines();
    
    // create empty bool matrix
    let mut mat: Vec<Vec<bool>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        let mut row: Vec<bool> = Vec::with_capacity(1000);
        for _ in 0..1000 {
            row.push(false);
        }
        mat.push(row);
    }

    mat[500][500] = true;
    let mut h_x = 500;
    let mut h_y = 500;
    let mut t_x = 500;
    let mut t_y = 500;

    for line in lines {
        let mut split = line.split_whitespace();
        let direction =  split.next().unwrap();
        let amount: i32 = split.next().unwrap().parse().unwrap();
        // println!("{} {} ", direction, amount);
        // println!("{} {} {} {} ", h_x, h_y, t_x, t_y);
        if direction == "R" {
            // print!("R");
            for _ in 0..amount {
                // print!("R{} {} {} {} ", h_x, h_y, t_x, t_y);

                h_x += 1;
                if h_x - t_x > 1 && h_y as isize - t_y as isize == 1 {
                    t_x += 1;
                    t_y += 1;
                    mat[t_x][t_y] = true;
                } else if h_x - t_x > 1 && h_y as isize - t_y as isize == !0 {
                    t_x += 1;
                    t_y -= 1;
                    mat[t_x][t_y] = true;
                } else if h_x - t_x > 1 {
                    t_x += 1;
                    mat[t_x][t_y] = true;
                }
            }

        } else if direction == "L"{
            // print!("L");
            for _ in 0..amount {
                // print!("L{} {} {} {} ", h_x, h_y, t_x, t_y);

                h_x -= 1;
                if t_x - h_x > 1 && h_y as isize - t_y as isize == 1 {
                    t_x -= 1;
                    t_y += 1;
                    mat[t_x][t_y] = true;
                } else if t_x - h_x > 1 && h_y as isize - t_y as isize == !0 {
                    t_x -= 1;
                    t_y -= 1;
                    mat[t_x][t_y] = true;
                } else if t_x - h_x > 1 {
                    t_x -= 1;
                    mat[t_x][t_y] = true;
                }
            }

        } else if direction == "U"{
            // print!("U");
            for _ in 0..amount {
                // print!("U{} {} {} {} ", h_x, h_y, t_x, t_y);

                h_y -= 1;
                if t_y - h_y > 1 && h_x as isize - t_x as isize == 1 {
                    t_x += 1;
                    t_y -= 1;
                    mat[t_x][t_y] = true;
                } else if t_y - h_y > 1 && h_x as isize - t_x as isize == !0 {
                    t_x -= 1;
                    t_y -= 1;
                    mat[t_x][t_y] = true;
                } else if t_y - h_y > 1 {
                    t_y -= 1;
                    mat[t_x][t_y] = true;
                }
                
            }

        } else if direction == "D"{
            // print!("D");
            for _ in 0..amount {
                // print!("D{} {} {} {} ", h_x, h_y, t_x, t_y);

                h_y += 1;
                if h_y - t_y > 1 && h_x as isize - t_x as isize == 1 {
                    t_x += 1;
                    t_y += 1;
                    mat[t_x][t_y] = true;
                } else if h_y - t_y > 1 && h_x as isize - t_x as isize == !0 {
                    t_x -= 1;
                    t_y += 1;
                    mat[t_x][t_y] = true;
                } else if h_y - t_y > 1 {
                    t_y += 1;
                    mat[t_x][t_y] = true;
                }
            }
        }
        // print!("{} {} {} {} ", h_x, h_y, t_x, t_y);

    }

    // calculate nr of true values (part1)
    let mut nr_true = 0;
    for row in mat.iter() {
        for val in row.iter() {
            if *val {
                nr_true += 1;
            }
        }
    }
    println!("Part 1: {}", nr_true);
}
