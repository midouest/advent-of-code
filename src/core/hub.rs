use std::{cell::RefCell, rc::Rc};

use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

use super::{
    controller::{emit, take_events, Controller},
    puzzle::{PuzzlePart, PuzzleRegistry},
};

pub enum HubEvent {
    DaySelected(usize),
    PuzzleSelected(usize, PuzzlePart),
}

pub struct Hub {
    registry: PuzzleRegistry,
}

impl Hub {
    pub fn new(registry: PuzzleRegistry) -> Self {
        Self { registry }
    }
}

impl Controller for Hub {
    fn show(&mut self, c: Rc<RefCell<Cursive>>) {
        let view = build_hub_view(&self.registry);
        c.borrow_mut().add_layer(view);
    }

    fn process_events(&mut self, c: Rc<RefCell<Cursive>>) -> bool {
        let events = take_events(&mut c.borrow_mut());
        for event in events {
            match event {
                HubEvent::DaySelected(day) => {
                    if !self.registry.has(day) {
                        return true;
                    }

                    let view = build_part_view(day);
                    c.borrow_mut().add_layer(view);
                }
                HubEvent::PuzzleSelected(day, part) => {
                    let puzzle = self.registry.get(day).as_ref().unwrap();
                    if !puzzle.is_implemented(part) {
                        return true;
                    }

                    c.borrow_mut().pop_layer();
                    c.borrow_mut().pop_layer();
                    puzzle.run(part, Rc::clone(&c));
                    self.show(Rc::clone(&c));
                }
            }
        }
        return true;
    }
}

fn build_hub_view(registry: &PuzzleRegistry) -> Dialog {
    let select_view = registry
        .get_puzzles()
        .iter()
        .enumerate()
        .fold(SelectView::new(), |view, (day, puzzle)| {
            let title = puzzle.as_ref().map(|puzzle| puzzle.get_title());
            let label = format_puzzle(day, title);
            view.item(label, day)
        })
        .on_submit(|c, day| {
            emit(c, HubEvent::DaySelected(*day));
        });

    Dialog::new()
        .title("Select Puzzle")
        .content(select_view)
        .button("Quit", |c| c.quit())
}

fn format_puzzle(day: usize, title: Option<String>) -> String {
    let title = title.unwrap_or("--".to_string());
    format!("Day {:02}: {}", day + 1, title)
}

fn build_part_view(day: usize) -> Dialog {
    let select_view = SelectView::new()
        .item("Part 1", PuzzlePart::One)
        .item("Part 2", PuzzlePart::Two)
        .on_submit(move |c, part| emit(c, HubEvent::PuzzleSelected(day, *part)));

    Dialog::new()
        .title("Select Part")
        .content(select_view)
        .dismiss_button("Back")
}
