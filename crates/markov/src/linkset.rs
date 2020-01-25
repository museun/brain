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
