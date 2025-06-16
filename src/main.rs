#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(mutex_data_ptr)]

use std:: {
    cell::RefCell, env, sync::{Mutex, Arc}, thread:: { self, sleep }, time::Duration, 
};

use gtk4::{self as gtk, glib::closure_local};
use gtk::prelude::*;

/*use nwd::NwgUi;
use nwg::NativeUi;*/
use serde_json::Value;
/*use winapi::um::wingdi:: {
    CombineRgn,
    CreatePen,
    CreatePolygonRgn,
    CreateRectRgn,
    CreateRoundRectRgn,
    CreateSolidBrush,
    DeleteObject,
    FillRgn,
    FrameRgn,
    PS_SOLID,
    RGB,
    RGN_OR,
    SelectObject,
    WINDING,
};*/
const APP_ID: &str = "io.github.dyft4w.askahmed";
/*#[derive(Default)]
pub struct BasicApp {
    #[nwg_control(size : (400, 270), center : true, title : "Ahmed", icon : load_icon().as_ref(), flags : "WINDOW|VISIBLE")]
    #[nwg_events(OnInit : [Self::main], OnWindowClose : [Self::exit], OnPaint : [Self::paint_bubble(SELF, EVT_DATA)])]
    window : nwg::Window,

    #[nwg_control(bitmap : load_ahmed_img().as_ref(), size : (140, 150), position : (230, 40))]
    image : nwg::ImageFrame,

    #[nwg_control(text : "hold up", size : (145, 150), position : (30, 35), background_color : Some([255, 255, 224]), font : build_font().as_ref(), flags : "MULTI_LINE|VISIBLE")]
    text : nwg::RichLabel,

    #[nwg_control]
    #[nwg_events(OnNotice : [Self::display_result])]
    request_notice : nwg::Notice, 
    request_result : RefCell<Option<thread::JoinHandle<Box<str>>>>,

    #[nwg_control(text : "OK", size : (80, 25), position : (160, 220))]
    #[nwg_events(OnButtonClick : [Self::exit])]
    hello_button : nwg::Button,
}

impl BasicApp
{*/

fn breh(tx:std::sync::mpsc::Sender<String>){
    thread::spawn(move ||{
        let task = ||
        {
            let upload_form = {
            let option = env::args().nth(1).and_then(| p | { reqwest::blocking::multipart::Form::new ().file("file", p).ok() });

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
        let conf = ini::Ini::load_from_file(path).ok() ? ;
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
        //label.whoopsies.set_label("asd");
        tx.send(result.into_string()).unwrap();
        gtk::glib::idle_add_once(move||{update_label()});
    });
    //return rx;

}
fn update_label(){
    let (label, rx) = GLOBAL.take().unwrap();
    //label.borrow().set_label(&rx.recv().unwrap());
}
/*



fn display_result(&self)
{
    if let
        Some(handle) = self.request_result.borrow_mut().take()
        {
            let request_result = handle.join().unwrap();
            self.text.set_text(&request_result);
        }
}

fn paint_bubble(&self, data : &nwg::EventData)
{
    use winapi::shared::windef::POINT as P;

    let paint = data.on_paint();
    let ps = paint.begin_paint();
    let hdc = ps.hdc;

    unsafe
    {
        // Setup pen and brush
        let pen = CreatePen(PS_SOLID as i32, 2, RGB(0, 0, 0));
        let brush = CreateSolidBrush(RGB(255, 255, 224));

        // Create regions
        let bubble = CreateRoundRectRgn(20, 25, 185, 195, 20, 20);
        let mut pts = [
            P { x : 180, y : 90 },
            P { x : 220, y : 90 },
            P { x : 180, y : 50 },
        ];
        let tail = CreatePolygonRgn(pts.as_mut_ptr(), pts.len() as i32, WINDING);

        // Combine into one region
        let combined = CreateRectRgn(0, 0, 0, 0);
        CombineRgn(combined, bubble, tail, RGN_OR);

        // Paint
        SelectObject(hdc, pen as _);
        SelectObject(hdc, brush as _);
        FillRgn(hdc, combined, brush);
        FrameRgn(hdc, combined, pen as _, 1, 1);

        // Cleanup
        DeleteObject(bubble as _);
        DeleteObject(tail as _);
        DeleteObject(combined as _);
        DeleteObject(pen as _);
        DeleteObject(brush as _);
    }

    paint.end_paint(&ps);
}

fn exit(&self)
{
    nwg::stop_thread_dispatch();
}
}

fn load_ahmed_img() -> Option<nwg::Bitmap> {
    nwg::Bitmap::from_bin(include_bytes !("Ahmed.png")).ok()
}

fn load_icon()
    ->Option<nwg::Icon> {
        nwg::Icon::from_bin(include_bytes !("Ahmed.ico")).ok()
    }

fn build_font()
    ->Option<nwg::Font>
{
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .size(24)
        .family("Segoe UI")
        .weight(500)
        .build(&mut font)
        .ok()
        ? ;

    Some(font)
}*/
/*    #[nwg_control(bitmap : load_ahmed_img().as_ref(), size : (140, 150), position : (230, 40))]
    image : nwg::ImageFrame,

    #[nwg_control(text : "hold up", size : (145, 150), position : (30, 35), background_color : Some([255, 255, 224]), font : build_font().as_ref(), flags : "MULTI_LINE|VISIBLE")]
    text : nwg::RichLabel,

    #[nwg_control]
    #[nwg_events(OnNotice : [Self::display_result])]
    request_notice : nwg::Notice, 
    request_result : RefCell<Option<thread::JoinHandle<Box<str>>>>,

    #[nwg_control(text : "OK", size : (80, 25), position : (160, 220))]
    #[nwg_events(OnButtonClick : [Self::exit])]
    hello_button : nwg::Button, */
struct AskAhmed{
    app:gtk::Application,
    label:gtk::Label,
    rx:std::sync::mpsc::Receiver<String>
}
fn get_ahmed() -> String{
    let path = std::env::current_exe();
    let ahmed_home = path.unwrap().parent().unwrap().as_os_str().to_str().unwrap().to_string();
    //gtk::gio::File::read(ahmed_home+"/Ahmed.png")
    ahmed_home+"/Ahmed.png"
}
thread_local!{static GLOBAL:RefCell<Option<AskAhmed>>=RefCell::new(None)}
fn build_ui(app: &gtk::Application, labelm: &gtk::Label){
    
    let fix_box = gtk::Fixed::builder().build();
    let ahmed = gtk::Image::builder().file(&get_ahmed()).build();
    let ok = gtk::Button::builder().label("ok").build();
    unsafe{
        (*labelm.data_ptr()).set_size_request(145,150);
        fix_box.put(&(*labelm.data_ptr()), 30.0, 35.0);
    }

    ahmed.set_size_request(140, 150);
    ok.set_size_request(80, 25);
    

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
fn build_ui_wrapper(){
    GLOBAL.with_borrow(|aa|{build_ui(&aa.unwrap().app, &aa.unwrap().label);});
}

fn main() -> gtk::glib::ExitCode{
    let app = gtk::Application::builder().application_id(APP_ID).build();
    let label = gtk::Label::builder().label("hold up").build();
    let (tx,rx) = std::sync::mpsc::channel();

    let ahhh = AskAhmed{
        app,label,rx
    };

    app.connect_closure("activate", true, closure_local!(|app|{build_ui_wrapper();}));
    GLOBAL.with(|global|{*global.borrow_mut() = Some(ahhh)});
    breh(tx);
    app.run()
}
