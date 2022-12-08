const SIZE: usize = 99;

// Part a is commented out at bottom, wrote some ugly code to try and get a fast split but it did
// not quite work out hehe. Do not have time to prettify any code so here is a raw ugly solution
// :)

fn main() {
    let mut trees = [[0u32; SIZE]; SIZE];
    let input = include_str!("../input.txt");
    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            trees[i][j] = char.to_digit(10).unwrap();
        }
    }
    
    let mut max_score = 0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            let up_score = get_score_col(&trees, i, j, true);
            let left_score = get_score_row(&trees[i], j, true);
            let right_score = get_score_row(&trees[i], j, false);
            let down_score = get_score_col(&trees, i, j, false);
            let score = up_score * left_score * right_score * down_score;
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);
}

fn get_score_row(arr: &[u32; SIZE], j: usize, left: bool) -> u32 {
    if j == 0 || j == SIZE-1 {
        return 0;
    }
    let mut score = 0;
    let mut curr = j;
    loop {
        if left {
            curr -= 1;
        } else {
            curr += 1;
        }
        score += 1;
        if curr == 0 || curr == SIZE-1 || arr[curr] >= arr[j] {
            return score;
        }
    }
}

fn get_score_col(trees: &[[u32; SIZE]; SIZE], i: usize, j: usize, up: bool) -> u32 {
    if i == 0 || i == SIZE-1 {
        return 0;
    }
    let mut score = 0;
    let mut curr = i;
    loop {
        if up {
            curr -= 1;
        } else {
            curr += 1;
        }
        score += 1;
        if curr == 0 || curr == SIZE-1 || trees[curr][j] >= trees[i][j] {
            return score;
        }
    }
}

    // let mut visible = [[[false; 4]; SIZE]; SIZE];
    // // from left
    // for i in 0..SIZE {
    //     visible[i][0][0] = true;
    //     for j in 1..SIZE {
    //         if (0..j).all(|x| trees[i][j] > trees[i][x]) {
    //             visible[i][j][0] = true;
    //         }
    //     }
    // }

    // // from up
    // for j in 0..SIZE {
    //     visible[0][j][1] = true;
    //     for i in 1..SIZE {
    //         if (0..i).all(|x| trees[i][j] > trees[x][j]) {
    //             visible[i][j][1] = true;
    //         }
    //     }
    // }

    // // from right
    // for i in 0..SIZE {
    //     visible[i][SIZE-1][2] = true;
    //     for j in 0..SIZE-1 {
    //         if (j+1..SIZE).all(|x| trees[i][j] > trees[i][x]) {
    //             visible[i][j][2] = true;
    //         }
    //     }
    // }

    // // from down
    // for j in 0..SIZE {
    //     visible[SIZE-1][j][3] = true;
    //     for i in 0..SIZE-1 {
    //         if (i+1..SIZE).all(|x| trees[i][j] > trees[x][j]) {
    //             visible[i][j][3] = true;
    //         }
    //     }
    // }

    // println!("Part a: {:?}", visible.iter().flatten().filter(|arr| arr.iter().any(|e| *e)).count());
