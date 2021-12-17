pub enum WebEvent {
    PageLoad,
    PageUnload
}

pub fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page load"),
    WebEvent::PageUnload => println!("page unload")
  }
}