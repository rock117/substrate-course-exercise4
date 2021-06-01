use crate::trafficlight::LightDuration;

pub mod area;
mod int_sum;
mod trafficlight;

fn main() {
   assert_eq!(10f64, area::area(&area::Rectangle(2f64, 5f64)));

   assert_eq!(Some(6), int_sum::sum(&[1,2,3]));
   assert_eq!(None, int_sum::sum(&[1,2,!0]));

   assert_eq!(3, trafficlight::TrafficLight::Green.duration());
   assert_eq!(5, trafficlight::TrafficLight::Red.duration());
   assert_eq!(4, trafficlight::TrafficLight::Yellow.duration())
}
