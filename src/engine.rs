use anyhow::Result;
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use crate::{data::Archive, game::options::Config, input::InputHelper, task::Signal};

pub struct State {
    pub arc: Archive,
    pub cfg: Config,
    pub input: Rc<RefCell<InputHelper>>,
}

pub struct GameEngine {
    task: Option<Pin<Box<dyn Future<Output = Result<()>>>>>,
}

impl GameEngine {
    pub fn new<T: Future<Output = Result<()>> + 'static>(
        arc: Archive,
        cfg: Config,
        input: Rc<RefCell<InputHelper>>,
        f: fn(State) -> T,
    ) -> Result<Self> {
        let state = State { arc, cfg, input };

        Ok(Self {
            task: Some(Box::pin(f(state))),
        })
    }

    pub fn step(&mut self, ctx: &mut Context<'static>, signal: &Signal) -> Result<Poll<()>> {
        match self.task.as_mut() {
            Some(task) => match task.as_mut().poll(ctx) {
                Poll::Pending => {
                    signal.wait();

                    Ok(Poll::Pending)
                }
                Poll::Ready(result) => {
                    // consume the completed task so it cannot be polled again
                    self.task = None;
                    Ok(Poll::Ready(result?))
                }
            },
            // already completed, avoid polling again
            None => Ok(Poll::Ready(())),
        }
    }
}
