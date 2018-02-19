use std::sync::mpsc::{Sender, Receiver};

use util::endpoint::Endpoint;
use grid::{DropletId, DropletInfo, Grid, GridView, Location};
use plan::plan::Placement;

#[derive(Debug)]
pub enum Action {
    AddDroplet {
        id: DropletId,
        location: Location,
    },
    RemoveDroplet {
        id: DropletId,
    },
    Mix {
        in0: DropletId,
        in1: DropletId,
        out: DropletId,
    },
    Split {
        inp: DropletId,
        out0: DropletId,
        out1: DropletId,
    },
    SetCollisionGroup {
        id: DropletId,
        cg: usize,
    },
    UpdateDroplet {
        old_id: DropletId,
        new_id: DropletId,
        // TODO take a closure here
    },
    MoveDroplet {
        id: DropletId,
        location: Location,
    },
    Lockstep {
        actions: Vec<Action>,
    },
    // TODO should be more general
    Ping {
        tx: Sender<()>,
    },
}

impl Action {
    #[allow(unused_variables)]
    pub fn translate(&mut self, placement: &Placement) {
        use self::Action::*;
        match *self {
            AddDroplet { id, ref mut location } => {
                *location = placement[location];
            }
            RemoveDroplet { id } => {}
            Mix { in0, in1, out } => {}
            Split { inp, out0, out1 } => {}
            SetCollisionGroup { id, cg } => {}
            UpdateDroplet { old_id, new_id } => {}
            MoveDroplet { id, ref mut location } => {
                *location = placement[location];
            }
            Lockstep { ref mut actions } => for a in actions {
                a.translate(placement);
            },
            Ping { ref tx } => {}
        }
    }
}

pub struct Executor {
    blocking: bool,
    gridview: GridView,
}

impl Executor {
    pub fn new(blocking: bool, grid: Grid) -> Self {
        Executor {
            blocking: blocking,
            gridview: GridView::new(grid),
        }
    }

    fn execute(&mut self, action: Action) {
        debug!("executing {:?}", action);
        use self::Action::*;
        match &action {
            &Ping { ref tx } => {
                tx.send(()).unwrap();
            }
            _ => {}
        }
        self.gridview.execute(&action);
    }

    pub fn run(&mut self, action_rx: Receiver<Action>, endpoint: Endpoint<Vec<DropletInfo>, ()>) {
        loop {
            // wait on the visualizer then reply
            if self.blocking {
                match endpoint.recv() {
                    Ok(()) => {},
                    Err(_) => return
                }
                endpoint.send(self.gridview.droplet_info(None)).unwrap();
            }

            // now do some stuff
            let action = match action_rx.recv() {
                Ok(action) => action,
                Err(_) => return
            };
            self.execute(action);
        }
    }
}