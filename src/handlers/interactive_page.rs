use super::common_key_events;

use rspotify::model::page::Page;

pub struct InteractivePage<T> {
  page: Page<T>,
  selected_index: usize,
}

impl<T> InteractivePage<T> {
  pub fn new(page: Page<T>) -> InteractivePage<T> {
    InteractivePage {
      page,
      selected_index: 0,
    }
  }

  pub fn items(&self) -> &Vec<T> {
    &self.page.items
  }

  pub fn selected_index(&self) -> usize {
    self.selected_index
  }

  pub fn selected_item(&self) -> &T {
      &self.page.items[self.selected_index]
  }

  pub fn select_next_item(&mut self) {
    self.selected_index = common_key_events::on_down_press_handler(&self.page.items, Some(self.selected_index));
  }

  pub fn select_prev_item(&mut self) {
    self.selected_index = common_key_events::on_up_press_handler(&self.page.items, Some(self.selected_index));
  }

  pub fn select_first_item(&mut self) {
    self.selected_index = common_key_events::on_high_press_handler();
  }

  pub fn select_middle_item(&mut self) {
    self.selected_index = common_key_events::on_middle_press_handler(&self.page.items);
  }

  pub fn select_last_item(&mut self) {
    self.selected_index = common_key_events::on_low_press_handler(&self.page.items);
  }
}
