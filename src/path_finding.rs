use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const TILE_SIZE: i32 = 32;

pub struct PathFinder {}

type Location = (i32, i32);

impl PathFinder {
    pub fn path(&self, from: Location, to: Location) -> Vec<Location> {
        let neighbor_deltas = vec![
            (0, -1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut came_from = HashMap::<Location, Location>::new();
        let mut cost_so_far = HashMap::<Location, i32>::new();

        let mut open_list = BinaryHeap::new();
        open_list.push(PathNodePriority {
            loc: from,
            f_score: 0,
        });
        cost_so_far.insert(from, 0);

        while !open_list.is_empty() {
            let current = open_list.pop().unwrap();
            println!("Checking current: {:?}", current.loc);

            if current.loc.0 == to.0 && current.loc.1 == to.1 {
                println!("FOUND!");
                return self.reconstruct_path(current.loc, came_from);
            }

            for (i, j) in &neighbor_deltas {
                let neighbor_location = (current.loc.0 + i, current.loc.1 + j);

                let new_cost = cost_so_far.get(&current.loc).unwrap() + 10;
                let neighbhor_cost = cost_so_far.get(&neighbor_location);

                if neighbhor_cost.is_none() || &new_cost < neighbhor_cost.unwrap() {
                    cost_so_far.insert(neighbor_location, new_cost);
                    let priority = new_cost + self.heuristic(neighbor_location, to);
                    open_list.push(PathNodePriority {
                        loc: neighbor_location,
                        f_score: priority,
                    });
                    came_from.insert(neighbor_location, current.loc);
                }
            }
        }

        // TODO Let's return a result here when path is not found
        vec![]
    }

    fn reconstruct_path(
        &self,
        end: Location,
        came_from: HashMap<Location, Location>,
    ) -> Vec<Location> {
        let mut path = vec![];
        path.push(end);

        let mut current = &end;

        while let Some(next) = came_from.get(&current) {
            current = next;
            path.push(next.clone());
        }

        path.reverse();
        path
    }

    fn heuristic(&self, a: Location, b: Location) -> i32 {
        (a.0 - b.1).abs() + (a.0 - b.1).abs()
    }
}

pub struct PathNodePriority {
    pub loc: Location,
    pub f_score: i32,
}

impl Ord for PathNodePriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl PartialOrd for PathNodePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNodePriority {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for PathNodePriority {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_finding() {
        let pf = PathFinder {};
        let path = pf.path((1, 1), (4, 6));
        assert_eq!(vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (4, 6)], path);
    }
}
