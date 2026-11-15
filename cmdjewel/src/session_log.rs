//! Session logging: `RUST_LOG=debug cmdjewel 2>cmdjewel.log` records every
//! raw input event as the terminal delivers it, before cursive dispatches it.
//! The backend is cursive's own, wrapped; nothing else changes.

use cursive::backend::Backend;
use cursive::event::Event;
use cursive::style::{Color, ColorPair, Effect};
use cursive::Vec2;

pub struct LoggingBackend(pub Box<dyn Backend>);

/// Wraps a backend so its input is logged.
pub fn wrap(backend: Box<dyn Backend>) -> Box<dyn Backend> {
    log::info!("backend {}, screen {:?}", backend.name(), backend.screen_size());
    Box::new(LoggingBackend(backend))
}

impl Backend for LoggingBackend {
    fn poll_event(&mut self) -> Option<Event> {
        let event = self.0.poll_event();
        match &event {
            None | Some(Event::Refresh) => {}
            Some(Event::WindowResize) => {
                log::debug!("input {:?} -> {:?}", event, self.0.screen_size())
            }
            Some(e) => log::debug!("input {:?}", e),
        }
        event
    }
    fn set_title(&mut self, title: String) {
        self.0.set_title(title)
    }
    fn refresh(&mut self) {
        self.0.refresh()
    }
    fn has_colors(&self) -> bool {
        self.0.has_colors()
    }
    fn screen_size(&self) -> Vec2 {
        self.0.screen_size()
    }
    fn move_to(&self, pos: Vec2) {
        self.0.move_to(pos)
    }
    fn print(&self, text: &str) {
        self.0.print(text)
    }
    fn clear(&self, color: Color) {
        self.0.clear(color)
    }
    fn set_color(&self, colors: ColorPair) -> ColorPair {
        self.0.set_color(colors)
    }
    fn set_effect(&self, effect: Effect) {
        self.0.set_effect(effect)
    }
    fn is_persistent(&self) -> bool {
        self.0.is_persistent()
    }
    fn unset_effect(&self, effect: Effect) {
        self.0.unset_effect(effect)
    }
    fn name(&self) -> &str {
        self.0.name()
    }
}
