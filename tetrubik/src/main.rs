use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Colour {
    Red,
    Yellow,
    Blue,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FaceState {
    one: Colour,
    two: Colour,
    three: Colour,
    four: Colour,
    five: Colour,
    six: Colour,
}

impl FaceState {
    fn new(
        one: Colour,
        two: Colour,
        three: Colour,
        four: Colour,
        five: Colour,
        six: Colour,
    ) -> FaceState {
        FaceState {
            one: one,
            two: two,
            three: three,
            four: four,
            five: five,
            six: six,
        }
    }

    fn from_colour(colour: Colour) -> FaceState {
        FaceState::new(colour, colour, colour, colour, colour, colour)
    }
}

impl fmt::Display for FaceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
            self.one, self.two, self.three, self.four, self.five, self.six
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    front: FaceState,
    left: FaceState,
    right: FaceState,
    base: FaceState,
}

impl State {
    fn new() -> State {
        State {
            front: FaceState::from_colour(Colour::Red),
            left: FaceState::from_colour(Colour::Yellow),
            right: FaceState::from_colour(Colour::Blue),
            base: FaceState::from_colour(Colour::Green),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\n  front: {},\n  left: {},\n  right: {},\n  base: {}}}",
            self.front, self.left, self.right, self.base
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rotation {
    Clockwise,
    AntiClockwise,
}

impl Rotation {
    fn new_vec() -> Vec<Rotation> {
        let mut res: Vec<Rotation> = Vec::new();
        res.push(Rotation::Clockwise);
        res.push(Rotation::AntiClockwise);
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Corner {
    FrontLeftBase,
    FrontRightBase,
    FrontLeftRight,
    BaseLeftRight,
}

impl Corner {
    fn new_vec() -> Vec<Corner> {
        let mut res: Vec<Corner> = Vec::new();
        res.push(Corner::FrontLeftBase);
        res.push(Corner::FrontRightBase);
        res.push(Corner::FrontLeftRight);
        res.push(Corner::BaseLeftRight);
        res
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Node {
    parent: Option<u32>,
    id: u32,
    corner: Corner,
    rotation: Rotation,
    state: State,
    count: u32,
}

struct Head {
    state: State,
    path: Vec<(Corner, Rotation)>,
}

fn main() {
    let complete_state: State = State::new();
    println!("Complete state is: {}", complete_state);
    let scrambled: State = State {
        front: FaceState::new(
            Colour::Red,
            Colour::Yellow,
            Colour::Red,
            Colour::Green,
            Colour::Red,
            Colour::Red,
        ),
        left: FaceState::new(
            Colour::Yellow,
            Colour::Blue,
            Colour::Yellow,
            Colour::Yellow,
            Colour::Yellow,
            Colour::Green,
        ),
        right: FaceState::new(
            Colour::Blue,
            Colour::Blue,
            Colour::Blue,
            Colour::Green,
            Colour::Blue,
            Colour::Blue,
        ),
        base: FaceState::new(
            Colour::Green,
            Colour::Red,
            Colour::Green,
            Colour::Red,
            Colour::Green,
            Colour::Yellow,
        ),
    };

    println!("Scrambled state is: {}", scrambled);

    println!(" solved {:?} ", solve(scrambled));
}

/*
fn scramble(state: State) -> State {
    let mut s: State = state;
    let mut rng = rand::thread_rng();

    for _ in 1..200 {
        let mut corners = Corner::new_vec();
        corners.shuffle(&mut rng);
        let mut rotations = Rotation::new_vec();
        rotations.shuffle(&mut rng);
        s = turn(s, corners[0], rotations[0]);
    }
    s
}
*/

fn turn(state: State, corner: Corner, rotation: Rotation) -> State {
    let f = state.front;
    let l = state.left;
    let r = state.right;
    let b = state.base;
    let new_state = match (corner, rotation) {
        (Corner::FrontLeftBase, Rotation::Clockwise) => State {
            front: FaceState::new(f.one, f.two, f.three, l.two, l.three, l.four),
            left: FaceState::new(l.one, b.two, b.three, b.four, l.five, l.six),
            right: r,
            base: FaceState::new(b.one, f.four, f.five, f.six, b.five, b.six),
        },

        (Corner::FrontRightBase, Rotation::Clockwise) => State {
            front: FaceState::new(f.one, b.four, b.five, b.six, f.five, f.six),
            left: l,
            right: FaceState::new(r.one, r.two, r.three, f.two, f.three, f.four),
            base: FaceState::new(b.one, b.two, b.three, r.four, r.five, r.six),
        },

        (Corner::FrontLeftRight, Rotation::Clockwise) => State {
            front: FaceState::new(r.one, r.two, f.three, f.four, f.five, r.six),
            left: FaceState::new(f.one, f.two, l.three, l.four, l.five, f.six),
            right: FaceState::new(l.one, l.two, r.three, r.four, r.five, l.six),
            base: b,
        },

        (Corner::BaseLeftRight, Rotation::Clockwise) => State {
            front: f,
            left: FaceState::new(l.one, l.two, l.three, r.two, r.three, r.four),
            right: FaceState::new(r.one, b.six, b.one, b.two, r.five, r.six),
            base: FaceState::new(l.five, l.six, b.three, b.four, b.five, l.four),
        },

        (Corner::FrontLeftBase, Rotation::AntiClockwise) => State {
            front: FaceState::new(f.one, f.two, f.three, b.two, b.three, b.four),
            left: FaceState::new(l.one, f.four, f.five, f.six, l.five, l.six),
            right: r,
            base: FaceState::new(b.one, l.two, l.three, l.four, b.five, b.six),
        },

        (Corner::FrontRightBase, Rotation::AntiClockwise) => State {
            front: FaceState::new(f.one, r.four, r.five, r.six, f.five, f.six),
            left: l,
            right: FaceState::new(r.one, r.two, r.three, b.four, b.five, b.six),
            base: FaceState::new(b.one, b.two, b.three, f.two, f.three, f.four),
        },

        (Corner::FrontLeftRight, Rotation::AntiClockwise) => State {
            front: FaceState::new(l.one, l.two, f.three, f.four, f.five, l.six),
            left: FaceState::new(r.one, r.two, l.three, l.four, l.five, r.six),
            right: FaceState::new(f.one, f.two, r.three, r.four, r.five, f.six),
            base: b,
        },

        (Corner::BaseLeftRight, Rotation::AntiClockwise) => State {
            front: f,
            left: FaceState::new(l.one, l.two, l.three, b.six, b.one, b.two),
            right: FaceState::new(r.one, l.four, l.five, l.six, r.five, r.six),
            base: FaceState::new(r.three, r.four, b.three, b.four, b.five, r.two),
        },
    };
    // println!("corner {:?} rotation {:?}", corner, rotation);
    // colour_correct(new_state);
    new_state
}

fn solve(state: State) -> Vec<(Corner, Rotation)> {
    colour_correct(state);
    // println!("{} solving state", state);
    let complete_state: State = State::new();
    let corners: Vec<Corner> = Corner::new_vec();
    let rotations: Vec<Rotation> = Rotation::new_vec();

    let mut heads: VecDeque<Head> = VecDeque::new();
    let mut counts: HashMap<State, usize> = HashMap::new();
    let mut complete: Option<Vec<(Corner, Rotation)>> = None;

    let root: Head = Head {
        state: state,
        path: Vec::new(),
    };

    heads.insert(0, root);

    loop {
        /*
        println!(
            "length of counts {}, length of heads {}",
            counts.len(),
            heads.len()
        );
        */

        let head: Head = match heads.pop_front() {
            Some(x) => x,
            None => break,
        };

        match counts.get(&head.state) {
            Some(count) if *count <= head.path.len() => continue,
            _ => counts.insert(head.state, head.path.len()),
        };

        match &complete {
            Some(y) if y.len() <= head.path.len() => continue,
            _ if head.state == complete_state => {
                complete = Some(head.path.clone());
                println!("Found solution {:?}", complete);
            }
            _ => (),
        }
        for corner in &corners {
            for rotation in &rotations {
                let new_state = turn(head.state, *corner, *rotation);
                let mut new_path: Vec<(Corner, Rotation)> = head.path.clone();
                new_path.push((*corner, *rotation));
                let new_head: Head = Head {
                    state: new_state,
                    path: new_path,
                };
                heads.push_back(new_head);
            }
        }
    }
    match complete {
        Some(c) => c,
        None => panic!("Shouldn't happen"),
    }
}

fn colour_correct(state: State) {
    let mut counts: HashMap<Colour, i32> = HashMap::new();

    counts.insert(Colour::Red, 0);
    counts.insert(Colour::Yellow, 0);
    counts.insert(Colour::Blue, 0);
    counts.insert(Colour::Green, 0);

    for f in vec![state.front, state.left, state.right, state.base] {
        for c in vec![f.one, f.two, f.three, f.four, f.five, f.six] {
            let n = counts[&c];
            counts.insert(c, n + 1);
        }
    }

    for val in counts.values() {
        assert_eq!(*val, 6);
    }

    // Corner::FrontLeftBase
    let colours: Vec<Vec<Colour>> = vec![
        vec![Colour::Red, Colour::Green, Colour::Yellow],
        vec![Colour::Green, Colour::Yellow, Colour::Red],
        vec![Colour::Yellow, Colour::Red, Colour::Green],
    ];
    assert!(colours.contains(&vec![state.front.five, state.base.three, state.left.three]));

    // Corner::FrontLeftRight
    let colours: Vec<Vec<Colour>> = vec![
        vec![Colour::Red, Colour::Yellow, Colour::Blue],
        vec![Colour::Yellow, Colour::Blue, Colour::Red],
        vec![Colour::Blue, Colour::Red, Colour::Yellow],
    ];
    assert!(colours.contains(&vec![state.front.one, state.left.one, state.right.one]));

    // Corner::FrontRightBase
    let colours: Vec<Vec<Colour>> = vec![
        vec![Colour::Red, Colour::Blue, Colour::Green],
        vec![Colour::Blue, Colour::Green, Colour::Red],
        vec![Colour::Green, Colour::Red, Colour::Blue],
    ];
    assert!(colours.contains(&vec![state.front.three, state.right.five, state.base.five]));

    //BaseLeftRight,
    let colours: Vec<Vec<Colour>> = vec![
        vec![Colour::Green, Colour::Blue, Colour::Yellow],
        vec![Colour::Blue, Colour::Yellow, Colour::Green],
        vec![Colour::Yellow, Colour::Green, Colour::Blue],
    ];
    assert!(colours.contains(&vec![state.left.five, state.base.one, state.right.three]));

    let mut pairs: HashSet<(Colour, Colour)> = HashSet::new();
    pairs.insert((Colour::Red, Colour::Yellow));
    pairs.insert((Colour::Red, Colour::Blue));
    pairs.insert((Colour::Red, Colour::Green));
    pairs.insert((Colour::Yellow, Colour::Blue));
    pairs.insert((Colour::Yellow, Colour::Green));
    pairs.insert((Colour::Blue, Colour::Green));

    for (c1, c2) in &[
        (state.front.two, state.right.six),
        (state.front.four, state.base.four),
        (state.front.six, state.left.two),
        (state.base.two, state.left.four),
        (state.base.six, state.right.four),
        (state.left.six, state.right.two),
    ] {
        match (pairs.contains(&(*c1, *c2)), pairs.contains(&(*c2, *c1))) {
            (true, true) => panic!("problem"),
            (false, false) => panic!("problem"),
            (true, false) => pairs.remove(&(*c1, *c2)),
            (false, true) => pairs.remove(&(*c2, *c1)),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{colour_correct, solve, turn, Colour, Corner, Rotation, State};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn turns() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            for rotation in vec![Rotation::Clockwise, Rotation::AntiClockwise] {
                let complete = State::new();
                let state: State = turn(complete, corner, rotation);
                colour_correct(state);
            }
        }
    }

    #[test]
    fn clockwise_turns() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            let mut state = State::new();
            for rotation in vec![
                Rotation::Clockwise,
                Rotation::Clockwise,
                Rotation::Clockwise,
            ] {
                state = turn(state, corner, rotation);
                colour_correct(state);
            }
            assert_eq!(state, State::new());
        }
    }

    #[test]
    fn anticlockwise_turns() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            let mut state = State::new();
            for rotation in vec![
                Rotation::AntiClockwise,
                Rotation::AntiClockwise,
                Rotation::AntiClockwise,
            ] {
                state = turn(state, corner, rotation);
                colour_correct(state);
            }
            assert_eq!(state, State::new());
        }
    }

    #[test]
    fn back_forth_turns() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            let mut state = State::new();
            for rotation in vec![Rotation::AntiClockwise, Rotation::Clockwise] {
                state = turn(state, corner, rotation);
                colour_correct(state);
            }
            assert_eq!(state, State::new());
        }
    }

    #[test]
    fn forth_back_turns() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            let mut state = State::new();
            for rotation in vec![Rotation::Clockwise, Rotation::AntiClockwise] {
                state = turn(state, corner, rotation);
                colour_correct(state);
            }
            assert_eq!(state, State::new());
        }
    }

    #[test]
    fn complete() {
        println!("starting complete test");
        let moves: Vec<(Corner, Rotation)> = solve(State::new());
        let expected: Vec<(Corner, Rotation)> = Vec::new();
        assert_eq!(moves, expected);
    }

    #[test]
    fn nearly_complete() {
        for corner in vec![
            Corner::FrontLeftBase,
            Corner::FrontRightBase,
            Corner::FrontLeftRight,
            Corner::BaseLeftRight,
        ] {
            let state: State = turn(State::new(), corner, Rotation::Clockwise);
            colour_correct(state);
            let moves: Vec<(Corner, Rotation)> = solve(state);
            let expected: Vec<(Corner, Rotation)> = vec![(corner, Rotation::AntiClockwise)];
            assert_eq!(moves, expected);
        }
    }
}
