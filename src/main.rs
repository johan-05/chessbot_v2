use std::{

    thread::{self,JoinHandle},

    sync::{
        mpsc::{self,Receiver,Sender}, 
        Arc,
        Mutex,
    },
};


#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Board {
    kings: u64,
    queens: u64, // u64 bitmaps of all piece-types
    rooks: u64,
    bishops: u64,
    knights: u64,
    pawns: u64,
    whites: u64, // colors are tracked with color-bitmasks
    blacks: u64,
    castelable_pieces: u64, // bitmap of rooks and kings that can castle
    fifty_rule: u8,         // number of moves without capture of pawn push
    en_passant_index: u8,   // index of piece susceptible to en passant
    eval: i16,              // evaluation of the position
}

#[allow(dead_code)]
impl Board {
    const fn new() -> Board {
        Board {
            kings: 576460752303423496,
            queens: 1152921504606846992,
            rooks: 9295429630892703873,
            bishops: 2594073385365405732,
            knights: 4755801206503243842,
            pawns: 71776119061282560,
            whites: 65535,
            blacks: 18446462598732840960,
            castelable_pieces: 9871890383196127369,
            fifty_rule: 0,
            en_passant_index: 0,
            eval: 0,
        }
    }
}

type Eval = i32;

struct GameState {
    board: Board,
    move_tree: MoveTree,
    eval: Eval,
}

impl GameState {
    fn init() -> GameState {
        let board = Board::new();
        let move_tree = MoveTree::new_empty();
        return GameState {
            board: board,
            move_tree: move_tree,
            eval: 0,
        };
    }

    fn placeholder() -> GameState {
        // TODO make placeholder an unallocated GameState
        let board = Board::new();
        let move_tree = MoveTree::new_empty();
        return GameState {
            board: board,
            move_tree: move_tree,
            eval: 0,
        };
    }

    fn advance_gamestate(self) -> GameState {
        let (best_move, future_tree, eval) = self.move_tree.advance_tree();
        return GameState {
            board: best_move,
            move_tree: future_tree,
            eval: eval
        };
    }
}

#[allow(dead_code)]
struct MoveTree {
    workers: Vec<Worker>,
}

impl MoveTree {
    fn new_empty() -> MoveTree {
        return MoveTree {
            workers: Vec::with_capacity(NTHREADS),
        };
    }

    fn advance_tree(self) -> (Board, MoveTree, Eval) {
        let best_worker = self
            .workers
            .into_iter()
            .max_by(|a, b| {
                let a = a.best_move.lock().unwrap();
                let b = b.best_move.lock().unwrap();
                (a.eval).cmp(&b.eval)
            })
            .expect("no moves computed yet");

        let game_state = best_worker.handle.join().unwrap();
    }
}

#[allow(dead_code)]
struct Worker {
    handle: JoinHandle<()>,
    tx: Sender<GameState>,
    best_move: Mutex<GameState>,
}

impl Worker {
    fn new() -> Worker {
        let (tx, rx) = mpsc::channel::<GameState>();
        let best_move = Arc::new(Mutex::new(GameState::placeholder()));
        let mutex_clone = Arc::clone(&best_move);
        let handle = thread::spawn(move || {thread_search(rx, mutex_clone)});
    }
}

const NTHREADS: usize = 4;

static THREAD_POOL: Vec<Worker> = (0..NTHREADS)
    .into_iter()
    .map(|_i| Worker::new())
    .collect();


#[allow(dead_code, unused_variables)]
fn main() {
    println!("Chessbot V2");

    let game = GameState::init();
}

fn thread_search(rx: Receiver<GameState>, best_move: Mutex<GameState>) {
    unimplemented!("amogus");
}
