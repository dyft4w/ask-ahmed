use crate::util;
use std:: {
    cell::RefCell,
    sync::mpsc::Receiver,
    f64::consts::PI
};

use gtk4::{
    self as gtk, 
    prelude::*,
};


pub fn update_label(){
    util::GLOBAL.with_borrow(|global|{
            match global{
                Some((rx, Some(label), None)) => label.set_label(&rx.recv().unwrap()),
                _ => println!("asd")
            }
    });
}


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
pub fn build_ui(app: &gtk::Application, message:&str){
    
    let fix_box = gtk::Fixed::builder().build();
    let mut ahmed_png = util::get_local();
    ahmed_png.set_file_name("Ahmed.png");
    let ahmed = gtk::Image::builder().file(ahmed_png.as_os_str().to_str().unwrap()).build();
    let ok = gtk::Button::builder().label("ok").build();
    let area = gtk::DrawingArea::builder().build();
    let overlay = gtk::Overlay::builder().child(&area).build();

    util::INIT.with_borrow_mut(|init|{
        *init = true;
    });


    draw_bubble(&area);
    overlay.set_valign(gtk::Align::Start);
    overlay.set_halign(gtk::Align::Start);

    util::GLOBAL.with_borrow(|global|{
        match global{
            Some((_,Some(label),_)) => {
                label.set_margin_start(35);
                label.set_margin_top(45);
                label.set_label(message);
                label.set_valign(gtk::Align::Start);
                label.set_halign(gtk::Align::Start);
                overlay.add_overlay(label);
            }
            _ => println!("skibidi toilet")
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

