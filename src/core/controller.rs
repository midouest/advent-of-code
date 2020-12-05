use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use cursive::Cursive;

pub trait Controller {
    fn show(&mut self, c: Rc<RefCell<Cursive>>);
    fn process_events(&mut self, c: Rc<RefCell<Cursive>>) -> bool;
}

pub fn run<C, E>(controller: C, c: Rc<RefCell<Cursive>>)
where
    C: Controller,
    E: 'static,
{
    EventLoop::<C, E>::new(controller, c).run()
}

pub fn emit<E: 'static>(c: &mut Cursive, event: E) {
    c.with_user_data(|events: &mut Vec<E>| {
        events.push(event);
    });
}

pub fn take_events<E: 'static>(c: &mut Cursive) -> Vec<E> {
    c.take_user_data::<Vec<E>>().unwrap()
}

pub struct EventLoop<C, E>
where
    C: Controller,
    E: 'static,
{
    controller: C,
    cursive: Rc<RefCell<Cursive>>,
    _phantom: PhantomData<E>,
}

impl<C, E> EventLoop<C, E>
where
    C: Controller,
{
    pub fn new(controller: C, curs: Rc<RefCell<Cursive>>) -> Self {
        Self {
            controller,
            cursive: curs,
            _phantom: PhantomData,
        }
    }

    pub fn run(&mut self) {
        self.controller.show(Rc::clone(&self.cursive));

        loop {
            let events: Vec<E> = Vec::new();
            self.cursive.borrow_mut().set_user_data(events);

            let mut c = self.cursive.borrow_mut();
            if !c.is_running() {
                break;
            }
            c.refresh();
            c.step();
            drop(c); // borrow_mut must be dropped before process_events

            let running = self.controller.process_events(Rc::clone(&self.cursive));
            if !running {
                break;
            }
        }
    }
}
