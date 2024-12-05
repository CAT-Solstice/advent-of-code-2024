use std::{cmp, collections::HashMap};

pub fn main() {
  let input = include_str!( "day_05.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

struct RuleSet {
  rules: HashMap<usize, Vec<usize>>,
}

impl<'input> FromIterator<&'input str> for RuleSet {
  fn from_iter<II>( input: II ) -> Self where II: IntoIterator<Item=&'input str> {
    let mut rules = HashMap::new();
    for line in input {
      let mut split = line.split( '|' );
      let (Some(Ok(left)), Some(Ok(right))) = (split.next().map(str::parse), split.next().map(str::parse)) else {
        panic!( "failed to parse input" );
      };
      rules.entry( left )
        .and_modify( |after: &mut Vec<_>| after.push(right) )
        .or_insert_with( || vec!(right) );
    }
    Self { rules }
  }
}

impl RuleSet {
  fn ordering( &self ) -> impl FnMut( &usize, &usize ) -> cmp::Ordering + use<'_> {
    |left: &usize, right: &usize| -> cmp::Ordering {
      if let Some(after) = self.rules.get(left) {
        if after.contains( right ) {
          return cmp::Ordering::Less;
        }
      }
      if let Some(before) = self.rules.get(right) {
        if before.contains( left ) {
          return cmp::Ordering::Greater;
        }
      }
      cmp::Ordering::Equal
    }
  }
}

type Updates = Vec<Vec<usize>>;

fn parse_input( input: &str ) -> (RuleSet, Updates) {
  let mut split = input.split( "\n\n" );
  let (Some(rule_set), Some(updates)) = (split.next(), split.next()) else {
    panic!( "failed to parse input" );
  };
  let rule_set = rule_set.lines()
    .collect::<RuleSet>();
  let updates = updates.lines()
    .map( |line| line.split(',') )
    .map( |pages| pages.map( str::parse )
      .collect::<Result<_,_>>().unwrap() )
    .collect();
  (rule_set, updates)
}

mod part_one {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (rule_set, updates) = parse_input( input );
    let mut ordering = rule_set.ordering();
    let mut check_update = move |mut update: &[usize]| -> bool {
      while let Some((left, tail)) = update.split_first() {
        for right in tail {
          if ordering( left, right ) == cmp::Ordering::Greater {
            return false;
          }
        }
        update = tail;
      }
      true
    };

    updates.iter()
      .filter( |update| check_update(update) )
      .map( |update| update[ update.len()/2 ] )
      .sum()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_compute_answer() {
      let expected = 143;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (rule_set, updates) = parse_input( input );
    let mut ordering = rule_set.ordering();
    let mut check_update = |mut update: &[usize]| -> bool {
      while let Some((left, tail)) = update.split_first() {
        for right in tail {
          if ordering( left, right ) == cmp::Ordering::Greater {
            return false;
          }
        }
        update = tail;
      }
      true
    };

    let mut answer = 0;
    for mut update in updates {
      if !check_update( &update ) {
        update.sort_by( rule_set.ordering() );
        answer += update[ update.len()/2 ];
      }
    }
    answer
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_compute_answer() {
      let expected = 123;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

  #[test]
  fn test_parse_input() {
    let (rule_set, updates) = parse_input( TEST_INPUT );
    println!( "{rule_set:?}", rule_set = rule_set.rules );
    println!( "{updates:?}" );
  }

  #[test]
  fn test_ordering() {
    let (rule_set, updates) = parse_input( TEST_INPUT );
    for mut update in updates {
      update.sort_by( rule_set.ordering() );
      println!( "{update:?}" );
    }
  }
}
