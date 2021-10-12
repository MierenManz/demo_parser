use super::helper_functions::new_view;
use super::helper_functions::read_to_string;

pub(crate) struct DemoHeader<'a> {
  header: &'a str,
  demo_protocol: u32,
  network_protocol: u32,
  server_name: &'a str,
  client_name: &'a str,
  map_name: &'a str,
  game_directory: &'a str,
  playback_time: f64,
  ticks: u64,
  frames: u64,
  sign_on_length: u64,
}

impl DemoHeader<'_> {
  pub fn new(data: &[u8]) -> DemoHeader {
    DemoHeader {
      header: &read_to_string(new_view(data, 0, 8)),
    }
  }
}
