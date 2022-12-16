#![no_std]

pub mod io;

use std::fs::Metadata;

use gear_lib::multitoken::io::*;
use gstd::{msg, prelude::*, ActorId};
use multitoken_io::*;

use crate::io::*;

const ZERO_ID: ActorId = ActorId::zero();
const NFT_COUNT: u128 = 100;

#[derive(Debug, Default)]
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
    pub ticket_art_uri: String,
    
    // TicketBuyerData
    pub buyers_data : BTreeSet<ActorId>,

    // Misc
    pub ct_id : u128,
    pub active: bool,
    pub metadata: BTreeMap<ActorId, BTreeMap<u128, Option<TokenMetadata>>>,

}

static mut CONTRACT: Option<Event> = None;

#[no_mangle]
pub unsafe extern "C" fn init() {
    let cfg : InitData = msg::load().expect("Failed to load init data");
    let event = Event {
        owner_id: cfg.owner_id,
        contract: cfg.contract,
        ..Default::default()
    };
    CONTRACT = Some(Event::new(event));
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
            event.create_event(event_id, name, description, available_tickets, date, location).await;
        },
        EventActions::Hold => {
            event.hold().await;
        },
        EventActions::Purchase{ammount, metadata} => {
            event.purchase(ammount, metadata).await;
        },
    }
}

#[no_mangle]
extern "C" fn meta_state () -> *mut [i32;2]
{
    let state : EventStateQ = msg::load().expect("Failed to load state query");
    let event = unsafe {CONTRACT.get_or_insert(Default::default())};
    let result = EventStateR = match state {
        EventStateQ::CurrentEvent => {
            EventStateR::CurrentEvent{
                name : event.name.clone(),
                description : event.description.clone(),
                date : event.date,
                location : event.location.clone(),
                tickets_sold : event.tickets,
                tickets_available : event.available_tickets,
                event_id : event.event_id,
            }
        },
        EventStateQ::Buyers => {
            EventStateR::Buyers{
                buyers: event.buyers_data,
            }
        },
        EventStateQ::UserTickets{user} => EventStateR::UserTickets {
            tickets: concert
                .metadata
                .get(&user)
                .unwrap_or(&BTreeMap::new())
                .values()
                .cloned()
                .collect(),
        },
    };
    gstd::util::to_leak_ptr(reply.encode())
}


impl Event
{
    fn create_event(
        &mut self,
        name: String,
        description: String,
        creator: ActorId,
        tickets: u128,
        date : u128,
    )
    {
        if self.owner_id != ZERO_ID {
            panic!("Event already exists");
        }
        self.creator = creator;
        self.event_id = self.id_counter;
        self.ticket_id = self.event_id;
        self.name = name;
        self.description = description;
        self.tickets = tickets;
        self.date = date;
        self.active = true;
        self.available_tickets = tickets;
        msg::reply(Event::Create{
            creator,
            event_id: self.event_id,
            tickets,
            date,
        }, 0).expect("Failed to reply");
    }

    async fn buy_ticket(&mut self, ammount: u128, metadata: Option<TokenMetadata>) {
        if self.available_tickets < ammount {
            panic!("Not enough tickets available");
        }
        if self.active == false {
            panic!("Event is not active");
        }

        if msg::source() == ZERO_ID {
            panic!("Zero address is not allowed to buy tickets");
        }
        
        if metadata.len() != amount as usize {
            panic!("Metadata length does not match amount");
        }

        if ammount < 1 {
            panic!("Ammount must be greater than 0");
        }

        if ammount > 5 {
            panic!("Ammount must be less than 5");
        }

        let buyer = msg::sender();
        let mut ticket_id = self.ticket_id;
        let mut tickets = self.tickets;
        let mut available_tickets = self.available_tickets;
        let mut buyers_data = self.buyers_data.clone();
        let mut metadata = self.metadata.clone();
        for _ in 0..ammount {
            tickets += 1;
            available_tickets -= 1;
            ticket_id += 1;
            buyers_data.insert(buyer);
            metadata
                .entry(buyer)
                .or_insert_with(BTreeMap::new)
                .insert(ticket_id, metadata.clone());
        }
        self.tickets = tickets;
        self.available_tickets = available_tickets;
        self.ticket_id = ticket_id;
        self.buyers_data = buyers_data;
        self.metadata = metadata;
        msg::reply(Event::Purchase {
            buyer,
            ammount,
            metadata,
        }, 0)
        .expect("Failed to reply");
    }

    async fn hold(&mut self) {
        if self.active == false {
            panic!("Event is not active");
        }
        self.active = false;
        msg::reply(Event::Hold, 0).expect("Failed to reply");
    }



}

gstd::metadata! {
    title: "Event",
    init:
        input: InitData,
        output: Event::Create,
    handle:
        input: EventActions,
        output: EventActions,
    state:
        input: EventStateQ,
        output: EventStateR,
}



