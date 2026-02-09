pub trait Viewable<ViewType> {
  type Callback;
  
  fn view(&self, callback: Self::Callback) -> ViewType;
}
