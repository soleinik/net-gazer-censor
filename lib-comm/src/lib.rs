
#[allow(dead_code)]
#[allow(unused_imports)]
mod messaging_generated;
use log::{info, error};
use messaging_generated::*;


use net_gazer_core::{CoreMessage, CoreSender, CoreReceiver};
use std::time::Instant;
use std::thread;
use async_std::task;


pub type CommRxChannel = CoreReceiver;
pub type CommTxChannel = CoreSender;

pub fn start(rx:CommRxChannel, conf:& lib_data::OptConf ){

    let url = conf.reporting_url.clone().unwrap();
    thread::spawn(move || {
        info!("Comm thread started...");

        let instance_id = "hcoded-instance-id";

        let mut builder = Builder::new(instance_id);

        loop{
            if let Ok(msg) = rx.recv(){
                let u = url.clone();
                let data = builder.create_message(&msg);
                task::spawn(async move {
                    let resp = ureq::post(&u)
                        //.set("X-My-Header", "Secret")
                        .send_bytes(&data);

                    if resp.is_err(){
                        error!("[{}]\t{}",u, resp.err().unwrap());
                    }
                });
            }
        }
    });
}


pub struct Builder<'fbb>{
    seq: u64,
    sensor_id:&'fbb str,
    start_time:Instant,

    bldr: flatbuffers::FlatBufferBuilder<'fbb>,
}


impl <'fbb> Builder<'fbb> {

    pub fn new(sensor_id:&'fbb str) -> Self{
        Builder { 
            seq:0,
            sensor_id,
            start_time:Instant::now(), 
            bldr: flatbuffers::FlatBufferBuilder::new()
        }
    }

    fn reset(&mut self){
        self.bldr.reset();
        self.seq += 1; //FIXME: overflow
    }

    pub fn create_message(&mut self, comm_msg:&CoreMessage) -> Vec<u8>{
        let mut msg = Vec::<u8>::new();
        self.reset();

        let mut args = MessageArgs::default();
        args.seq = self.seq;
        args.uptime = self.start_time.elapsed().as_secs();

        args.sensor_id = Some(self.bldr.create_string(self.sensor_id));

        args.ptype = comm_msg.0;
        args.payload = Some(self.bldr.create_vector(&comm_msg.1));

        let message_offset = Message::create(&mut self.bldr, &args);

        finish_message_buffer(&mut self.bldr, message_offset);
        let finished_data = self.bldr.finished_data();
        msg.extend_from_slice(finished_data);
        msg
    }
}