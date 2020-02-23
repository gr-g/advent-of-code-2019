use std::cmp::Ordering;
use std::collections::HashSet;
use advent_of_code_2019::intcode::IntCode;
use advent_of_code_2019::grid::Direction::{self, *};
use advent_of_code_2019::graph::Backtracking;

#[derive(Clone, Debug)]
enum DroidAction {
    Move(Direction),
    Drop(String),
}

#[derive(Clone, Debug)]
enum Goal {
    Explore,
    GoToCheckpoint,
    VerifyIdentity,
}

#[derive(Clone)]
struct Droid {
    computer: IntCode,
    room: String,
    doors: Vec<Direction>,
    last_move: Option<Direction>,
    trap_objects: HashSet<String>,
    objects_carried: HashSet<String>,
    checkpoint_move: Option<Direction>, // the move to go through the checkpoint
    checkpoint_status: Ordering, // whether we are too heavy or too light
    goal: Goal,
}

impl Droid {
    fn new( program: &[i64] ) -> Droid {
        let mut d = Droid {
            computer: IntCode::new(program),
            room: String::new(),
            doors: Vec::new(),
            last_move: None,
            trap_objects: HashSet::new(),
            objects_carried: HashSet::new(),
            checkpoint_move: None,
            checkpoint_status: Ordering::Less,
            goal: Goal::Explore,
        };
        d.computer.display = false;
        
        // program the droid to avoid trap objects
        d.trap_objects.insert("molten lava".to_string());
        d.trap_objects.insert("infinite loop".to_string());
        d.trap_objects.insert("giant electromagnet".to_string());
        d.trap_objects.insert("escape pod".to_string());
        d.trap_objects.insert("photons".to_string());
        
        let room = d.computer.run_ascii_command("").0;
        d.read_room(room);
        d
    }
    
    fn read_room( &mut self, room: String ) {
        self.room = room;
        self.doors.clear();
        for l in self.room.lines() {
            if l.starts_with("- ") {
                match &l[2..] {
                    "north" => self.doors.push(Up),
                    "east" => self.doors.push(Right),
                    "south" => self.doors.push(Down),
                    "west" => self.doors.push(Left),
                    obj => {
                        if !self.trap_objects.contains(obj) {
                            self.computer.run_ascii_command(&format!("take {}", obj));
                            self.objects_carried.insert(obj.to_string());
                            println!("Found {} in {}!", obj, self.room.lines().nth(3).unwrap());
                        }
                    },
                }
            }
        }
    }
    
    fn try_checkpoint( &mut self ) {
        self.room = match self.checkpoint_move.unwrap() {
            Up => { self.computer.run_ascii_command("north").0 },
            Right => { self.computer.run_ascii_command("east").0 },
            Down => { self.computer.run_ascii_command("south").0 },
            Left => { self.computer.run_ascii_command("west").0 },
        };
        if self.room.contains("lighter than the detected value") {
            self.checkpoint_status = Ordering::Greater;
        } else if self.room.contains("heavier than the detected value") {
            self.checkpoint_status = Ordering::Less;
        } else {
            self.checkpoint_status = Ordering::Equal;
        }
    }
}

impl Backtracking for Droid {
    type Action = DroidAction;
    
    fn list_actions( &self ) -> Vec<DroidAction> {
        let mut v = Vec::new();
        match self.goal {
            Goal::Explore | Goal::GoToCheckpoint => {
                for d in self.doors.iter() {
                    if self.last_move != Some(d.reverse()) {
                        v.push(DroidAction::Move(*d));
                    }
                }
            },
            Goal::VerifyIdentity => {
                match self.checkpoint_status {
                    Ordering::Greater => {
                        // we are too heavy
                        for obj in self.objects_carried.iter() {
                            v.push(DroidAction::Drop(obj.clone()));
                        }
                    },
                    _ => {
                        // we are too light, backtrack
                    },
                }
            },
        }
        v
    }

    fn try_action( &mut self, action: &DroidAction ) -> bool {
        match action {
            DroidAction::Move(d) => {
                let room = match d {
                    Up => { self.computer.run_ascii_command("north").0 },
                    Right => { self.computer.run_ascii_command("east").0 },
                    Down => { self.computer.run_ascii_command("south").0 },
                    Left => { self.computer.run_ascii_command("west").0 },
                };
                if room.contains("ejected back to the checkpoint") {
                    self.checkpoint_move = Some(*d);
                    return false;
                }
                self.last_move = Some(*d);
                self.read_room(room);
            },
            DroidAction::Drop(obj) => {
                self.objects_carried.remove(obj);
                self.computer.run_ascii_command(&format!("drop {}", obj)).0;
                self.try_checkpoint();
            },
        }
        true
    }

    fn backtrack( &mut self, action: &DroidAction ) {
        let room = match action {
            DroidAction::Move(Up) => { self.computer.run_ascii_command("south").0 },
            DroidAction::Move(Right) => { self.computer.run_ascii_command("west").0 },
            DroidAction::Move(Down) => { self.computer.run_ascii_command("north").0 },
            DroidAction::Move(Left) => { self.computer.run_ascii_command("east").0 },
            DroidAction::Drop(obj) => {
                self.objects_carried.insert(obj.clone());
                self.computer.run_ascii_command(&format!("take {}", obj)).0
            },
                
        };
        self.last_move = None;
        self.read_room(room);
    }

    fn is_solution( &self ) -> bool {
        match self.goal {
            Goal::Explore => false,
            Goal::GoToCheckpoint => {
                self.room.contains("== Security Checkpoint ==")
            },
            Goal::VerifyIdentity => {
                self.checkpoint_status == Ordering::Equal
            }
        }
    }
}

fn solve( input: &str ) -> i64 {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    
    // Explore the environment and pick up objects.
    let mut droid = Droid::new(&program);
    droid.explore();
    
    // Save the current state.
    let mut saved_state = droid.clone();
    
    // Go to the checkpoint.
    droid.goal = Goal::GoToCheckpoint;
    let winning_moves = droid.explore().expect("checkpoint not found");
    
    // Try to drop objects until we pass the checkpoint.
    // Backtrack if we get too light.
    droid.goal = Goal::VerifyIdentity;
    droid.try_checkpoint();
    let winning_drops = droid.explore().expect("solution not found");
    
    let password = droid.room.split(' ').find_map(|w| w.parse::<i64>().ok()).unwrap();
    
    // Replay the winning moves with display on.
    print!("{}", saved_state.room);
    saved_state.computer.display = true;
    for a in winning_moves.iter().chain(winning_drops.iter()) {
        saved_state.try_action(a);
    }
    
    password
}

fn main() {
    let input = std::fs::read_to_string("input/25.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
