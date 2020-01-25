use crate::*;

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
