fn main() {
  let test1 = "We need more space.";
  assert_eq!(trim_spaces(test1), "We need more space.");
  
  let test2 = String::from("   There's space in front.");
  assert_eq!(trim_spaces(&test2), "There's space in front.");
  
  let test3 = String::from("There's space to the rear. ");
  assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
  
  let test4 = "  We're surrounded by space!    ";
  assert_eq!(trim_spaces(test4), "We're surrounded by space!");
  
  let test5 = "     ";
  assert_eq!(trim_spaces(test5), "");
  
  let test6 = "";
  assert_eq!(trim_spaces(test6), "");
  
  let test7 = " 🚀 ";
  assert_eq!(trim_spaces(test7), "🚀");
  println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
  let bytes = s.as_bytes();
  let mut start_index: (usize, bool) = (0, false);
  let mut end_index: usize = 0;

  for (index, &item) in bytes.iter().enumerate() {
    if item != b' ' {
      start_index.0 = if start_index.1 == false { index } else { start_index.0 };
      start_index.1 = true;
      end_index = index;
    }
  }

  if start_index.1 == false {
    return "";
  }
  
  &s[start_index.0..end_index + 1]
}