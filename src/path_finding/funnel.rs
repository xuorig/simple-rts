use bevy::prelude::*;

type Location = (i32, i32);

/// A portal is kind of a door, it has a left and right position
/// Left and Right are from the perspective of walking through the door
///      l
/// o ->
///      r
#[derive(Debug)]
pub struct Portal {
    pub left: Vec2,
    pub right: Vec2,
}

impl PartialEq for Portal {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

#[derive(Debug)]
pub struct Funnel {
    start: Vec2,
    end: Vec2,
    pub portals: Vec<Portal>,
}

impl Funnel {
    /// Creates a new Funnel (A list of portals or channels)
    /// Given a path of waypoints or nodes, most likely produced by something like A*
    pub fn from_path(
        start: Vec2,
        end: Vec2,
        path: Vec<Location>,
        tile_size: f32,
        world_width: f32,
        world_height: f32,
    ) -> Self {
        Self {
            start,
            end,
            portals: Self::generate_portals(path, tile_size, world_width, world_height),
        }
    }

    /// Generates a list of Portals given a path in a grid
    /// The grid path members are Tuples of i32 but the portals are in world coordinates (Vec2)
    fn generate_portals(
        path: Vec<Location>,
        grid_world_size: f32,
        world_width: f32,
        world_height: f32,
    ) -> Vec<Portal> {
        let mut portals = vec![];

        for (i, loc) in path.iter().enumerate() {
            let next_node = path.get(i + 1);

            if let Some(next_node) = next_node {
                let diff = (next_node.0 - loc.0, next_node.1 - loc.1);

                let portal = match diff {
                    // Top
                    (0, 1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size - world_height / 2.0,
                        ),
                    },

                    // Diagonal Top-Right
                    (1, 1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size
                                - grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size
                                - grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                    },

                    // Right
                    (1, 0) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size - world_height / 2.0,
                        ),
                    },

                    // Bottom-Right
                    (1, -1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size * 1.5
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size
                                - grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                    },

                    // Bottom
                    (0, -1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size
                                - world_height / 2.0,
                        ),
                    },

                    // Bottom-Left
                    (-1, -1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size * 1.5
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size * 1.5
                                - world_height / 2.0,
                        ),
                    },

                    // Left
                    (-1, 0) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size
                                - world_height / 2.0,
                        ),
                    },

                    // Top-Left
                    (-1, 1) => Portal {
                        left: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size
                                - grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                        right: Vec2::new(
                            next_node.0 as f32 * grid_world_size + grid_world_size * 1.5
                                - world_width / 2.0,
                            next_node.1 as f32 * grid_world_size + grid_world_size / 2.0
                                - world_height / 2.0,
                        ),
                    },

                    _ => panic!("Should never receive path items that are not adjacent!"),
                };

                portals.push(portal);
            }
        }

        portals
    }

    /// Returns the optimal path across the grid running the funnel / string pulling algorithm
    /// See Simple Stupid Funnel Algorithm: https://digestingduck.blogspot.com/2010/03/simple-stupid-funnel-algorithm.html
    /// See Paper: https://www.aaai.org/Papers/AAAI/2006/AAAI06-148.pdf
    pub fn string_pull(&self) -> Vec<Vec2> {
        let mut points: Vec<Vec2> = vec![];
        points.push(self.start);

        // Setup our initial search state
        let first_portal = self.portals.get(0);
        if first_portal.is_none() {
            return points;
        }

        let mut apex = self.start;
        let mut portal_left = first_portal.unwrap().left;
        let mut portal_right = first_portal.unwrap().right;

        let mut left_index = 0;
        let mut right_index = 0;
        let mut i = 1;

        while i < self.portals.len() {
            let portal = self.portals.get(i).unwrap();

            println!(
                "Apex = {} Current Portal R = {} Next Portal R = {}",
                apex, portal_right, portal.right
            );
            println!(
                "Apex = {} Current Portal L = {} Next Portal L = {}",
                apex, portal_left, portal.left
            );

            // Start with updating the right vertex
            // Don't update if we're outside the funnel
            if Funnel::cross_product_magnitude_2d(apex, portal_right, portal.right) <= 0.0 {
                if apex == portal_right
                    || Funnel::cross_product_magnitude_2d(apex, portal_left, portal.right) > 0.0
                {
                    // No crossing, we can go to the next portal.
                    portal_right = portal.right;
                    right_index = i;
                    println!("Advancing Right Portal");
                } else {
                    // If we crossed the left portal, we found a point
                    points.push(portal_left);
                    apex = portal_left;
                    right_index = left_index;
                    i = left_index;
                    println!("Right Crossed Left: {} is now apex", portal_left);
                }
            }

            // Now update the left vertex
            // Don't update if we're outside the funnel
            if Funnel::cross_product_magnitude_2d(apex, portal_left, portal.left) >= 0.0 {
                // If we crossed the left portal, we found a point
                if apex == portal_left
                    || Funnel::cross_product_magnitude_2d(apex, portal_right, portal.left) < 0.0
                {
                    portal_left = portal.left;
                    left_index = i;
                    println!("Advancing Left Portal")
                } else {
                    points.push(portal_right);
                    apex = portal_right;
                    left_index = right_index;
                    i = right_index;
                    println!("Left Crossed Right: {} is now apex", portal_right);
                }
            }

            i += 1;
        }

        points.push(self.end);

        points
    }

    // Cross Product Magniture
    // In the "right-handed" coordinate system, if the result is 0, the points are collinear;
    // if it is positive, the three points constitute a positive angle of rotation around p 1 from p 2 to p 3,
    // otherwise a negative angle. From another point of view, the sign of P whether p 3 lies to the left or to the right of line p1, p2.
    pub fn cross_product_magnitude_2d(apex: Vec2, left: Vec2, right: Vec2) -> f32 {
        let a = left - apex;
        let b = right - apex;
        b.x * a.y - a.x * b.y
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
        let funnel = Funnel::from_path(
            Vec2::zero(),
            Vec2::zero(),
            vec![(0, 0), (0, 1), (0, 2), (1, 3), (2, 4), (2, 5)],
            32.0,
            320.0,
            320.0,
        );

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
    fn test_string_pull_simple() {
        // [1]|[2]
        // [x]|[3]
        let funnel = Funnel::from_path(
            Vec2::new(1.5 * 32.0, 1.5 * 32.0),
            Vec2::new(2.5 * 32.0, 16.0),
            vec![(0, 1), (1, 1), (1, 0)],
            32.0,
            320.0,
            320.0,
        );

        // First lets make sure we generate the right portals
        assert_eq!(
            vec![
                Portal {
                    left: Vec2::new(32.0, 32.0),
                    right: Vec2::new(32.0, 64.0)
                },
                Portal {
                    left: Vec2::new(32.0, 32.0),
                    right: Vec2::new(64.0, 32.0)
                }
            ],
            funnel.portals
        );

        let expected: Vec<Vec2> = vec![
            Vec2::new(48.0, 48.0),
            Vec2::new(32.0, 32.0),
            Vec2::new(80.0, 16.0),
        ];
        assert_eq!(expected, funnel.string_pull());
    }

    #[test]
    fn test_string_pull_dog_leg() {
        //    [6]
        //    [5]
        //       [4]
        //    [3]
        //    [2]
        // [1]
        let funnel = Funnel::from_path(
            Vec2::zero(),
            Vec2::new(48.0, 32.0 * 6.0),
            vec![(0, 0), (1, 1), (1, 2), (2, 3), (1, 4), (1, 5)],
            32.0,
            320.0,
            320.0,
        );

        let expected: Vec<Vec2> = vec![
            Vec2::zero(),
            Vec2::new(32.0, 64.0),
            Vec2::new(48.0, 112.0),
            Vec2::new(48.0, 192.0),
        ];
        assert_eq!(expected, funnel.string_pull());
    }

    #[test]
    fn test_cross_product_angle() {
        // |
        // |   l
        // |   r
        // |o_________
        let magnitude = Funnel::cross_product_magnitude_2d(
            Vec2::zero(),
            Vec2::new(3.0, 5.0),
            Vec2::new(3.0, 2.0),
        );

        // Positive angle
        assert_eq!(9.0, magnitude);

        // |
        // |   r
        // |   l
        // |o_________
        let magnitude = Funnel::cross_product_magnitude_2d(
            Vec2::zero(),
            Vec2::new(3.0, 2.0),
            Vec2::new(3.0, 5.0),
        );

        // Negative because right crossed left
        assert_eq!(-9.0, magnitude);
    }
}
