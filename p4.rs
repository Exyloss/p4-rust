use std::io::{stdin,stdout,Write};
fn main() {
    let mut v: Vec<Vec<u8>> = vec![vec![0;7];7];
    let num: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];
    let mut j: u8 = 1;
    let mut err: u8 = 0;
    let yellow = "\x1b[33m";
    let white = "\x1b[39m";
    let red = "\x1b[31m";
    loop {
        affiche_tours(&v);
        let mut r = String::new();
        if j == 1 {
            println!("{}joueur 1{}", red, white);
            print!(":");
        } else {
            println!("{}joueur 2{}", yellow, white);
            print!(":");
        }
        let _=stdout().flush();
        stdin().read_line(&mut r).expect("Did not enter a correct string");
        let n: usize = r.trim().parse().unwrap_or(9);
        if num.contains(&n) {
            poser_pion(n, &mut v, j);
        } else {
            println!("{}Valeur incorrecte", red);
            println!("\x1b[32mAide : \nQuitter le programme : ctrl+c\nSélectionner une colonne : 1..7.");
            err = 1;
        }
        if is_winning(&mut v) == true {
            affiche_tours(&v);
            if j == 1 {
                println!("{}Le joueur 1 a gagné.", red);
            } else {
                println!("{}Le joueur 2 a gagné.", yellow);
            }
            let mut r2 = String::new();
            print!("{}Rejouer ? (o/n) ", white);
            let _=stdout().flush();
            stdin().read_line(&mut r2).expect("Did not enter a correct string");
            if r2.trim() == "o" {
                v = vec![vec![0;7];7];
                j = 2;
                err = 0;
            } else {
                break;
            }
        }
        if j == 1 && err == 0 {
            j = 2;
        } else if err == 0 && j == 2 {
            j = 1;
        } else {
            err = 0;
        }
    }
}

fn affiche_tours(vec: &Vec<Vec<u8>>) {
    let mut temp = String::new();
    println!("\x1b[34m  1 2 3 4 5 6 7");
    for i in 0..7 {
        temp=temp+"\x1b[34m"+&(i+1).to_string()+" ";
        for j in 0..7 {
            if vec[i][j] == 1 {
                temp = temp+"\x1b[31m0 \x1b[39m";
            } else if vec[i][j] == 2 {
                temp = temp+"\x1b[33m0 \x1b[39m";
            } else {
                temp = temp+"\x1b[39m0 ";
            }
        }
        println!("{}", temp);
        temp = String::new();
    }
}

fn poser_pion(col: usize, vec: &mut Vec<Vec<u8>>, j: u8) {
    for i in (0..7).rev() {
        if vec[i][col-1] == 0 && j == 1 {
            vec[i][col-1] = 1;
            break;
        } else if vec[i][col-1] == 0 && j == 2 {
            vec[i][col-1] = 2;
            break;
        }
    }
}

fn is_winning(vec: &mut Vec<Vec<u8>>) -> bool {
    let mut victoire = 0;
    for i in 0..7 {
        for j in 0..6 {
            if victoire == 3 {
                return true
            } else if vec[i][j] == vec[i][j+1] && vec[i][j] != 0 {
                victoire=victoire+1;
            } else {
                victoire = 0;
            }
        }
    }
    
    victoire = 0;
    for i in 0..7 {
        for j in 0..6 {
            if victoire == 3 {
                return true
            }
            if vec[j][i] == vec[j+1][i] && vec[j][i] != 0 {
                victoire=victoire+1;
            } else {
                victoire = 0;
            }
        }
    }
    for i in (3..7).rev() {
        if victoire == 3 {
            return true
        }
        for j in 0..3 {            
            if victoire == 3 {
                return true
            }
            victoire = 0;
            for k in 0..3 {
                if victoire == 3 {
                    return true
                }
                if vec[i-k][j+k] == vec[i-k-1][j+k+1] && vec[i-k][j+k] != 0 {
                    victoire=victoire+1;
                    println!("{}", victoire);
                } else {
                    victoire = 0;
                }
            }
        }
    }
    for i in (3..7).rev() {
        if victoire == 3 {
            return true
        }
        for j in (3..7).rev() {
            if victoire == 3 {
                return true
            }
            victoire = 0;
            for k in 0..3 {
                if victoire == 3 {
                    return true
                }
                if vec[i-k][j-k] == vec[i-k-1][j-k-1] && vec[i-k][j-k] != 0 {
                    victoire=victoire+1;
                } else {
                    victoire = 0;
                }
            }
        }
    }
    return false
}
