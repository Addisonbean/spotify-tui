use rspotify::model::page::Page;

pub struct InteractivePage<T> {
  page: Page<T>,
  selected_index: usize,
  filtered_items: Vec<T>,
}

impl<T: Clone> InteractivePage<T> {
  pub fn new(page: Page<T>) -> InteractivePage<T> {
    let filtered_items = page.items.clone();
    InteractivePage {
      page,
      selected_index: 0,
      filtered_items,
    }
  }

  pub fn apply_filter(&mut self, filter: impl Fn(&T) -> bool) {
    self.selected_index = 0;
    self.filtered_items.clear();
    let results = self.page.items.iter().filter(|&i| filter(i)).cloned();
    self.filtered_items.extend(results);
  }

  pub fn remove_filter(&mut self) {
    self.selected_index = 0;
    self.filtered_items.clear();
    self.filtered_items.extend(self.page.items.iter().cloned());
  }
}

impl<T> InteractivePage<T> {
  pub fn items(&self) -> &Vec<T> {
    &self.filtered_items
  }

  pub fn selected_index(&self) -> usize {
    self.selected_index
  }

  pub fn selected_item(&self) -> &T {
    // TODO: why no get???
    &self.filtered_items[self.selected_index]
  }

  pub fn select_next_item(&mut self) {
    if !self.filtered_items.is_empty() && self.selected_index + 1 < self.filtered_items.len() {
      self.selected_index += 1;
    } else {
      self.selected_index = 0;
    }
  }

  pub fn select_prev_item(&mut self) {
    if !self.filtered_items.is_empty() {
      if self.selected_index > 0 {
        self.selected_index -= 1;
      } else {
        self.selected_index = self.filtered_items.len() - 1;
      }
    } else {
      self.selected_index = 0;
    }
  }

  pub fn select_first_item(&mut self) {
    self.selected_index = 0;
  }

  pub fn select_middle_item(&mut self) {
    self.selected_index = self.filtered_items.len() / 2;
    if self.filtered_items.len() % 2 == 0 && self.selected_index != 0 {
      self.selected_index -= 1;
    }
  }

  pub fn select_last_item(&mut self) {
    if !self.filtered_items.is_empty() {
      self.selected_index = self.filtered_items.len() - 1;
    } else {
      self.selected_index = 0;
    }
  }
}
