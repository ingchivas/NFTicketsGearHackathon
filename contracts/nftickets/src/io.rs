use gear_lib::multitoken::io::*;
use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum EventActions {
    Create{
        event_id: u128,
        name: String,
        description: String,
        available_tickets: u128,
        date: u128,
        location: String,
    },
    Hold,
    Purchase {
        ammount: u128,
        metadata: Vec<Option<TokenMetadata>>
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum EventData {
    Create{
        creator: ActorId,
        event_id: u128,
        tickets: u128,
        date: u128,
        location: String,
    },
    Hold{
        event_id: u128,
    },
    Purchase {
        event_id: u128,
        ammount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]

pub enum EventStateQ{
    CurrentEvent,
    Buyers,
    UserTickets {
        user: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum EventStateR{
    CurrentEvent {
        name: String,
        description: String,
        date: u128,
        location: String,
        event_id: u128,
        tickets_available: u128,
        tickets_sold: u128,
    },
    Buyers {
        buyers: BTreeSet<ActorId>,
    },
    UserTickets {
        tickets: Vec<u128>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitData {
    pub owner: ActorId,
    pub contract_id: ActorId,
}