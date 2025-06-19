use std::{
    env,
    cell::RefCell,
    sync::mpsc::{Receiver, Sender},
    thread::{self, sleep},
    time::Duration,
};
use gtk4 as gtk;
use serde_json::Value;


thread_local!{
    pub static GLOBAL:RefCell<Option<(Receiver<String>, Option<gtk::Label>, Option<crate::tui::Canvas>)>>=RefCell::new(None);
    pub static INIT:RefCell<bool> = RefCell::new(false);
}


pub fn get_local() -> std::path::PathBuf{
    // if we're on windows, everything (binary, apikey and pngs) is in C:\Program Files\AskAhmed
    // if we're on linux, we install locally (since context menu configs are all local anyways), and files are a bit more dispersed
    // there are two common options to arrange data:
    //      1: have binary in .local/bin and have files in .local/share/ask-ahmed
    //      2: put the binary in .local/share/ask-ahmed and symlink it to ./local/bin (what we do)
    // so the code below returns where the symlink comes from, and we don't have to worry about #cfg for linux and windows
    env::current_exe().unwrap()
}
pub fn get_ahmed() -> String{
    //inc support for other images
    let mut path = get_local();
    path.set_file_name("Ahmed.png");
    path.to_str().unwrap().to_string()
}




pub fn breh<T:Fn()>(tx:Sender<String>, file: String, callback:T)
where 
    T:Send + 'static
{
    println!("{}", file);
    thread::spawn(move ||{
        let task = ||
        {
            let upload_form = {
                let option = reqwest::blocking::multipart::Form::new ().file("file", file).ok();

                if let Some(form) = option{
                    form
                }
                else {
                    return Some(Box::from("the fuck you want"));
                }
            };

            let mut path = get_local();
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
                if status == "completed"
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
        gtk::glib::idle_add_once(move||{callback()});
    });

}
   