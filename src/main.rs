// simple port scanner using rust


use std::net::{TcpStream,IpAddr};
use std::env;
use std::str::FromStr;
use std::process;
use std::thread;
use std::io::{self, Write};

const MAX_PORT:u16 = 65535; 

struct Params{
    port: Option<u16>,
    ipArrd: IpAddr
}

impl  Params{

    fn validate_port(port: &str) -> Result<Option<u16>, &'static str>{
        if port.chars().nth(0).unwrap()=='-'{
            return Ok(None)
        }

        for chr in port.chars(){
            if !chr.is_numeric() {
                return Err("invalid port.");
            }
        }

        return Ok(Some(port.parse::<u16>().unwrap()));

    }

    fn validate_ipAddr(ip: &str) -> Result<IpAddr,&'static str> {
        if ip.chars().nth(0).unwrap()=='-'{
            return Err("Ip address cannot be empty");
        }

        match IpAddr::from_str(&ip) {
            Ok(vipaddr) => return Ok(vipaddr),
            Err(_) => return Err("not a valid ip address")
        };

    }

    pub fn new(args: &Vec<String>) -> Result<Params, &'static str>{
        if args.len() < 3{
            return Err("less arguments passed");
        } else if args.len() > 5 {
            return Err("Too many arguments passed");
        }

        let idx_port : usize = args.iter().position(|r| r =="-p").unwrap();
        let mut initPortVal: &str = &args[idx_port+1].clone();

        let idx_ip : usize = args.iter().position(|r| r == "-ip").unwrap();
        let mut initIpAddrVal: &str = &args[idx_ip+1].clone(); 
        
        // clean port
        let port: Option<u16> = match self::Params::validate_port(initPortVal){
            Ok(port_val) => port_val,
            Err(err_val) => return Err(err_val)
        };

        // clean ip 

        let ipAddr: IpAddr = match self::Params::validate_ipAddr(initIpAddrVal) {
            Ok(ip_val) => ip_val,
            Err(err_val) => return Err(err_val)
        };

        Ok(
            Params{
                port: port,
                ipArrd: ipAddr
            }
        )
    }

    pub fn info(&self){
        println!("Port: {}, IpArrdess: {}", self.port.unwrap(),self.ipArrd);
    }

}

fn connect(port: u16,ipAddr: IpAddr){
    match TcpStream::connect((ipAddr,port)) {
        Ok(_) =>{
            println!("Port {} Open",port);
        },
        Err(_) => {}
    }
}

fn scan(params: &Params){

    // None -> All ports

    if !params.port.is_none() {
           connect(params.port.unwrap(),params.ipArrd);
    }
    else{
        let mut curr_port: u16 = 1;

        loop{
            let port = curr_port.clone();
            let ipAddr = params.ipArrd.clone();

            thread::spawn(move || {
                connect(port,ipAddr);
            });

            if curr_port >= MAX_PORT {
                break;
            }

            curr_port +=1;

        }
    }

}



fn main(){
    let args: Vec<_> = env::args().collect();

    let parsedParam = Params::new(&args).unwrap_or_else(
        |err| {
            eprintln!("{}",err);
            process::exit(0);
        } 
    );

    scan(&parsedParam);

}