use gear_lib::multitoken::io::*;
use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EventAction {
    Create {
        creator: ActorId,
        name: String,
        desc: String,
        tickets: u128,
        date: u128,
        location: String,
    },
    Hold,
    BuyEvTickets {
        amount: u128,
        metadata: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    EvCreation {
        creator: ActorId,
        event_id: u128,
        available_tickets: u128,
        date: u128,
    },
    EvHold {
        concert_id: u128,
    },
    EvPurchase {
        concert_id: u128,
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EventStateQ {
    ActiveEvent,
    Buyers,
    UserTickets {
        user: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EventStateR {
    CurrentConcert {
        name: String,
        description: String,
        date: u128,
        n_tickets: u128,
        tickets_available: u128,
    },
    Buyers {
        wallets: BTreeSet<ActorId>,
    },
    UserTickets {
        tickets: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitEvent {
    pub owner_id: ActorId,
    pub mtk_ct: ActorId,
}