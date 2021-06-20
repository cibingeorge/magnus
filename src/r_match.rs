use std::{fmt, ops::Deref};

use crate::{
    error::Error,
    object::Object,
    ruby_sys::ruby_value_type,
    try_convert::TryConvert,
    value::{NonZeroValue, Value},
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct RMatch(NonZeroValue);

impl RMatch {
    #[inline]
    pub fn from_value(val: Value) -> Option<Self> {
        unsafe {
            (val.rb_type() == ruby_value_type::RUBY_T_MATCH)
                .then(|| Self(NonZeroValue::new_unchecked(val)))
        }
    }
}

impl Deref for RMatch {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        self.0.get_ref()
    }
}

impl fmt::Display for RMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", unsafe { self.to_s_infallible() })
    }
}

impl fmt::Debug for RMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

impl From<RMatch> for Value {
    fn from(val: RMatch) -> Self {
        *val
    }
}

impl Object for RMatch {}

impl TryConvert for RMatch {
    #[inline]
    fn try_convert(val: &Value) -> Result<Self, Error> {
        Self::from_value(*val).ok_or_else(|| {
            Error::type_error(format!(
                "no implicit conversion of {} into MatchData",
                unsafe { val.classname() },
            ))
        })
    }
}
