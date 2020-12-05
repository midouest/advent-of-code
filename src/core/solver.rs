use std::{cell::RefCell, rc::Rc};

use cursive::{
    event::Event,
    event::EventResult,
    event::{Callback, Key},
    views::Canvas,
    views::{Dialog, TextView},
    Cursive, Printer,
};

use super::controller::{emit, run, take_events, Controller};

pub trait Solver {
    fn is_done(&self) -> bool;
    fn solution(&self) -> Option<i64>;
    fn step(&mut self);
    fn draw(&self, printer: &Printer);

    fn solve(&mut self) -> Option<i64> {
        while !self.is_done() {
            self.step();
        }
        self.solution()
    }

    fn with_done<T, F>(&self, f: F) -> Option<T>
    where
        F: Fn() -> T,
    {
        if self.is_done() {
            Some(f())
        } else {
            None
        }
    }

    fn with_done_some<T>(&self, t: T) -> Option<T> {
        if self.is_done() {
            Some(t)
        } else {
            None
        }
    }
}

pub enum SolverEvent {
    Stop,
}

pub struct SolverController<S: Solver> {
    is_running: bool,
    is_solved: bool,
    state: Rc<RefCell<S>>,
}

impl<S> SolverController<S>
where
    S: Solver + 'static,
{
    pub fn new(solver: S) -> Self {
        Self {
            is_running: true,
            is_solved: false,
            state: Rc::new(RefCell::new(solver)),
        }
    }

    pub fn run(self, c: Rc<RefCell<Cursive>>) {
        run::<SolverController<S>, SolverEvent>(self, c)
    }
}

impl<S> Controller for SolverController<S>
where
    S: Solver + 'static,
{
    fn show(&mut self, c: Rc<RefCell<Cursive>>) {
        let mut siv = c.borrow_mut();

        let canvas = Canvas::new(Rc::clone(&self.state))
            .with_required_size(|_, constraints| constraints)
            .with_draw(|s, printer| {
                s.borrow().draw(printer);
            })
            .with_on_event(|_, event| match event {
                Event::Key(Key::Esc) => EventResult::Consumed(Some(Callback::from_fn(|c| {
                    emit(c, SolverEvent::Stop);
                }))),
                _ => EventResult::Ignored,
            });

        siv.add_fullscreen_layer(canvas);
    }

    fn process_events(&mut self, c: Rc<RefCell<Cursive>>) -> bool {
        let events = take_events(&mut c.borrow_mut());
        for event in events {
            match event {
                SolverEvent::Stop => self.is_running = false,
            }
        }

        if self.is_running {
            let mut state = self.state.borrow_mut();
            if state.is_done() {
                if !self.is_solved {
                    let solution = state.solution().unwrap().to_string();
                    c.borrow_mut().add_layer(
                        Dialog::new()
                            .title("Solution")
                            .content(TextView::new(solution))
                            .dismiss_button("Ok"),
                    );
                    self.is_solved = true;
                }
            } else {
                state.step();
            }
        } else {
            c.borrow_mut().pop_layer();
        }

        return self.is_running;
    }
}
