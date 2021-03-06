use chrono::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::env::args;
use human_panic::setup_panic;
use gio::{ApplicationExt, ApplicationFlags};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    self, CellRendererText, CellRendererProgress, AboutDialog, CheckMenuItem, IconSize, Image, Label, Menu, MenuBar, MenuItem, Window,
    WindowPosition, WindowType, ListStore, TreeView, TreeViewColumn, Builder, Grid, Button, Orientation,
    ReliefStyle, Widget, TextView, Fixed, ScrolledWindow, ListBox, ListBoxRow
};
use gdk_pixbuf::Pixbuf;
use glib::{Receiver, MainContext};
use crossbeam_channel::unbounded;
use log::{info, trace, warn, debug};
use crate::model::{self, User, OperUser, Operate, ShareInfo, Packet, FileInfo, ReceivedSimpleFileInfo, ReceivedPacketInner, ErrMsg};
use crate::chat_window::ChatWindow;
use crate::events::{ui::UiEvent, model::ModelEvent, model::model_run};
use crate::main_win::MainWindow;

pub fn run(){
    setup_panic!();
    ::std::env::set_var("RUST_LOG", "info");
    drop(env_logger::init());
    let application = gtk::Application::new(Some("com.github.raudient"),
                                            ApplicationFlags::FLAGS_NONE)
        .expect("Initialization failed...");
    /*let (err_tx, err_rx):  (glib::Sender<ErrMsg>, glib::Receiver<ErrMsg>) = MainContext::channel::<ErrMsg>(glib::PRIORITY_DEFAULT);
    let (model_sender, model_receiver): (crossbeam_channel::Sender<ErrMsg>, crossbeam_channel::Receiver<ErrMsg>) = unbounded();
    let main_context = MainContext::default();
    main_context.acquire();
    err_rx.attach(&main_context, |err_msg| {
        //crate::errors::ErrorDialog::new(err_msg);
        glib::Continue(false)
    });*/

    application.connect_startup(move |app| {
        info!("starting up");
        MainWindow::new(app);
    });
    application.connect_activate(|_| {
        info!("connect_activate");
    });

    application.connect_shutdown(move |_| {
        info!("shutdown!");
    });

    application.run(&args().collect::<Vec<_>>());
}