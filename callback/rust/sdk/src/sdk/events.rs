use core::fmt;
use std::io;

use anchor_client::solana_sdk::{bs58, native_token::LAMPORTS_PER_SOL};
use anchor_lang::{prelude::borsh::BorshDeserialize, Discriminator};

use crate::events::{
    CallbackUpdated, CalledBack, Fulfilled, Registered, Requested, RequestedAlt, Responded,
    Transferred, Withdrawn,
};

/// It is an error indicating that the event discriminator does not match known events
/// (see [`Event::try_from_bytes`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unknown event")]
pub struct UnknownEvent;

/// This helper enumerates all the events emitted by the program.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Event {
    CallbackUpdated(crate::events::CallbackUpdated),
    CalledBack(crate::events::CalledBack),
    Fulfilled(crate::events::Fulfilled),
    Registered(crate::events::Registered),
    Requested(crate::events::Requested),
    RequestedAlt(crate::events::RequestedAlt),
    Responded(crate::events::Responded),
    Transferred(crate::events::Transferred),
    Withdrawn(crate::events::Withdrawn),
}

impl Event {
    /// Try to create an event based on the given bytes.
    ///
    /// This can deserialize an event from a representation written
    /// in the `Program Data: <base64...>` log record.
    ///
    /// Expects bytes decoded from base64.
    ///
    /// # Error
    ///
    /// *   errors with [`UnknownEvent`] wrapped in [`io::ErrorKind::InvalidData`]
    ///     in case of unknown event
    pub fn try_from_bytes(bytes: &[u8]) -> io::Result<Self> {
        macro_rules! match_bytes {
            ($($name:ident,)+) => {
                $(
                    if bytes.starts_with(crate::events::$name::DISCRIMINATOR) {
                        return crate::events::$name::deserialize(
                            &mut &bytes[crate::events::$name::DISCRIMINATOR.len()..]
                        ).map(Self::$name);
                    }
                )+
            };
        }

        match_bytes!(
            CallbackUpdated,
            CalledBack,
            Fulfilled,
            Registered,
            Requested,
            RequestedAlt,
            Responded,
            Transferred,
            Withdrawn,
        );

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unknown discriminator for an event",
        ))
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::CallbackUpdated(ev) => ev.fmt(f),
            Event::CalledBack(ev) => ev.fmt(f),
            Event::Fulfilled(ev) => ev.fmt(f),
            Event::Registered(ev) => ev.fmt(f),
            Event::Requested(ev) => ev.fmt(f),
            Event::RequestedAlt(ev) => ev.fmt(f),
            Event::Responded(ev) => ev.fmt(f),
            Event::Transferred(ev) => ev.fmt(f),
            Event::Withdrawn(ev) => ev.fmt(f),
        }
    }
}

impl fmt::Display for CallbackUpdated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = if self.defined { "set" } else { "unset" };
        write!(
            f,
            "CallbackUpdated: {action} for {} by {}",
            self.client, self.owner
        )
    }
}

impl fmt::Display for CalledBack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CalledBack: {}", self.program)
    }
}

impl fmt::Display for Fulfilled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fulfilled: {} for {} with {}",
            bs58::encode(&self.seed).into_string(),
            self.client,
            bs58::encode(&self.randomness).into_string()
        )
    }
}

impl fmt::Display for Registered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Registered: {} as {} with {} by {}",
            self.client, self.program, self.state, self.owner,
        )
    }
}

impl fmt::Display for Requested {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let with = if self.callback.is_some() {
            if self.callback_override {
                "with request-level"
            } else {
                "with client-level"
            }
        } else {
            "without"
        };
        write!(
            f,
            "Requested: {} by {} {with} callback",
            bs58::encode(&self.seed).into_string(),
            self.client,
        )
    }
}

impl fmt::Display for RequestedAlt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let with = if self.callback.is_some() {
            "with request-level"
        } else {
            "without"
        };
        write!(
            f,
            "Requested (ALT): {} by {} {with} callback",
            bs58::encode(&self.seed).into_string(),
            self.client,
        )
    }
}

impl fmt::Display for Responded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Responded: {} to {} of {} with {}",
            self.client,
            bs58::encode(&self.seed).into_string(),
            self.client,
            bs58::encode(&self.randomness).into_string(),
        )
    }
}

impl fmt::Display for Transferred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transferred: {} from {} to {}",
            self.client, self.owner, self.new_owner,
        )
    }
}

impl fmt::Display for Withdrawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Withdrawn: {} SOL from {} by {}",
            (self.amount as f64 / LAMPORTS_PER_SOL as f64),
            self.client,
            self.owner,
        )
    }
}
