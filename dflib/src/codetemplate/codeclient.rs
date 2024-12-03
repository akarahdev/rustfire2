#![allow(unused)]

use std::io::{Read, Write};
use std::net::TcpStream;
use base64::alphabet::STANDARD;
use base64::Engine;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use flate2::Compression;
use flate2::write::GzEncoder;
use tungstenite::{accept, connect, ClientRequestBuilder, Message};
use tungstenite::http::Uri;
use crate::codetemplate::template::{Template, TemplateBlock};

pub fn send_to_code_client(
    templates: Vec<Template>
) {
    let request =
        ClientRequestBuilder::new(Uri::from_static("ws://localhost:31375"));
    let mut websocket = connect(request).unwrap();
    websocket.0.send(Message::Text("scopes inventory movement read_plot write_code clear_plot".into()));
    loop {
        let msg = websocket.0.read().unwrap();
        println!("msg: {:?}", msg);

        if let Message::Text(text) = msg {
            if text == "auth" {
                println!("Authorized!");
                websocket.0.send(Message::Text("clear".into()));

                for template in &templates {
                    let json_encoded = serde_json::to_string(&template).unwrap();
                    let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::default());
                    gzip_encoder.write_all(json_encoded.as_bytes()).unwrap();
                    let base64_encoded = GeneralPurpose::new(
                        &STANDARD,
                        GeneralPurposeConfig::new()
                    ).encode(gzip_encoder.finish().unwrap());

                    println!("Encoded: {}", &json_encoded);
                    println!("Base64: {}", &base64_encoded);

                    websocket.0.send(Message::Text(format!("place {}", base64_encoded))).unwrap();
                }
                websocket.0.send(Message::Text("place go".to_string())).unwrap();

            }
        }
    }
}