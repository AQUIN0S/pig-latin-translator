pub fn translate_word(word: String) -> String {

  const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

  fn is_consonant(character: &char, is_end: bool) -> bool {
    if *character == 'y' && is_end {
      return false;
    }
    !VOWELS.contains(&character.to_ascii_lowercase())
  }

  fn has_soft_ending(string: &String) -> bool {
    let mut soft_endings = vec!['y', 'r', 'h'];
    for vowel in VOWELS.iter() {
      soft_endings.push(*vowel);
    }

    soft_endings.contains(&string.chars().nth_back(0).unwrap_or(' '))
  }

  if word.is_empty() {
    return word;
  }

  let chars = word.chars();
  let mut consonant_cluster = String::new();
  let mut word = String::new();
  
  for character in chars {
    if is_consonant(&character, !consonant_cluster.is_empty()) && word.is_empty() {
      consonant_cluster.push(character);
    } else {
      word.push(character);
    }
  }

  if consonant_cluster.is_empty() && has_soft_ending(&word) {
    word.push('w');
  } else if consonant_cluster.is_empty() {
    word.push('y');
  } else {
    word.push_str(&consonant_cluster.to_ascii_lowercase()[..]);
  }
  word.push_str("ay");

  word
}
