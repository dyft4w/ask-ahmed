mod gui;
mod util;
mod tui;

use gtk4::{self as gtk, 
    glib::{self, closure_local},
    prelude::*
};
use std::sync::mpsc;

const APP_ID: &str = "io.github.dyft4w.askahmed";

macro_rules! run_ui{
    ($mod:ident, $app:ident, $label:ident, $cv:ident) => {
        let (tx,rx) = mpsc::channel();
        
    
    
        util::GLOBAL.with_borrow_mut(|global|{
            *global=Some((rx, $label, $cv))
        });
    
        $app.connect_open(move |app,file,_str|{
            util::breh(tx.clone(), file[0].path().unwrap().to_str().unwrap().to_string(), $mod::update_label); //god fucking damn it
    
            //check if init
            util::INIT.with_borrow(|init|{
                if !init {
                    $mod::build_ui(app, "hold on");
                }
            });
        });
        //$app.connect_closure("activate", true, closure_local!(|app|{println!("shii");$mod::build_ui(app,"the fuck you want");}));
        $mod::build_ui($app,"the fuck you want");
    };
}



fn register_gui(app:&gtk::Application) -> i32{
    let label = Some(gtk4::Label::builder().build());
    run_ui!(gui, app, label, None);
    0
}
fn register_tui(app:&gtk::Application) -> i32{
    let cv = Some(tui::Canvas::new(60));
    run_ui!(tui, app, None, cv);
    0
}




fn main() -> gtk::glib::ExitCode{
    //init gtk
    gtk::init().unwrap();
    let app = gtk::Application::builder().application_id(APP_ID).flags(gtk::gdk::gio::ApplicationFlags::HANDLES_OPEN).build();
    // register argument '-G/--gui' (gui mode)
    app.add_main_option(
        "gui",
        glib::Char('G' as i8),
        glib::OptionFlags::IN_MAIN,
        glib::OptionArg::None,
        "open a gui window for the application",
        None
    );
    app.connect_startup(|_app|{});
    //let gui = std::env::args().find(|str|{if str=="-G"{true}else{false}});
    app.connect_handle_local_options(|app, var|{
        if var.contains("gui"){
            println!("rungui");
            register_gui(app)
        }else{
            register_tui(app)
        }
    });
    /*if gui==None {
        register_tui(&app);
    }else{
        register_gui(&app);
    }*/



    app.run()    
}