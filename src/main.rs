use std:: {
    cell::RefCell, env, thread:: { self, sleep }, time::Duration, 
};

use gtk4::{self as gtk, glib::closure_local};
use gtk::prelude::*;
use gtk::glib;
use std::sync::mpsc::{Receiver,Sender};
use std::f64::consts::PI;

use serde_json::Value;
const APP_ID: &str = "io.github.dyft4w.askahmed";
thread_local!{static GLOBAL:RefCell<Option<(Receiver<String>, gtk::Label)>>=RefCell::new(None);}// static INIT:bool = false;}
fn cairo_rounded_rectangle(x:f64, 
            y:f64, 
            width:f64, 
            height:f64, 
            corner_width:f64, 
            corner_height:f64, 
            cr: &gtk::cairo::Context){
    let aspect        = corner_width/corner_height;
    let corner_radius = height / 10.0;
    let radius = corner_radius / aspect;

    cr.new_sub_path();
    cr.arc(x + width - radius, y + height - radius, radius, 0.0, PI/2.0);
    cr.arc(x + radius, y + height - radius, radius, PI/2.0, PI);
    cr.arc(x + radius, y + radius, radius, PI, PI*3.0/2.0);
    cr.arc(x + width - radius, y + radius, radius, -PI/2.0, 0.0);
}
                                                                                        

fn breh(tx:Sender<String>, file: String){
    println!("{}", file);
    thread::spawn(move ||{
        let task = ||
        {
            let upload_form = {
            let option = reqwest::blocking::multipart::Form::new ().file("file", file).ok();

            if let Some(form) = option
            {
            form
            }
            else {
                return Some(Box::from("the fuck you want"));
            }
        };

        let mut path = env::current_exe().ok() ? ;
        path.set_file_name("settings.ini");
        let conf = ini::Ini::load_from_file(path).ok()?;
        let apikey = conf.section(Some("Settings")) ?.get("APIKEY") ? ;

        let client = reqwest::blocking::Client::builder()
                            .use_native_tls()
                            .build()
                            .ok()
            ? ;

        // Obtain upload url
        let response = client
                            .get("https://www.virustotal.com/api/v3/files/upload_url")
                            .header("accept", "application/json")
                            .header("x-apikey", apikey)
                            .send()
                            .ok()
            ? ;
            let v: Value = serde_json::from_str(&response.text().ok()?).ok()?;
            let upload_url = v.get("data") ?.as_str() ? ;

            // Upload file
            let response = client
                                .post(upload_url)
                                .header("accept", "application/json")
                                .header("x-apikey", apikey)
                                .multipart(upload_form)
                                .send()
                                .ok()
                ? ;
            let v: Value = serde_json::from_str(&response.text().ok()?).ok()?;
            let analysis_id = v.get("data") ?.get("id") ?.as_str() ? ;

            // Get analysis results
            let response_content = loop
            {
                let response = client
                                    .get(format !(
                                        "https://www.virustotal.com/api/v3/analyses/{analysis_id}"))
                                    .header("accept", "application/json")
                                    .header("x-apikey", apikey)
                                    .send()
                                    .ok()
                    ? ;

                if !response
                    .status().is_success()
                    {
                        return None;
                    }

                let v: Value = serde_json::from_str(&response.text().ok()?).ok()?;
                let status = v.get("data") ?.get("attributes") ?.get("status") ?.as_str() ? ;
                if status
                    == "completed"
                    {
                        break v;
                    }
                sleep(Duration::from_secs(1));
            };

            #[derive(Default)]
            struct Agregate {
                good : u32,
                bad : u32,
            }

            // Agregate analysis results
            let agregate
                = response_content
                        .get("data")
                ?.get("attributes")
                ?.get("results")
                ?.as_object()
                ?.values()
        .map(| v | { v.get("category").and_then(Value::as_str).unwrap_or("type-unsupported") })
                        .fold(Agregate::default(), | mut aggr, cat | { match cat { "harmless" | "undetected" => aggr.good += 1, "suspicious" | "malicious" => aggr.bad += 1, _ => {} } aggr });
            let bad_ratio = agregate.bad as f64 / (agregate.good + agregate.bad) as f64;

            let msg = if bad_ratio < 0.05 {
                "looks fine bro"
            } else if bad_ratio
                < 0.1 { "kinda sus" } else { "not good" };
            Some(Box::from(msg))
        };
        
        let result:Box<str> = task().unwrap_or(Box::from("idk"));
        tx.send(result.into_string()).unwrap();
        gtk::glib::idle_add_once(move||{update_label()});
    });

}

fn update_label(){
    GLOBAL.with_borrow(|global|{
            match global{
                Some((rx, label)) => label.set_label(&rx.recv().unwrap()),
                None =>println!("asd")
            }


});
    //label.borrow().set_label(&rx.recv().unwrap());
}


#[cfg(target_os = "linux")]
fn get_ahmed() -> String{
    "/usr/share/ask-ahmed/Ahmed.png".to_string()
}


#[cfg(target_os = "windows")]
fn get_ahmed() -> String{
    let path = std::env::current_exe();
    let ahmed_home = path.unwrap().parent().unwrap().as_os_str().to_str().unwrap().to_string();
    ahmed_home+"/Ahmed.png"
}
fn draw_bubble(area:&gtk::DrawingArea){
    area.set_draw_func(|_area, ctx, _x, _y|{
        cairo_rounded_rectangle(20.0, 25.0, 160.0, 165.0, 20.0, 20.0, &ctx);
        ctx.line_to(180.0,50.0);
        ctx.line_to(220.0,90.0);
        ctx.line_to(180.0,90.0);
        ctx.close_path();
        ctx.set_source_rgb(1.0, 1.0, 0.93);
        ctx.fill_preserve().unwrap();
        ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
        ctx.set_line_width(5.0);
        ctx.stroke().unwrap();
        
    });

}
fn build_ui(app: &gtk::Application, message:&str){
    
    let fix_box = gtk::Fixed::builder().build();
    let ahmed = gtk::Image::builder().file(&get_ahmed()).build();
    let ok = gtk::Button::builder().label("ok").build();
    let area = gtk::DrawingArea::builder().build();
    let overlay = gtk::Overlay::builder().child(&area).build();

    draw_bubble(&area);
    overlay.set_valign(gtk::Align::Start);
    overlay.set_halign(gtk::Align::Start);

    GLOBAL.with_borrow(|global|{match global{
        Some((_,label)) => {
            label.set_margin_start(35);
            label.set_margin_top(45);
            label.set_label(message);
            label.set_valign(gtk::Align::Start);
            label.set_halign(gtk::Align::Start);
            overlay.add_overlay(label);
        }
        None => println!("skibidi toilet")
    }});


    area.set_size_request(225,200);
    ahmed.set_size_request(140, 150);
    ok.set_size_request(80, 25);
    
    fix_box.put(&overlay, 0.0, 0.0);
    fix_box.put(&ahmed, 230.0, 40.0);
    fix_box.put(&ok, 160.0,220.0);
    


    let window = gtk::ApplicationWindow::builder().application(app)
        .title("ask ahmed")
        .resizable(false)
        .default_height(270)
        .default_width(400)
        .child(&fix_box)
        .build();

    //ok.connect_closure("clicked", true, closure_local!(|_ok: gtk4::Button, window: gtk4::ApplicationWindow|{window.close()}));
    ok.connect_clicked(|_ok|{std::process::exit(0)});
    window.present();
}

fn main() -> gtk::glib::ExitCode{
    gtk::init().unwrap();
    let app = gtk::Application::builder().application_id(APP_ID).flags(gtk::gdk::gio::ApplicationFlags::HANDLES_OPEN).build();
    let (tx,rx) = std::sync::mpsc::channel();
    let label = gtk4::Label::builder().build();
    GLOBAL.with_borrow_mut(|global|{*global=Some((rx,label))});
    app.connect_open(move |app,file,_str|{
        breh(tx.clone(), file[0].path().unwrap().to_str().unwrap().to_string()); //god fucking damn it
        build_ui(&app, "hold up");
        //INIT.with(|init|{if !init {build_ui(&app)}});
    });
    app.connect_closure("activate", true, closure_local!(|app|{build_ui(app,"the fuck you want");}));
    app.run()
}

