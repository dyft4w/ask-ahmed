use crate::util;
use rascii_art;


const TEXT_X:u32=32;
const TEXT_Y:u32=32;

pub struct Canvas{
    x:u32,
    y:u32,
}
impl Canvas{
    pub fn go_to(&mut self, x:u32, y:u32){
        if y>self.y{
            print!("\x1B[{}B", y-self.y) //move down
        }
        if y<self.y{
            print!("\x1B[{}A", self.y-y) //move up
        }



        if x>self.x{
            print!("\x1B[{}C", x-self.x) //move right
        }
        if x<self.x{
            print!("\x1B[{}D", self.x-x) //move left
        }
        self.x=x;
        self.y=y;
        //print!("\x1B[{};{}H", self.lines-y, x);
        //self.x=x;
        //self.y=y;
    }
    pub fn print_line(&mut self, text:&str){
        self.x+=text.len() as u32;
        print!("{}", text);
    }
    pub fn ln(&mut self){
        self.y+=1;
        self.x=0;
        print!("\n");
    }
    pub fn new(lines:u32) -> Canvas{
        print!("{}\x1B[{}A", "\n".repeat((lines+1).try_into().unwrap()), lines);
        Canvas{
            x:0,
            y:0,
        }
    }
}
pub fn draw_image(cv: &mut Canvas, path:&str, x:u32, y:u32){
    let mut buf=String::new();
    cv.go_to(x,y);

    rascii_art::render_to(
        path, 
        &mut buf, 
        &rascii_art::RenderOptions::new()
            .width(60)
            .charset(&[" ", "░", "▒", "▓", "█"])
            .colored(true)
    ).unwrap();
    let x = cv.x;
    let mut y = cv.y;
    for i in buf.lines().into_iter(){
        cv.go_to(x,y);
        cv.print_line(i);
        cv.ln();
        y+=1;
    }
}
pub fn draw_bubble(cv: &mut Canvas, width:u32, height:u32){
    let (x,y) = (cv.x,cv.y);
    let (c1,c2,c3,c4) = ("╭", "╮", "╯","╰");
    let (lh,lv) = ("─", "│");

    cv.go_to(x,y);
    cv.print_line(c1);
    cv.print_line(&lh.repeat((width-3).try_into().unwrap()));
    cv.print_line(c2);
    for i in 1..height{
        cv.go_to(x,y+i);
        cv.print_line(lv);
        cv.go_to(x+width,y+i);
        cv.print_line(lv);
    }
    cv.go_to(x,y+height);
    cv.print_line(c4);
    cv.print_line(&lh.repeat((width-3).try_into().unwrap()));
    cv.print_line(c3);
    cv.ln();


    //print!("{}",bubble);
}

pub fn update_label(){
    util::GLOBAL.with_borrow_mut(|global|{
        if let Some((recv, _, opt)) = global{
            match opt{
                Some(cv) =>{
                    cv.go_to(TEXT_X,TEXT_Y);
                    cv.print_line(&recv.recv().unwrap());
                },
                None => {}
            }
        }
    });
}
pub fn build_ui(_app:&gtk4::Application, message:&str){
    util::GLOBAL.with_borrow_mut(|global|{
        if let Some((_,_,Some(cv))) = global {
            cv.go_to(0,15);
            draw_bubble(cv, 50, 10);
            cv.go_to(10, 18);
            cv.print_line(message);
            cv.go_to(88, 0);
            cv.print_line("ahmed");
            draw_image(cv, &util::get_ahmed(), 60, 1);
        }
    });
}