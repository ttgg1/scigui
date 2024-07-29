pub mod data_module {
    struct Data {
        xy: Vec<[f64; 2]>,
    }
    impl Data {
        fn new(data: Vec<[f64; 2]>) -> Self {
            Self { xy: data }
        }
        fn rebin(&mut self, bins: usize) -> Vec<[f64; 2]> {
            let mut sum_x: f64 = 0.0;
            let mut sum_y: f64 = 0.0;
            let mut new_xy: Vec<[f64; 2]> = Vec::new();

            for (i, xy) in self.xy.iter().enumerate() {
                if i % bins == 0 {
                    new_xy.push([sum_x, sum_y]);
                } else {
                    sum_x += xy[0];
                    sum_y += xy[1];
                }
            }

            new_xy.clone_into(&mut self.xy);
            new_xy
        }
    }
}
