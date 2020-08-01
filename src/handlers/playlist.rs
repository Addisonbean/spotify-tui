use super::{
  super::app::{App, DialogContext, TrackTableContext},
  common_key_events,
};
use crate::app::ActiveBlock;
use crate::event::Key;
use crate::network::IoEvent;

pub fn handler(key: Key, app: &mut App) {
  if let Some(playlists) = app.playlists.as_mut() {
    match key {
      k if common_key_events::right_event(k) => common_key_events::handle_right_event(app),
      k if common_key_events::down_event(k) => playlists.select_next_item(),
      k if common_key_events::up_event(k) => playlists.select_prev_item(),
      k if common_key_events::high_event(k) => playlists.select_first_item(),
      k if common_key_events::middle_event(k) => playlists.select_middle_item(),
      k if common_key_events::low_event(k) => playlists.select_last_item(),
      Key::Enter => {
        let selected_playlist = playlists.selected_item();
        app.track_table.context = Some(TrackTableContext::MyPlaylists);
        app.playlist_offset = 0;

        let playlist_id = selected_playlist.id.to_owned();
        app.dispatch(IoEvent::GetPlaylistTracks(playlist_id, app.playlist_offset));
      }
      Key::Char('D') => {
        let selected_playlist = playlists.selected_item();
        app.dialog = Some(selected_playlist.name.clone());
        app.confirm = false;

        let route = app.get_current_route().id.clone();
        app.push_navigation_stack(route, ActiveBlock::Dialog(DialogContext::PlaylistWindow));
      }
      _ => {}
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {}
}
