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
use valence_nbt::Tag::String;
use crate::api::COMPILER_CONFIG;
use crate::codetemplate::template::{Template, TemplateBlock};

pub fn send_to_code_client(
    mut templates: Vec<Template>
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

                templates.push(Template { blocks: vec![] });
                let mut builder = vec![];
                let mut index = 0;
                for template in &templates {
                    index += 1;
                    println!("index: {}", index);
                    println!("builder: {}", builder.len());
                    println!("next: {}", template.blocks.len());
                    println!("cap: {}", COMPILER_CONFIG.plot.size.max_blocks());
                    if ((builder.len() + template.blocks.len()) >= COMPILER_CONFIG.plot.size.max_blocks())
                        || index >= templates.len()-1 {

                        let json_encoded = serde_json::to_string(
                            &Template { blocks: builder }
                        ).unwrap();

                        let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::default());
                        gzip_encoder.write_all(json_encoded.as_bytes()).unwrap();
                        let base64_encoded = GeneralPurpose::new(
                            &STANDARD,
                            GeneralPurposeConfig::new()
                        ).encode(gzip_encoder.finish().unwrap());

                        println!("Encoded: {}", &json_encoded);
                        println!("Base64: {}", &base64_encoded);

                        websocket.0.send(Message::Text(format!("place {}", base64_encoded))).unwrap();


                        builder = template.blocks.clone();
                    } else {
                        builder.extend_from_slice(&template.blocks);
                    }
                }

                websocket.0.send(Message::Text("place go".to_string())).unwrap();
            }
        }
    }
}