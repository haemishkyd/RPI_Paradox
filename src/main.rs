use std::{
    env,
    process,
    time::Duration
};

use serialport::{available_ports, SerialPortType};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER:&str = "mqtt://192.168.1.100:1883";
const DFLT_CLIENT:&str = "rust_publish";
const DFLT_TOPICS:&[&str] = &["homeassistant/sensor/alarm/zone/1", "homeassistant/sensor/alarm/zone/2"];
// Define the qos.
const QOS:i32 = 1;

fn main(){
    let host = env::args().nth(1).unwrap_or_else(||
        DFLT_BROKER.to_string()
    );

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
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
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Create a message and publish it.
    // Publish message to 'test' and 'hello' topics.
    let content =  "1".to_string();
    let msg = mqtt::Message::new("test", "Hello world!",QOS);
    let tok = cli.publish(msg);

    if let Err(e) = tok {
       println!("Error sending message: {:?}", e);
    }
    
    // Disconnect from the broker.
    let tok = cli.disconnect(None);
    println!("Disconnect from the broker");
    tok.unwrap();
}
