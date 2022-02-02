macro_rules! assert_unqoted {
    ($self:ident) => {
        if $self.should_quote() {
            return Some(crate::parser::Error::QuotingNotSupported)
        }
    }
}

pub(crate) use {assert_unqoted};