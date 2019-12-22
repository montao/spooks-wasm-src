#![recursion_limit = "128"]

extern crate easy_reader;
extern crate getopts;
extern crate rand;

use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use std::env;
use std::io::{Cursor, Error, Seek, SeekFrom, Write};
use crate::rand::Rng;

use easy_reader::EasyReader;
use getopts::Options;


pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    value: i64,
    spooks: String,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let mut c = Cursor::new(Vec::new());
        let str42 = include_str!("spook.lines").as_bytes();    
        
        // // Write into the "file" and seek to the beginning
        c.write_all(str42).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();
    
        let mut reader = EasyReader::new(c).unwrap();
        let _res = reader.build_index();
        let tmpspooks = reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap();
        
        
        Model {
            link,
            console: ConsoleService::new(),
            value: 0,
            spooks: tmpspooks
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {        
        
        let mut c = Cursor::new(Vec::new());
        let str42 = include_str!("spook.lines").as_bytes();    
        
        // // Write into the "file" and seek to the beginning
        c.write_all(str42).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();
    
        let mut reader = EasyReader::new(c).unwrap();
        let _res = reader.build_index();
        self.spooks = reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap() + &reader.random_line().unwrap().unwrap();
        
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            }
        }
        true
    }

    fn view(&self) -> Html {        
        html! {
            <div>
                <p>{ &self.spooks }</p>               
                <nav class="menu">
                <button onclick=self.link.callback(|_| Msg::Increment)>
                    { "Spook" }
                </button>
            </nav>
            </div>
        }
    }
}
