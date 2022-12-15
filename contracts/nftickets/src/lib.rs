#![no_std]

pub mod io;

use gear_lib::multitoken::io::*;
use gstd::{msg, prelude::*, ActorId};
use multitoken_io::*;

use crate::io::*;

const ZERO_ID: ActorId = ActorId::zero();
const NFT_COUNT: u128 = 100;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Event {
    // CreatorData
    pub owner_id: ActorId,
    pub contract: ActorId,
    pub creator : ActorId,

    // EventData
    pub name: String,
    pub description: String,
    pub event_id: u128,


    // TicketData
    pub available_tickets: u128,
    pub date: u128,
    pub location: String,
    pub tickets: u128,
    pub ticket_id: u128,
    
    // TicketBuyerData
    pub buyers_data : BTreeSet<ActorId>,

    // Misc
    pub ct_id : u128,
    pub active: bool,
    pub metadata: BTreeMap<ActorId, BTreeMap<u128, Option<TokenMetadata>>>,

}

static mut CONTRACT: Option<Concert> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let cfg : InitData = msg::load().expect("Failed to load init data");
    let event = Event {
        owner_id: cfg.owner_id,
        contract: cfg.contract,
        ..Default::default()
    };
    CONTRACT = Some(Concert::new(event));
}

#[gstd::async_main]
async unsafe fn main()
{
    let actions : EventActions = msg::load().expect("Failed to load actions");
    let event : &mut Event = unsafe {
        CONTRACT.get_or_insert(Default::default())
    };
    match actions {
        EventActions::Create{event_id, name, description, available_tickets, date, location} => {
            event.create(event_id, name, description, available_tickets, date, location).await;
        },
        EventActions::Hold => {
            event.hold().await;
        },
        EventActions::Purchase{ammount, metadata} => {
            event.purchase(ammount, metadata).await;
        },
    }
}