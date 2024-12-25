pub fn show_found(config: &crate::Config, text: &str) {
  if config.case_insentitive {
    for (line, lineno) in search_icase_iter(&config.query, text) {
      println!("{}:{lineno}: {line}", &config.file_path);
    }
  } else {
    for (line, lineno) in search_iter(&config.query, text) {
      println!("{}:{lineno}: {line}", &config.file_path);
    }
  }
}

fn search_icase_iter<'a>(
  query: &str,
  contents: &'a str,
) -> impl std::iter::Iterator<Item = (&'a str, usize)> {
  let query = query.to_lowercase();
  contents
    .lines()
    .map(|line| (line, line.to_lowercase()))
    .zip(1usize..)
    .filter(move |((_, line), _)| line.contains(&query))
    .map(|((line, _), lineno)| (line, lineno))
}

fn search_iter<'a>(
  query: &str,
  contents: &'a str,
) -> impl std::iter::Iterator<Item = (&'a str, usize)> {
  contents
    .lines()
    .zip(1usize..)
    .filter(move |(line, _)| line.contains(query))
}

#[cfg(test)]
mod tests {
  #[cfg(test)]
  mod search {
    #[cfg(test)]
    mod case_sensitive {
      use super::super::super::search_iter;
      #[test]
      fn found() {
        let query = "duct";
        let contents = "\
I am
very productive
duck farmer
in a production
environment
near a Ductile
indistry.";
        let found: Vec<(&str, usize)> = search_iter(query, contents).collect();
        assert_eq!(
          vec![("very productive", 2), ("in a production", 4),],
          found
        );
      }

      #[test]
      fn not_found() {
        let query = "five";
        let contents = "\
one
two
three
four
Five
six
seven
";
        let found: Vec<_> = search_iter(query, contents).collect();
        assert_eq!(Vec::<(&str, usize)>::new(), found);
      }
    }
    #[cfg(test)]
    mod case_insensitive {
      use super::super::super::search_icase_iter;
      #[test]
      fn found() {
        let query = "rUsT";
        let contents = "\
Rust is a systems
programming language,
rust also refers to
a chemical compound
that is formed when
iron reacts with water.
Chemists see rust as highly
toxic while systems
programmers cannot imagine
a world without rust.
Get Rusty people.
Choose your poison.";
        let found: Vec<_> = search_icase_iter(query, contents).collect();
        assert_eq!(
          vec![
            ("Rust is a systems", 1),
            ("rust also refers to", 3),
            ("Chemists see rust as highly", 7),
            ("a world without rust.", 10),
            ("Get Rusty people.", 11)
          ],
          found
        );
      }

      #[test]
      fn not_found() {
        let query = "javascript";
        let contents = "\
Languages I love:
python
Rust
C++
c";
        let found: Vec<_> = search_icase_iter(query, contents).collect();
        assert_eq!(Vec::<(&str, usize)>::new(), found);
      }
    }
  }
}
