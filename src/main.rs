extern crate clap;
use reqwest;
use std::collections::HashMap;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;
use clap::{Arg, App};
use console;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {

        let matches = App::new("Clcicd")
        .version("0.1.0")
        .author("Jon Barker <jonathan.t.barker@gmail.com.com>")
        .about("command line tool for repeated api endpoint actions written in Rust")
        .arg(Arg::with_name("frequency")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("how often do you want to deploy canary (every X minutes)?"))
        .arg(Arg::with_name("user")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("what is your userid?"))  //in my example this goes in the post body
        .get_matches();
    let frequency = matches.value_of("frequency").unwrap();
    let frequency_uint32: u32 = frequency.parse().unwrap();
    let username = matches.value_of("user").unwrap();
    let userstring: String = username.parse().unwrap();
    println!("{}",frequency_uint32);
    println!("Your canary will deploy every {} minutes, congratulations!", frequency_uint32);

    let mut scheduler = Scheduler::new();
    scheduler.every(frequency_uint32.minutes()).plus(30.seconds()).run(move ||{
    let mut map = HashMap::new();
    let userstring = userstring.clone();
    map.insert("user", &userstring);
    let client = reqwest::Client::new();
    client.post("https://APIURLENDPOINTHERE/apifunctionname")
    .json(&map)
    .send();
    println!("Canary deployed!");
}
);

loop {
    scheduler.run_pending();
    thread::sleep(Duration::from_millis(100));
}

// fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
//     let bar = match quiet_mode {
//         true => ProgressBar::hidden(),
//         false => {
//             match length {
//                 Some(len) => ProgressBar::new(len),
//                 None => ProgressBar::new_spinner(),
//             }
//         }
//     };

//     bar.set_message(msg);
//     match length.is_some() {
//         true => bar
//             .set_style(ProgressStyle::default_bar()
//                 .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
//                 .progress_chars("=> ")),
//         false => bar.set_style(ProgressStyle::default_spinner()),
//     };

//     bar
// }

// fn download(target: &str, quiet_mode: bool) -> Result<(), Box<::std::error::Error>> {

//     // parse url
//     let url = parse_url(target)?;
//     let client = Client::new().unwrap();
//     let mut resp = client.get(url)?
//         .send()
//         .unwrap();
//     print(format!("HTTP request sent... {}",
//                   style(format!("{}", resp.status())).green()),
//           quiet_mode);
//     if resp.status().is_success() {

//         let headers = resp.headers().clone();
//         let ct_len = headers.get::<ContentLength>().map(|ct_len| **ct_len);

//         let ct_type = headers.get::<ContentType>().unwrap();

//         match ct_len {
//             Some(len) => {
//                 print(format!("Length: {} ({})",
//                       style(len).green(),
//                       style(format!("{}", HumanBytes(len))).red()),
//                     quiet_mode);
//             },
//             None => {
//                 print(format!("Length: {}", style("unknown").red()), quiet_mode); 
//             },
//         }

//         print(format!("Type: {}", style(ct_type).green()), quiet_mode);

//         let fname = target.split("/").last().unwrap();

//         print(format!("Saving to: {}", style(fname).green()), quiet_mode);

//         let chunk_size = match ct_len {
//             Some(x) => x as usize / 99,
//             None => 1024usize, // default chunk size
//         };

//         let mut buf = Vec::new();

//         let bar = create_progress_bar(quiet_mode, fname, ct_len);

//         loop {
//             let mut buffer = vec![0; chunk_size];
//             let bcount = resp.read(&mut buffer[..]).unwrap();
//             buffer.truncate(bcount);
//             if !buffer.is_empty() {
//                 buf.extend(buffer.into_boxed_slice()
//                                .into_vec()
//                                .iter()
//                                .cloned());
//                 bar.inc(bcount as u64);
//             } else {
//                 break;
//             }
//         }

//         bar.finish();

//         save_to_file(&mut buf, fname)?;
//     }

//     Ok(())

// }

} //end main



