use std::{
    env,
    process,
    time::Duration
};

use syslog::{Facility, Error};
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};
use serialport::{available_ports, SerialPortType};
use std::io::Write;
use std::io;

extern crate paho_mqtt as mqtt;

const DFLT_BROKER:&str = "mqtt://192.168.1.100:1883";
const DFLT_CLIENT:&str = "rust_publish";
const DFLT_TOPICS:[&str;10] = ["homeassistant/sensor/alarm/armed", 
                              "homeassistant/sensor/alarm/zone/1", 
                              "homeassistant/sensor/alarm/zone/2",
                              "homeassistant/sensor/alarm/zone/3",
                              "homeassistant/sensor/alarm/zone/4",
                              "homeassistant/sensor/alarm/zone/5",
                              "homeassistant/sensor/alarm/zone/6",
                              "homeassistant/sensor/alarm/zone/7",
                              "homeassistant/sensor/alarm/zone/8",
                              "homeassistant/sensor/alarm/zone/9"];
// Define the qos.
const QOS:i32 = 1;
const port_name:&str = "/dev/ttyAMA0";
const baud_rate:u32 = 9600;


fn main(){
    let mut message_started:bool = false;

    syslog::init(Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("Paradox Terminal"));

    log::debug!("Application Start");
    
    let host = env::args().nth(1).unwrap_or_else(||
        DFLT_BROKER.to_string()
    );

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        log::debug!("{}", p.port_name);
    }

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();
    
    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        log::error!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Define the set of options for the connection.
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .user_name("haemish")
        .password("test_pw")
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail.
    if let Err(e) = cli.connect(conn_opts) {
        log::error!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    match port {
        Ok(mut port) => {            
            let mut final_buf: Vec<u8> = vec![0; 100];            
            log::debug!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                let mut serial_buf: Vec<u8> = vec![0; 1];
                match port.read(&mut serial_buf[..]) {
                    Ok(t) => { if t>0 {
                            for n in 0..t{
                                match serial_buf[n] == 0xE0 {
                                    true => {message_started = true;println!("Message Started");},
                                    _ => {}
                                };
                                match message_started == true {
                                    true => {final_buf.push(serial_buf[n]);},
                                    _ => {}
                                };
                            }                          
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),                    
                }
                if final_buf.len() >= 36{
                    println!("{:x?}",final_buf);
                    log::debug!("Full message recieved!");
                    if final_buf[0] == 0xE0{
                        if (final_buf[7] == 0x02) && (final_buf[8] == 0x0C){
                            log::debug!("Alarm Armed");
                        }
                        if (final_buf[7] == 0x02) && (final_buf[8] == 0x0B){
                            log::debug!("Alarm Disarmed");
                        }
                        if final_buf[7] == 0x00 {
                            send_to_ha(final_buf[8],"{\"state\":0}".to_string(), cli.clone());
                            if final_buf[8] == 0x01 {                                
                                log::debug!("Zone 1 Deactivated");
                            }
                            if final_buf[8] == 0x02{
                                log::debug!("Zone 2 Deactivated");
                            }
                            if final_buf[8] == 0x03{
                                log::debug!("Zone 3 Deactivated");
                            }
                            if final_buf[8] == 0x04{
                                log::debug!("Zone 4 Deactivated");
                            }
                            if final_buf[8] == 0x05{
                                log::debug!("Zone 5 Deactivated");
                            }
                            if final_buf[8] == 0x06{
                                log::debug!("Zone 6 Deactivated");
                            }
                            if final_buf[8] == 0x07{
                                log::debug!("Zone 7 Deactivated");
                            }
                            if final_buf[8] == 0x08{
                                log::debug!("Zone 8 Deactivated");
                            }
                            if final_buf[8] == 0x09{
                                log::debug!("Zone 9 Deactivated");
                            }
                        } 
                        if final_buf[7] == 0x01 {
                            send_to_ha(final_buf[8],"{\"state\":1}".to_string(), cli.clone());
                            if final_buf[8] == 0x01{                                
                                log::debug!("Zone 1 Activated");
                            }
                            if final_buf[8] == 0x02{
                                log::debug!("Zone 2 Activated");
                            }
                            if final_buf[8] == 0x03{
                                log::debug!("Zone 3 Activated");
                            }
                            if final_buf[8] == 0x04{
                                log::debug!("Zone 4 Activated");
                            }
                            if final_buf[8] == 0x05{
                                log::debug!("Zone 5 Activated");
                            }
                            if final_buf[8] == 0x06{
                                log::debug!("Zone 6 Activated");
                            }
                            if final_buf[8] == 0x07{
                                log::debug!("Zone 7 Activated");
                            }
                            if final_buf[8] == 0x08{
                                log::debug!("Zone 8 Activated");
                            }
                            if final_buf[8] == 0x09{
                                log::debug!("Zone 9 Activated");
                            }
                        }
                    }                   
                    final_buf.clear();
                    message_started = false;
                }
            }
        }
        Err(e) => {
            log::error!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
     
    // Disconnect from the broker.
    let tok = cli.disconnect(None);
    log::debug!("Disconnect from the broker");
    tok.unwrap();
}

fn send_to_ha(topic: u8, content: String, cli: mqtt::Client){
    // Create a message and publish it.
    // Publish message to 'test' and 'hello' topics.
    let content =  content.to_string();
    let msg = mqtt::Message::new(DFLT_TOPICS[topic as usize], content, QOS);
    let tok = cli.publish(msg);

    if let Err(e) = tok {
       println!("Error sending message: {:?}", e);
    }
}