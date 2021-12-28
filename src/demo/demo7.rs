pub enum WebEvent {
    PageLoad,
    PageUnload
}

// 这只是个方法  叫什么无所谓
pub fn inspects(event: WebEvent) {
  
  match event {
    WebEvent::PageLoad => println!("page load"),
    WebEvent::PageUnload => println!("page unload")
  }
}
