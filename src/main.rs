use local_ip_address::local_ip;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn update_server(ip: &str) {
    let path = Path::new("C:\\DigiSat\\SuiteG6\\Servidor\\ConfiguracaoServer.xml");
    let mut file = File::open(path).expect("Falha ao abrir ConfiguracaoServer.xml");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Falha ao ler arquivo ConfiguracaoServidor.xml");

    let mut lines: Vec<&str> = contents.split("\r\n").collect();

    let newip = format!("\t<Ip>{ip}</Ip>");

    lines[3] = &newip;

    let data = lines.join("\r\n");

    let mut file = File::create(path).expect("Falha ao abrir ConfiguracaoServer.xml");
    file.write_all(data.as_bytes())
        .expect("Falha ao salvar arquivo ConfiguracaoServidor.xml");

    println!("Atualizado arquivo ConfiguracaoServidor.xml")
}

fn update_sistema(ip: &str) {
    let path = Path::new("C:\\DigiSat\\SuiteG6\\Sistema\\ConfiguracaoClient.xml");
    let mut file = File::open(path).expect("Falha ao abrir ConfiguracaoClient.xml");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Falha ao ler arquivo ConfiguracaoClient.xml");

    let mut lines: Vec<&str> = contents.split("\r\n").collect();

    let newip = format!("\t<Ip>{ip}</Ip>");

    lines[68] = &newip;

    let data = lines.join("\r\n");

    let mut file = File::create(path).expect("Falha ao abrir ConfiguracaoClient.xml");
    file.write_all(data.as_bytes())
        .expect("Falha ao salvar arquivo ConfiguracaoClient.xml");

    println!("Atualizado arquivo ConfiguracaoClient.xml")
}

fn main() {
    let ip = local_ip().unwrap();
    let ipstr = ip.to_string();
    println!("IP local: {}", ipstr);

    update_server(&ipstr);
    update_sistema(&ipstr);
}
