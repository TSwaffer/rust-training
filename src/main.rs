fn main() {
  let rocket_fuel: String = String::from("RP-1");
  let rocket_fuel: String = process_fuel(rocket_fuel);
  println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
  println!("processing propellant {}...", propellant);
  let new_fuel: String = String::from("LNG");
  new_fuel
}
