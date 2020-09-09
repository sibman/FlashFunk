#![allow(dead_code, unused_imports, unused_must_use, unused_variables)]

use actix::Addr;
use ctpbee_rs::ac::Ac;
use ctpbee_rs::app::CtpbeeR;
use ctpbee_rs::structs::{BarData, TickData};
use std::borrow::Borrow;
use std::thread;

struct Strategy {
    pub name: String,
    pub addr: Option<Addr<CtpbeeR>>,
}

impl Ac for Strategy {
    fn on_bar(&mut self, bar: BarData) {
        let name = self.name.clone();
        println!("{} got bar", name);
    }

    fn on_tick(&mut self, tick: TickData) {
        let name = self.name.clone();
        println!("{} got tick {:?}", name, self.get_addr());
    }

    fn init(&mut self, runtime: Addr<CtpbeeR>) {
        self.addr = Some(runtime);
    }

    fn get_addr(&mut self) -> &Addr<CtpbeeR> {
        self.addr.as_ref().unwrap()
    }
}

#[actix_rt::main]
async fn main() {
    let mut account = CtpbeeR::new("ctpbee".to_string());
    let str = Strategy {
        name: "hello".to_string(),
        addr: None,
    };
    let str2 = Strategy {
        name: "bug".to_string(),
        addr: None,
    };
    account.add_strategy(Box::new(str));
    account.add_strategy(Box::new(str2));
    let (addr, x) = account.run_forever();
    let copy = addr.clone();
    thread::spawn(move || loop {
        addr.do_send(TickData::default());
    });
    x.await;
}
