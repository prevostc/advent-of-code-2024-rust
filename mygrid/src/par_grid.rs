use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;

use crate::{grid::Grid, point::Point};

impl<T: Sized + Send + Sync + Clone> Grid<T> {
    pub fn par_iter(&self) -> impl ParallelIterator<Item = &T> {
        self.content.par_iter()
    }

    pub fn par_iter_item_and_position(&self) -> impl ParallelIterator<Item = (Point, &T)> {
        let width = self.width;
        let height = self.height;
        self.content
            .par_iter()
            .enumerate()
            .map(move |(i, t)| (Point::new_usize(i / width, i % height), t))
    }
}
