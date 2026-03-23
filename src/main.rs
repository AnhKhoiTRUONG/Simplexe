use std::io::{self, Write};

struct ProgrammationLineaire {
    programme: Vec<Vec<f32>>,
    nb_contraints: usize,
    base: Vec<u8>,
}

impl ProgrammationLineaire {
    fn new() -> Self {
        let mut buffer = String::new();
        let mut prog = Vec::new();

        print!("Number of variables: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Fail to read variables");
        let nb_var: usize = buffer.trim().parse().expect("Input not integer");
        buffer.clear();

        print!("Number of contraints: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Fail to read contraintes");
        let nb_ctr = buffer.trim().parse().expect("Input not integer");
        buffer.clear();

        for i in 0..nb_ctr + 1 {
            println!("Line {i}:");
            io::stdout().flush().unwrap();
            let mut les_contraintes: Vec<f32> = Vec::new();
            for j in 0..(nb_var + nb_ctr + 1) {
                print!("x{j}: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Fail to read variables");
                les_contraintes.push(buffer.trim().parse::<f32>().expect("Failed to read input"));
                buffer.clear();
            }
            prog.push(les_contraintes);
        }

        let mut bases = Vec::new();
        for i in nb_var..(nb_ctr + nb_var) {
            bases.push(i as u8);
        }

        ProgrammationLineaire {
            programme: prog,
            nb_contraints: nb_ctr,
            base: bases,
        }
    }
    fn solved(&mut self) {
        loop {
            // find the max of the z(max) line:
            let mut max = self.programme[self.nb_contraints][0];
            let mut indice = 0;
            let mut ind = 0;
            for i in self.programme[self.nb_contraints].iter() {
                if *i > max {
                    max = *i;
                    indice = ind;
                }
                ind += 1;
            }
            println!("Max is {max} with indice = {indice}");

            // choisir la base
            let mut cand = 100;
            let mut min_div = 100000.;
            for i in 0..self.nb_contraints {
                if self.programme[i][indice] > 0. {
                    println!(
                        "{}/{} = {}",
                        self.programme[i][self.programme[0].len() - 1],
                        self.programme[i][indice],
                        self.programme[i][self.programme[0].len() - 1] / self.programme[i][indice]
                    );
                    if self.programme[i][self.programme[0].len() - 1] / self.programme[i][indice]
                        < min_div
                    {
                        min_div = self.programme[i][self.programme[0].len() - 1]
                            / self.programme[i][indice];
                        cand = i;
                    }
                }
            }
            if cand == 100 {
                println!("Termine, pas trouver solution optimale");
            }
            println!(
                "Prends {} a la position ({cand},{indice})",
                self.programme[cand][indice]
            );

            self.base[cand] = indice as u8;

            // Update the line base first
            let mut pivot = self.programme[cand][indice];
            for j in 0..self.programme[0].len() {
                self.programme[cand][j] = self.programme[cand][j] / pivot;
                println!("{}", self.programme[cand][j]);
            }

            for i in 0..self.nb_contraints + 1 {
                let mut fois;
                if i == self.nb_contraints {
                    fois = max;
                } else {
                    fois = self.programme[i][indice];
                }
                for j in 0..self.programme[0].len() {
                    // if i == cand {
                    // self.programme[i][j] = self.programme[i][j] / self.programme[i][indice];
                    // } else {
                    // println!("{}", fois);
                    if i != cand {
                        self.programme[i][j] =
                            self.programme[i][j] - fois * self.programme[cand][j];
                    }
                }
            }
            println!("{:?}", self.programme);
            println!("Base = {:?}", self.base);
            let mut dont_break = 0;
            for i in self.programme[self.nb_contraints].iter() {
                if *i > 0. {
                    dont_break = 1;
                }
            }
            if dont_break == 0 {
                break;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    // let ctr1 = vec![1., -2., 2., 1., 0., 1.];
    // let ctr2 = vec![2., 1., 2., 0., 1., 3.];
    // let ctr3 = vec![2., 1., -1., 0., 0., 0.];
    // let mut prog1 = ProgrammationLineaire {
    //     programme: vec![ctr1, ctr2, ctr3],
    //     nb_contraints: 2,
    //     base: vec![4, 5],
    // };
    // prog1.solved();
    let mut prog1 = ProgrammationLineaire::new();
    println!("{:?}", prog1.programme);
    println!("{:?}", prog1.base);
    prog1.solved();
}
