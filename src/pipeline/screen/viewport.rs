#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
     width: usize,
     height: usize,
     aspect_ratio: f64,
 }
 
 impl Viewport {
     pub fn new(width: usize, height: usize) -> Self {
         let aspect_ratio = width as f64 / height as f64;
         Viewport {
             width,
             height,
             aspect_ratio,
         }
     }
 
     pub fn width(&self) -> usize {
         self.width
     }
 
     pub fn height(&self) -> usize {
         self.height
     }
 
     pub fn aspect_ratio(&self) -> f64 {
         self.aspect_ratio
     }
 }