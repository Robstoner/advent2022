use std::fs;

fn main() {
    // println!("Hello, world!");

    let filename = "input.txt";
    // let filename = "input_t.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // println!("{}", contents);

    // create int matrix from file

    let lines = contents.lines();
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            // println!("{}", c);
            row.push(c.to_digit(10).unwrap() as i32);
        }

        matrix.push(row);
    }

    let no_elements = matrix.len() * matrix[0].len();

    let mut count = 0;

    // Try 1

    // for i in 0..matrix.len() {
    //     for j in 0..matrix[i].len() {
    //         if i != 0 && i != matrix.len() - 1 {
    //             if j != 0 && j != matrix[i].len() - 1 {
    //                 // print!("{} ", matrix[i][j]);

    //                 if !(matrix[i][j] <= matrix[i][j - 1]
    //                     && matrix[i][j] <= matrix[i][j + 1]
    //                     && matrix[i][j] <= matrix[i - 1][j]
    //                     && matrix[i][j] <= matrix[i + 1][j])
    //                 {
    //                     count += 1;
    //                     println!("{} ", matrix[i][j])
    //                 }
    //             }
    //         }
    //     }
    //     // println!();
    // }

    // Try 2

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            // print!("{} ", matrix[i][j]);
            if j != 0 && j != matrix[i].len() - 1 {
                let mut ok_top = false;
                let mut ok_bottom = false;
                let mut ok_left = false;
                let mut ok_right = false;

                for k in 0..j {
                    if matrix[i][j] <= matrix[i][k] {
                        ok_left = true;
                        break;
                    }
                }

                for k in j + 1..matrix[i].len() {
                    if matrix[i][j] <= matrix[i][k] {
                        ok_right = true;
                        break;
                    }
                }

                for k in 0..i {
                    if matrix[i][j] <= matrix[k][j] {
                        ok_top = true;
                        break;
                    }
                }

                for k in i + 1..matrix.len() {
                    if matrix[i][j] <= matrix[k][j] {
                        ok_bottom = true;
                        break;
                    }
                }

                // print!("{} ", matrix[i][j]);
                // println!("{} {} {} {}", ok_top, ok_bottom, ok_left, ok_right);
                if ok_bottom && ok_left && ok_right && ok_top {
                    count += 1;
                    // println!("{} ", matrix[i][j])
                }
            }
        }
        // println!();
    }

    println!("Part 1 : {}", no_elements - count);

    let mut max_score = 1;

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            let mut score = 1;
            let mut nr_trees = 0;
            // print!("{} ", matrix[i][j]);
            
            for k in (0..j).rev() {
                // print!("l");
                if matrix[i][k] < matrix [i][k + 1] || matrix[i][k] < matrix[i][j] {
                    nr_trees += 1;
                }
                else {
                    nr_trees += 1;
                    break;
                }
            }
            // print!("{} ", nr_trees);
            score *= nr_trees;
            nr_trees = 0;
            for k in j + 1..matrix[i].len() {
                // print!("r");
                if matrix[i][k] < matrix [i][k - 1] || matrix[i][k] < matrix[i][j] {
                    nr_trees += 1;
                }
                else {
                    nr_trees += 1;
                    break;
                }
            }
            // print!("{} ", nr_trees);
            score *= nr_trees;
            nr_trees = 0;
            for k in (0..i).rev() {
                // print!("t");
                if matrix[k][j] < matrix [k + 1][j] || matrix[k][j] < matrix[i][j] {
                    nr_trees += 1;
                }
                else {
                    nr_trees += 1;
                    break;
                }
            }
            // print!("{} ", nr_trees);
            score *= nr_trees;
            nr_trees = 0;
            for k in i + 1..matrix.len() {
                // print!("b");
                if matrix[k][j] < matrix [k - 1][j] || matrix[k][j] < matrix[i][j] {
                    nr_trees += 1;
                }
                else {
                    nr_trees += 1;
                    break;
                }
            }
            // println!("{} ", nr_trees);
            score *= nr_trees;
            nr_trees = 0;
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 2 : {}", max_score);
}
