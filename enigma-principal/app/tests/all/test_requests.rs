use enigma_crypto::KeyPair;
use enigma_principal_app::{
    cli::{self, options::Opt},
    esgx,
};
use enigma_tools_m::primitives::km_primitives::{PrincipalMessage, PrincipalMessageType};
use jsonrpc_client_core::Error;
use jsonrpc_client_http::{HttpTransport, HttpHandle};
use rustc_hex::ToHex;
use std::{
    thread::{self, JoinHandle},
    time,
};
use structopt::StructOpt;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub data: String,
    pub sig: String,
}

jsonrpc_client!(pub struct KeyManagementClient {
    pub fn getStateKeys(&mut self, data: &str, sig: &str) -> RpcRequest<Response>;
});



#[test]
fn test_bad_hex() {
    let mut client = get_client(3040);

    let res = client.getStateKeys("garbage", "More garbage").call();
    println!("{:?}", res);
}

#[test]
fn test_bad_sig() {
    let mut client = get_client(3040);
    let (msg, _) = generate_msg();
    let sig = [255; 65];
    let res = client.getStateKeys(&msg.to_hex(), &sig.to_hex()).call();
    println!("{:?}", res);
}

#[test]
fn test_zero_sig() {
    let mut client = get_client(3040);
    let (msg, _) = generate_msg();
    let sig = [0; 65];
    let res = client.getStateKeys(&msg.to_hex(), &sig.to_hex()).call();
    println!("{:?}", res);
}

#[test]
fn test_bad_sig_len() {
    let mut client = get_client(3040);
    let (msg, _) = generate_msg();
    let sig = [255; 32];
    let res = client.getStateKeys(&msg.to_hex(), &sig.to_hex()).call();
    println!("{:?}", res);
}

#[test]
fn test_bad_msg() {
    let mut client = get_client(3040);
    let (mut msg, sig) = generate_msg();
    msg[10..20].copy_from_slice(&[255; 10]);
    let res = client.getStateKeys(&msg.to_hex(), &sig.to_hex()).call();
    println!("{:?}", res);
}



#[test]
#[ignore]
fn test_spam_success() {
//    let opts = generate_opts();
//    let _principal_handle =  run_principal(opts);
    thread::sleep(time::Duration::from_secs(5));
    let handles = send_requests(2000);
    let handles = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();
    let failed_amounts = handles.iter().fold(0, |amount, result| {
        if result.is_err() {
            println!("{:?}", result);
            amount + 1
        } else {
            amount
        }
    });
    println!("failed: {} times", failed_amounts);
//    principal_handle.join().unwrap();
}

fn sign_whatever(msg: &[u8]) -> [u8; 65] {
    let keys = KeyPair::new().unwrap();
    keys.sign(msg).unwrap()
}

fn get_pubkey() -> [u8; 64] { KeyPair::new().unwrap().get_pubkey() }

fn generate_msg() -> (Vec<u8>, [u8; 65]) {
    let pubkey = get_pubkey();
    let t = PrincipalMessageType::Request(None);
    let msg = PrincipalMessage::new(t, pubkey).unwrap();
    let to_sign = msg.to_sign().unwrap();
    let msg = msg.into_message().unwrap();
    let sig = sign_whatever(&to_sign);

    (msg, sig)
}

fn send_requests(n: usize) -> Vec<JoinHandle<Result<Response, Error>>> {
    let mut handles = Vec::with_capacity(n);
    for i in 0..n {
        if i % 5 == 0 {
            thread::sleep(time::Duration::from_millis(30));
        }
        let handle = thread::spawn(|| {
            let mut client = get_client(3040);

            let (msg, sig) = generate_msg();
            client.getStateKeys(&msg.to_hex(), &sig.to_hex()).call()
        });
        handles.push(handle);
    }
    handles
}


fn get_client(port: u16) -> KeyManagementClient<HttpHandle> {
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle(&format!("http://127.0.0.1:{}",port)).unwrap();

    KeyManagementClient::new(transport_handle)
}





#[allow(dead_code)]
fn generate_opts() -> Opt {
    let args = ["test", "--mine", "5"];
    let clap = Opt::clap().get_matches_from(&args);
    Opt::from_clap(&clap)
}

#[allow(dead_code)]
pub fn run_principal(opt: Opt) -> JoinHandle<()> {
    thread::spawn(move || {
        let enclave = esgx::general::init_enclave_wrapper().expect("[-] Init Enclave Failed");
        let eid = enclave.geteid();
        cli::app::start(eid, opt).unwrap();
    })
}