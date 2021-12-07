use std::collections::HashMap;

//             X    Y
type Point = (i32, i32);
//              X1    Y1   X2    Y2
//             (0.0, 0.1) (1.0, 1.1)
type Section = (Point, Point);

struct LineTraverser {
    section: Section,
    versor: Point,
    position: Option<Point>,
}

impl LineTraverser {
    fn new(section: Section) -> LineTraverser {
        let vector = (section.1 .0 - section.0 .0, section.1 .1 - section.0 .1);
        let versor_x = if vector.0 != 0 {
            vector.0 / vector.0.abs()
        } else {
            0
        };
        let versor_y = if vector.1 != 0 {
            vector.1 / vector.1.abs()
        } else {
            0
        };
        let versor = (versor_x, versor_y);
        let position = Some(section.0);

        LineTraverser {
            section,
            versor,
            position,
        }
    }
}

impl Iterator for LineTraverser {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.position {
            let old_pos = self.position;
            if position != self.section.1 {
                self.position = Some((position.0 + self.versor.0, position.1 + self.versor.1));
                return old_pos;
            }
            self.position = None;
            return old_pos;
        }
        None
    }
}

fn main() {
    let sections: Vec<Section> = std::fs::read_to_string("input.txt")
        .expect("File not found!")
        .lines()
        .map(|x| {
            let line = String::from(x);
            let points: Vec<&str> = line.split(" -> ").collect();

            let from: Vec<i32> = points[0]
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            let to: Vec<i32> = points[1]
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            ((from[0], from[1]), (to[0], to[1]))
        })
        .collect();

    let mut overlaps = HashMap::<Point, i32>::new();
    sections.iter().for_each(|v| {
        let traverser = LineTraverser::new(*v);
        for point in traverser {
            *overlaps.entry(point).or_insert(0) += 1;
        }
    });

    println!("{}", overlaps.values().cloned().filter(|x| *x > 1).count());
}
