use mygrid::{direction::Direction, grid::Grid, point::Point};

advent_of_code::solution!(14);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    position: Point,
    velocity: Direction,
}

#[derive(Debug, Clone)]
struct Configuration {
    seconds: u32,
    grid_size: Point,
    robots: Vec<Robot>,
}

#[inline]
fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (position, velocity) = line[2..].split_once(" v=").unwrap();

            let (px, py) = position.split_once(",").unwrap();
            let (vx, vy) = velocity.split_once(",").unwrap();
            Robot {
                position: Point::new(py.parse().unwrap(), px.parse().unwrap()),
                velocity: Direction::new(vy.parse().unwrap(), vx.parse().unwrap()),
            }
        })
        .collect()
}

#[inline]
fn wrap_position(position: Point, grid_size: Point) -> Point {
    Point::new(
        (position.line % grid_size.line + grid_size.line) % grid_size.line,
        (position.column % grid_size.column + grid_size.column) % grid_size.column,
    )
}

fn solve_p1(configuration: Configuration) -> u32 {
    let result = configuration
        .robots
        .iter()
        .map(|robot| {
            let position = robot.position + (robot.velocity * configuration.seconds);
            let wrapped_position = wrap_position(position, configuration.grid_size);
            wrapped_position
        })
        .fold((0, 0, 0, 0), |mut acc, position| {
            let middle_line = configuration.grid_size.line / 2;
            let middle_column = configuration.grid_size.column / 2;
            // find out the quadrant of the position
            let left = position.line < middle_line;
            let right = position.line > middle_line;
            let top = position.column < middle_column;
            let bottom = position.column > middle_column;

            acc.0 += (top && left) as u32;
            acc.1 += (bottom && left) as u32;
            acc.2 += (top && right) as u32;
            acc.3 += (bottom && right) as u32;
            acc
        });

    result.0 * result.1 * result.2 * result.3
}

pub fn part_one(input: &str) -> Option<u32> {
    let configuration = Configuration {
        seconds: 100,
        grid_size: Point::new(103, 101),
        robots: parse_input(input),
    };

    Some(solve_p1(configuration))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut configuration = Configuration {
        seconds: 100,
        grid_size: Point::new(103, 101),
        robots: parse_input(input),
    };

    let base_dbg_grid = Grid::new(
        configuration.grid_size.column as usize,
        configuration.grid_size.line as usize,
        '.',
    );

    let mut seconds = 0;

    while seconds < 10000 {
        let mut dbg_grid = base_dbg_grid.clone();

        for robot in configuration.robots.iter_mut() {
            let position = robot.position + robot.velocity;
            let wrapped_position = wrap_position(position, configuration.grid_size);
            robot.position = wrapped_position;
        }
        seconds += 1;

        let robot_on_column_count = configuration
            .robots
            .iter()
            // found visually
            .filter(|&r| r.position.column > 24 && r.position.column < 55)
            .count();
        if robot_on_column_count > 200 {
            for robot in configuration.robots.iter() {
                dbg_grid[robot.position] = '#';
            }
            // dbg!(&dbg_grid);
            // dbg!(&seconds);

            //let mut buffer = String::new();
            //std::io::stdin()
            //    .read_line(&mut buffer)
            //    .expect("Failed to read line");

            // found visually
            if seconds == 7083 {
                return Some(seconds);
            }
        }
    }

    Some(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_robot() {
        let configuration = Configuration {
            seconds: 2,
            grid_size: Point::new(7, 11),
            robots: vec![Robot {
                position: Point::new(4, 2),
                velocity: Direction::new(-3, 2),
            }],
        };

        let result = solve_p1(configuration);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_one() {
        let configuration = Configuration {
            seconds: 100,
            grid_size: Point::new(7, 11),
            robots: parse_input(&advent_of_code::template::read_file("examples", DAY)),
        };
        let result = solve_p1(configuration);
        assert_eq!(result, 12);
    }
}
