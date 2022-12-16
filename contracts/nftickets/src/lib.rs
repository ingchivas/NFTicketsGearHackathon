#![no_std]

pub mod io;

use gear_lib::multitoken::io::*;
use gstd::{msg, prelude::*, ActorId};
use multitoken_io::*;

use crate::io::*;

const ZERO_ID: ActorId = ActorId::zero();
const NFT_COUNT: u128 = 1000;

#[derive(Debug, Default)]
pub struct Event {
    // Creator Data
    pub owner_id: ActorId,
    pub contract_id: ActorId,

    // Event Data
    pub name: String,
    pub desc: String,
    pub event_id: u128,

    // Ticket Data
    pub ftid: u128,
    pub tickets_available: u128,
    pub date: u128,
    pub creator: ActorId,
    pub ntickets: u128,
    
    // Buyer Data
    pub ticket_buyers: BTreeSet<ActorId>,

    // Misc
    pub metadata: BTreeMap<ActorId, BTreeMap<u128, Option<TokenMetadata>>>,
    pub active: bool,
    pub id_cntr: u128,
}

static mut CONTRACT: Option<Event> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let cfg: InitEvent = msg::load().expect("Failed to load init data");
    let event = Event {
        owner_id: cfg.owner_id,
        contract_id: cfg.mtk_ct,
        ..Default::default()
    };
    CONTRACT = Some(event);
}

#[gstd::async_main]
async unsafe fn main() {
    let actions: EventAction = msg::load().expect("Failed to load actions");
    let event: &mut Event = unsafe { 
        CONTRACT.get_or_insert(Default::default()) 
    };
    match action {
        EventAction::Create {
            creator,
            name,
            desc,
            tickets,
            date,
            location,
        } => concert.create_ev(name, description, creator, number_of_tickets, date),
        EventAction::Hold => concert.hold_concert().await,
        EventAction::BuyEvTickets { 
            amount, metadata 
        } => {
            concert.buy_tickets(amount, metadata).await
        }
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: EventStateQ = msg::load().expect("Failed to load state query");
    let event = unsafe { 
        CONTRACT.get_or_insert(Default::default()) 
    };
    let reply = match state {
        EventStateQ::ActiveEvent => EventStateR::CurrentConcert {
            name: event.name.clone(),
            description: event.desc.clone(),
            date: event.date,
            n_tickets: event.ntickets,
            tickets_available: event.tickets_available,
        },
        EventStateQ::Buyers => EventStateR::Buyers {
            wallets: event.ticket_buyers.clone(),
        },
        EventStateQ::UserTickets { 
            user 
        } => EventStateR::UserTickets {
            tickets: event
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

impl Event {
    fn create_ev(
        &mut self,
        name: String,
        description: String,
        creator: ActorId,
        n_tickets: u128,
        date: u128,
    ) {
        if self.active {
            panic!("Event already active");
        }
        self.creator = creator;
        self.event_id = self.id_cntr;
        self.ftid = self.event_id;
        self.name = name;
        self.desc = description;
        self.ntickets = n_tickets;
        self.date = date;
        self.active = true;
        self.tickets_available = n_tickets;
        msg::reply(
            Event::EvCreation {
                creator,
                event_id: self.event_id,
                available_tickets: n_tickets,
                date,
            },
            0,
        )
        .expect("Failed to reply to creator");
    }

    async fn buy_tickets(&mut self, amount: u128, mtd: Vec<Option<TokenMetadata>>) {

        if amount < 1 {
            panic!("Amount must be greater than 0");
        }

        if msg::source() == ZERO_ID {
            panic!("Cannot buy tickets from zero address");
        }

        if self.tickets_available < amount {
            panic!("Not enough tickets available");
        }

        if mtd.len() != amount as usize {
            panic!("Metadata length does not match amount");
        }

        if amount > 3 {
            panic!("Cannot buy more than 3 tickets at a time");
        }

        for meta in mtd {
            self.id_cntr += 1;
            self.metadata
                .entry(msg::source())
                .or_default()
                .insert(self.id_cntr + 1, meta);
        }

        self.ticket_buyers.insert(msg::source());
        self.tickets_available -= amount;
        msg::send_for_reply_as::<_, MTKEvent>(
            self.contract_id,
            MyMTKAction::Mint {
                amount,
                token_metadata: None,
            },
            0,
        )
        .expect("Error sending mint request to MTK contract")
        .await
        .expect("Error during minting");

        msg::reply(
            Event::EvPurchase {
                concert_id: self.event_id,
                amount,
            },
            0,
        )
        .expect("Failed to reply to buyer");
    }


    async fn hold_ev(&mut self) {
        if msg::source() != self.creator {
            panic!("Only the creator can hold the concert");
        }

        let accounts: Vec<_> = self.ticket_buyers.clone().into_iter().collect();
        let tokens: Vec<TokenId> = iter::repeat(self.ftid)
            .take(accounts.len())
            .collect();

        let balance_response: MTKEvent = msg::send_for_reply_as(
            self.contract_id,
            MyMTKAction::BalanceOfBatch {
                accounts,
                ids: tokens,
            },
            0,
        )
        .expect("Error sending balance of batch request to MTK contract")
        .await
        .expect("Error during balance of batch");

        let balances: Vec<BalanceReply> =
            if let MTKEvent::BalanceOf(balance_response) = balance_response {
                balance_response
            } else {
                Vec::new()
            };

            for balance in &balances {
            msg::send_for_reply_as::<_, MTKEvent>(
                self.contract_id,
                MyMTKAction::Burn {
                    id: balance.id,
                    amount: balance.amount,
                },
                0,
            )
            .expect("Error in message to MTK contract")
            .await
            .expect("Error during burning");
        }

        for actor in &self.ticket_buyers {
            let mut ids = vec![];
            let mut ammts = vec![];
            let mut meta = vec![];
            let actor_mdata = self.metadata.get(actor);
            if let Some(actor_md) = actor_mdata.cloned() {
                for (token, token_meta) in actor_md {
                    ids.push(token);
                    ammts.push(NFT_COUNT);
                    meta.push(token_meta);
                }
                msg::send_for_reply_as::<_, MTKEvent>(
                    self.contract_id,
                    MyMTKAction::MintBatch {
                        ids,
                        amounts: ammts,
                        tokens_metadata: meta,
                    },
                    0,
                )
                .expect("Error sending mint request to MTK contract")
                .await
                .expect("Error minting NFTs");
            }
        }
        self.active = false;
        msg::reply(
            Event::EvHold {
                concert_id: self.event_id,
            },
            0,
        )
        .expect("Failed to reply to creator");
    }
}

gstd::metadata! {
    title: "Event",
    init:
        input: InitEvent,
    handle:
        input: EventAction,
        output: Event,
    state:
        input: EventStateQ,
        output: EventStateR,
}
