use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkSet(Vec<Link>);

impl LinkSet {
    #[inline]
    pub fn insert(&mut self, token: Token) {
        if let Some(existing) = self.existing(&token) {
            existing.merge(&token.into());
            self.sort_unstable_by(|a, b| b.cmp(a)); // reverse
            self.dedup();
        } else {
            self.push(token.into());
        }
    }

    #[inline]
    fn existing(&mut self, token: &Token) -> Option<&mut Link> {
        self.iter_mut().find(|l| l.token == *token)
    }
}

impl std::ops::Deref for LinkSet {
    type Target = Vec<Link>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LinkSet {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, Serialize, Deserialize)]
pub struct Link {
    pub token: Token,
    pub count: usize,
}

impl From<Token> for Link {
    #[inline(always)]
    fn from(token: Token) -> Self {
        Link { token, count: 1 }
    }
}

impl Link {
    #[inline(always)]
    pub fn merge(&mut self, rhs: &Self) {
        debug_assert!(rhs.token == self.token);
        self.count += rhs.count;
    }
}

impl PartialEq for Link {
    #[inline(always)]
    fn eq(&self, rhs: &Self) -> bool {
        self.count.eq(&rhs.count)
    }
}

impl Ord for Link {
    #[inline(always)]
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.count.cmp(&rhs.count)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub enum Token {
    Word(Vec<u8>),
    End,
}
