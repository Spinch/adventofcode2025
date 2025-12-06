

pub mod utils {
    use std::option::Option;
    use grid::Grid;

    const DIRECTION4: &[(isize, isize)] = &[
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    static DIRECTION8: &[(isize, isize)] = &[
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub struct Field<T> {
        pub data: Grid<T>,
    }

    pub struct NeighborsIterator<'a, T> {
        field: &'a Field<T>,
        coord: (usize, usize),
        direction_i: usize,
        directions: &'static[(isize, isize)],
    }

    pub struct Neighbor<'a, T> {
        pub val: &'a T,
        pub coord: (usize, usize),
    }

    impl <T> Field<T> {
        pub fn from_vec(vec: Vec<T>, cols: usize) -> Self<> {
            Field{data: Grid::from_vec(vec, cols)}
        }

        pub fn neighbors4(&self, coord: (usize, usize)) -> NeighborsIterator<'_, T> {
            NeighborsIterator {field: &self, coord, direction_i: 0, directions: DIRECTION4}
        }

        pub fn neighbors8(&self, coord: (usize, usize)) -> NeighborsIterator<'_, T> {
            NeighborsIterator {field: &self, coord, direction_i: 0, directions: DIRECTION8}
        }
    }

    impl<'a, T> Iterator for NeighborsIterator<'a, T> {
        type Item = Option<Neighbor<'a, T>>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.direction_i >= self.directions.len() {
                return None;
            }
            let direction = self.directions[self.direction_i];
            self.direction_i += 1;
            let new_x = self.coord.0 as isize + direction.0;
            let new_y = self.coord.1 as isize + direction.1;
            if new_x < 0
                || new_y < 0
                || new_x >= self.field.data.cols() as isize
                || new_y >= self.field.data.rows() as isize {
                return Some(None);
            }
            Some(Some(Neighbor{val: &self.field.data[(new_x as usize, new_y as usize)], coord: (new_x as usize, new_y as usize)}))
        }
    }

}