use colored::Colorize;
use interfaces::Interface;

fn main() {
    let mut interfaces = Interface::get_all().expect("Error: Couldn't get interfaces.");
    interfaces.sort_by(|a, b| a.name.cmp(&b.name));

    let mut if_iter = interfaces.iter();
    while let Some(interface) = if_iter.next() {
        println!("{}",interface.name.to_string().blue());
        
        let mut addr_iter = interface.addresses.iter();
        let mut idx: usize = 0;
        while let Some(address) = addr_iter.next() {
            if let Some(addr) = address.addr {
                println!(" {}─┬{} {}\n {} ╰──NM {}",["├","╰"][(idx == interface.addresses.len() - 1) as usize], address.kind, addr.ip().to_string().green(), ["│"," "][(idx == interface.addresses.len() - 1) as usize], address.mask.unwrap().ip().to_string().purple());
            }
            idx += 1;
        }
    }
}