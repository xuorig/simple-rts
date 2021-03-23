use crate::path_finding::Location;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Funnel {
    portals: Vec<Portal>,
}

/// A portal is kind of a door, it has a left and right position
#[derive(Debug)]
pub struct Portal {
    left: Vec2,
    right: Vec2,
}

impl PartialEq for Portal {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl Funnel {
    /// Creates a new Funnel (A list of portals or channels)
    /// Given a path of waypoints or nodes
    fn from_path(path: Vec<Location>, tile_size: f32) -> Self {
        Self {
            portals: Self::generate_portals(path, tile_size),
        }
    }

    fn generate_portals(path: Vec<Location>, grid_world_size: f32) -> Vec<Portal> {
        let mut portals = vec![];

        for (i, loc) in path.iter().enumerate() {
            let next_node = path.get(i + 1);

            if let Some(next_node) = next_node {
                let diff = (next_node.0 - loc.0, next_node.1 - loc.1);

                let portal = match diff {
                    // Right above us
                    (0, 1) => Portal {
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size,
                            next_node.1 as f32 * grid_world_size,
                        ),
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size,
                            next_node.1 as f32 * grid_world_size,
                        ),
                    },

                    // Diagonal Top-Right
                    (1, 1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size - grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size - grid_world_size / 2.0,
                        ),
                    },

                    // To our right
                    (1, 0) => Portal {
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size,
                            next_node.1 as f32 * grid_world_size,
                        ),
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size,
                            next_node.1 as f32 * grid_world_size + grid_world_size,
                        ),
                    },

                    // Diagonal Bottom-Right
                    // [ ] l
                    //  r [ ]
                    (1, -1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size * 1.5,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size - grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0,
                        ),
                    },

                    // Bellow Us
                    // [ ]
                    // l r
                    // [ ]
                    (0, -1) => Portal {
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size,
                            next_node.1 as f32 * grid_world_size + grid_world_size,
                        ),
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size,
                            next_node.1 as f32 * grid_world_size + grid_world_size,
                        ),
                    },

                    // Diagonal Bottom-Left
                    //   r [ ]
                    // [  ] l
                    (-1, -1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size * 1.5,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size * 1.5,
                        ),
                    },

                    // To our Left
                    // [ ]|<-[ ]
                    (-1, 0) => Portal {
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size,
                            next_node.1 as f32 * grid_world_size + grid_world_size,
                        ),
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size,
                            next_node.1 as f32 * grid_world_size,
                        ),
                    },

                    // Diagonal Top-Left
                    // (-1, 1) => {}
                    // [ ] r
                    //  l [ ]
                    (-1, 1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0,
                            next_node.1 as f32 * grid_world_size - grid_world_size / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size * 1.5,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0,
                        ),
                    },

                    _ => panic!("Should never receive path items that are not adjacent!"),
                };

                portals.push(portal);
            }
        }

        portals
    }

    /// Returns the optimal path across the grid
    /// running the funnel / string pulling algorithm
    pub fn path(&self) -> Vec<Vec2> {
        // TODO
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_portals() {
        //       [ ]
        //       [ ]
        //    [ ]
        // [ ]
        // [ ]
        // [ ]
        let funnel = Funnel::from_path(vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4), (2, 5)], 32.0);

        assert_eq!(
            vec![
                Portal {
                    left: Vec2::new(0.0, 32.0),
                    right: Vec2::new(32.0, 32.0)
                },
                Portal {
                    left: Vec2::new(0.0, 64.0),
                    right: Vec2::new(32.0, 64.0)
                },
                Portal {
                    left: Vec2::new(16.0, 112.0),
                    right: Vec2::new(48.0, 80.0)
                },
                Portal {
                    left: Vec2::new(48.0, 144.0),
                    right: Vec2::new(80.0, 112.0)
                },
                Portal {
                    left: Vec2::new(64.0, 160.0),
                    right: Vec2::new(96.0, 160.0)
                }
            ],
            funnel.portals
        );
    }

    #[test]
    fn test_path() {
        //       [ ]
        //       [ ]
        //    [ ]
        // [ ]
        // [ ]
        // [ ]
        let funnel = Funnel::from_path(vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4), (2, 5)], 32.0);

        let expected: Vec<Vec2> = vec![];
        assert_eq!(expected, funnel.path());
    }
}
