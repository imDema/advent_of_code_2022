use eyre::Result;
use ndarray::Array2;

struct TreeLine {
    grid: Array2<u8>,
    viz: Array2<bool>,
}

impl TreeLine {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let width = lines.next().unwrap().len();
        let height = lines.count() + 1;
    
        let grid = Array2::from_shape_vec((width, height), s.chars().filter(|c| !c.is_whitespace()).map(|c| c as u8 - b'0').collect()).unwrap();
        let viz = Array2::from_shape_simple_fn(grid.raw_dim(), || false);

        Self {
            grid,
            viz,
        }
    }

    fn height_viz(&mut self) {
        let shape = self.grid.raw_dim();
        for i in 0..shape[0] {
            let mut frontier = 0;
            for j in 0..shape[1] {
                if self.grid[[i, j]] > frontier {
                    frontier = self.grid[[i, j]];
                    self.viz[[i, j]] = true;
                }
            }

            let mut frontier = 0;
            for j in (0..shape[1]).rev() {
                if self.grid[[i, j]] > frontier {
                    frontier = self.grid[[i, j]];
                    self.viz[[i, j]] = true;
                }
            }
        }
        for j in 0..shape[1] {
            let mut frontier = 0;
            for i in 0..shape[0] {
                if self.grid[[i, j]] > frontier {
                    frontier = self.grid[[i, j]];
                    self.viz[[i, j]] = true;
                }
            }

            let mut frontier = 0;
            for i in (0..shape[0]).rev() {
                if self.grid[[i, j]] > frontier {
                    frontier = self.grid[[i, j]];
                    self.viz[[i, j]] = true;
                }
            }
        }
    }

        
    fn visibility(&mut self) {
        self.height_viz();
        
        let shape = self.grid.raw_dim();
        self.viz.column_mut(0).iter_mut().for_each(|v| *v = true);
        self.viz.row_mut(0).iter_mut().for_each(|v| *v = true);
        self.viz.column_mut(shape[0] - 1).iter_mut().for_each(|v| *v = true);
        self.viz.row_mut(shape[1] - 1).iter_mut().for_each(|v| *v = true);
    }

    fn point_viz(&self, coord: [usize; 2]) -> usize {
        let dim = self.grid.raw_dim();
        let mut p = 1;
        p *= self.line_viz(coord, (0..coord[0]).rev().map(|i| [i, coord[1]]));
        p *= self.line_viz(coord, (coord[0] + 1..dim[0]).map(|i| [i, coord[1]]));
        
        p *= self.line_viz(coord, (0..coord[1]).rev().map(|j| [coord[0], j]));
        p *= self.line_viz(coord, (coord[1] + 1..dim[1]).map(|j| [coord[0], j]));
        
        p
    }

    fn line_viz(&self, coord: [usize; 2], line: impl IntoIterator<Item=[usize; 2]>) -> usize {
        let h = self.grid[coord];
        let mut cnt = 0;
        for c in line {
            if self.grid[c] < h {
                cnt += 1;
            } else {
                cnt += 1;
                break;
            }
        }
        cnt
    }

    fn count_viz(&self) -> usize {
        self.viz.iter().filter(|&v| *v).count()
    }
}


fn main() -> Result<()> {
    let input = std::io::read_to_string(std::io::stdin())?;

    // PART 1
    let mut t = TreeLine::parse(&input);
    t.visibility();



    let r = t.count_viz();

    println!("{r}");

    // PART 2
    let r = t.grid.indexed_iter().map(|(i, _)| t.point_viz([i.0, i.1])).max().unwrap();

    println!("{r}");
    Ok(())
}
