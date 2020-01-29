use crate::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Markov {
    pub chain: HashMap<Vec<Vec<u8>>, LinkSet>,
    pub starts: HashSet<Vec<u8>>,
    pub depth: usize,
    pub name: String,
}

impl Markov {
    pub fn new(depth: usize, name: impl ToString) -> Self {
        Markov {
            depth,
            name: name.to_string(),
            chain: Default::default(),
            starts: Default::default(),
        }
    }

    // TODO make this trait-based so the desired behavior can be specified
    pub fn generate<R: ?Sized + Rng>(
        &self,
        rng: &mut R,
        min: usize,
        max: usize,
        query: Option<&str>,
    ) -> Option<String> {
        #[inline(always)]
        fn context(words: &[Vec<u8>], depth: usize) -> &[Vec<u8>] {
            &words[words.len().saturating_sub(depth)..]
        };

        let chances = [rng.gen_range(0.10, 0.40), rng.gen_range(0.10, 0.40)];
        let mut desired = rng.gen_range(1, 3);
        let mut last = false;

        tracing::trace!(length.min = %min, length.max = %max, query = ?query);

        let mut words: Vec<Vec<u8>> = vec![];
        loop {
            match query {
                Some(query) if rng.gen_bool(chances[0]) && desired > 0 && !last => {
                    words.push(query.as_bytes().to_vec());
                    desired -= 1;
                    last = true;
                }
                _ => {
                    let start = self.starts.iter().choose(rng)?.clone();
                    words.push(start);
                    last = false;
                }
            }

            if words.len() >= max {
                break;
            }

            while let Token::Word(word) = self.next_word(rng, context(words.as_slice(), self.depth))
            {
                if let Some(query) = query {
                    if rng.gen_bool(chances[1]) && desired > 0 && !last {
                        words.push(query.as_bytes().to_vec());
                        desired -= 1;
                    }
                }

                words.push(word.clone());
                last = false;
                if words.len() >= max {
                    break;
                }
            }

            if words.len() >= min {
                break;
            }
        }

        words
            .iter()
            .map(|s| s.as_ref())
            .flat_map(std::str::from_utf8)
            .fold(String::new(), |mut acc, str| {
                if !acc.is_empty() {
                    acc.push_str(" ")
                }
                acc.push_str(str);
                acc
            })
            .into()
    }

    pub fn train_text(&mut self, text: &str) {
        for set in text
            .split_terminator(|c| ".?!\n".contains(c))
            .map(|s| {
                s.trim()
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.as_bytes().to_vec())
                    .collect::<Vec<_>>()
            })
            .filter(|s| !s.is_empty())
        {
            self.train_words(set)
        }
    }

    fn train_words(&mut self, words: Vec<Vec<u8>>) {
        let depth = std::cmp::min(self.depth, words.len() - 1);

        let start = &words[0];
        self.starts.insert(start.clone());

        for width in 1..=depth {
            for window in words.windows(width + 1) {
                let tail = window.last().expect("get last window").clone();
                self.train_link(&window[..window.len() - 1], Token::Word(tail))
            }
            self.train_link(&words[words.len() - width..], Token::End)
        }
    }

    fn train_link(&mut self, context: &[Vec<u8>], token: Token) {
        if let Some(link_set) = self.chain.get_mut(context) {
            link_set.insert(token);
            return;
        }

        self.chain
            .entry(context.to_vec())
            .or_default()
            .insert(token);
    }

    fn next_word<R: ?Sized + Rng>(&self, rng: &mut R, context: &[Vec<u8>]) -> Token {
        let mut link_sets = (1..=std::cmp::min(self.depth, context.len()))
            .filter_map(|width| {
                self.chain
                    .get(&context[context.len() - width..])
                    .map(|link_set| (width, link_set))
            })
            .peekable();

        let mut pooled_links = match link_sets.peek() {
            Some((_, link_set)) => Vec::<Link>::with_capacity(link_set.len()),
            _ => return Token::End,
        };

        for (width, link_set) in link_sets {
            for mut link in link_set.iter().cloned() {
                link.count *= width;
                match pooled_links.iter_mut().find(|l| l.token == link.token) {
                    Some(existing) => existing.merge(&link),
                    None => pooled_links.push(link),
                }
            }
        }

        weighted_selection(rng, &pooled_links).token.clone()
    }
}

fn weighted_selection<'a, R: ?Sized + Rng>(rng: &mut R, links: &'a [Link]) -> &'a Link {
    let total_count = links.iter().map(|l| l.count).sum::<usize>();
    links
        .iter()
        .cycle()
        .skip(rng.gen_range(0, total_count))
        .scan(total_count, |remaining, link| {
            *remaining = remaining.saturating_sub(link.count);
            Some((*remaining, link))
        })
        .filter_map(|(remaining, link)| if remaining == 0 { None } else { Some(link) })
        .next()
        .expect("get next weighted")
}
