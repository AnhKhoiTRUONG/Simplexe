struct ProgrammationLineaire {
    programme: Vec<Vec<f32>>,
    nb_contraints: usize,
}

impl ProgrammationLineaire {
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
            let mut max_div = 0.;
            for i in 0..self.nb_contraints {
                if self.programme[i][indice] > 0. {
                    if self.programme[self.programme.len() - 1][indice] / self.programme[i][indice]
                        > max_div
                    {
                        max_div = self.programme[self.programme.len() - 1][indice]
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
    let ctr1 = vec![1., -2., 2., 1., 0., 1.];
    let ctr2 = vec![2., 1., 2., 0., 1., 3.];
    let ctr3 = vec![2., 1., -1., 0., 0., 0.];
    let mut prog1 = ProgrammationLineaire {
        programme: vec![ctr1, ctr2, ctr3],
        nb_contraints: 2,
    };
    prog1.solved();
}
