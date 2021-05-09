use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::io::{self};

const MAIL_API: &str = "https://www.1secmail.com/api/v1/?action=";

#[derive(Serialize, Deserialize)]
struct MailList {
    logins: Vec<String>,
    domains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MailboxItem {
    id: i64,
    from: String,
    subject: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mailbox {
    items: Vec<MailboxItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: i64,
    from: String,
    subject: String,
    date: String,
    attachments: Vec<Value>,
    body: String,
    textBody: String,
    htmlBody: String,
}

impl MailList {
    fn new() -> MailList {
        MailList {
            logins: Vec::new(),
            domains: Vec::new(),
        }
    }
}

impl Mailbox {
    fn new() -> Mailbox {
        Mailbox { items: Vec::new() }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    let mut error: bool = true;

    let mut count: usize = 0;

    while error{
        println!("Enter the number of random mail adresses:");

        io::stdin().read_line(&mut input)?;

        count = match input.trim().parse() {
            Ok(n) => {
                input = "".to_string();
                error = false;
                n
            },

            Err(_) => {
                error = true;
                input = "".to_string();
                0
            }
        };
    }

    error = true;

    let mail_list = get_random_mail_adress(count);

    let mut selected_mail_address = String::new();

    let mut selected_login = String::new();
    let mut selected_domain = String::new();

    while error {
        println!("Select the index of a mail address");
        
        input = "".to_string();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(n) => {
                if n < mail_list.logins.len() {
                    selected_login = String::from(&mail_list.logins[n]);
                    selected_domain = String::from(&mail_list.domains[n]);
                    selected_mail_address = format!("{}@{}", selected_login, selected_domain);
                    error = false;
                    n
                }else{
                    error = true;
                    0
                }
                
            },

            Err(_) => {
                error = true;
                0
            }
        };

    }

    println!("\nSelected mail address: {}\n", selected_mail_address);

    println!("Actions:\nCheck mailbox - 'c', Read message - 'r', Quit - 'q'\n");

    let mut quit: bool = false;

    error = true;

    while !quit{
        println!("Enter action:");

        input = "".to_string();
        io::stdin().read_line(&mut input)?;

        match &input[..1]{
            "c" =>{
                let mailbox = check_mailbox(&selected_login, &selected_domain);

                println!("\nMessage count: {}", mailbox.items.len());

                for item in mailbox.items.iter(){
                    println!("\nMessage id: {}\nFrom: {}\nSubject: {}\nDate: {}\n\n", item.id, item.from, item.subject, item.date);
                }
            },

            "r" => {
                while error{
                    println!("Enter message id:");

                    input = "".to_string();
                    io::stdin().read_line(&mut input)?;

                    match input.trim().parse::<usize>() {
                        Ok(n) => {
                                error = false;

                                let message: String = check_message(&selected_login, &selected_domain, n as i64);
                                println!("{}", message);
                        },
            
                        Err(_) => {
                            error = true;
                        }
                    };
                }
            },

            "q" => quit = true,
            _ => {}
        }
    }

    Ok(())
}

fn get_random_mail_adress(count: usize) -> MailList {
    let response =
        match reqwest::blocking::get(format!("{}genRandomMailbox&count={}", MAIL_API, count)) {
            Ok(r) => match r.text() {
                Ok(s) => s,
                Err(_) => panic!("Response text error!"),
            },
            Err(_) => panic!("GET request error!"),
        };

    let json_response: Value = match serde_json::from_str(&response) {
        Ok(s) => s,
        Err(_) => panic!("JSON error!"),
    };

    let mut mail_list = MailList::new();

    println!("\nRandom mail adresses:\n");
    for i in 0..count {
        let mail = &json_response.as_array().unwrap()[i];
        let mail = mail.to_string();

        let len = mail.len();

        let mail = &mail[1..len-1];

        println!("{} - {}", i, mail);

        let tokens: Vec<&str> = mail.split("@").collect();

        mail_list.logins.push(String::from(tokens[0]));
        mail_list.domains.push(String::from(tokens[1]));
    }
    println!();

    mail_list
}

fn check_mailbox(login: &str, domain: &str) -> Mailbox {
    let response = match reqwest::blocking::get(format!(
        "{}getMessages&login={}&domain={}",
        MAIL_API, login, domain
    )) {
        Ok(r) => match r.text() {
            Ok(s) => s,
            Err(_) => panic!("Response text error!"),
        },
        Err(_) => panic!("GET request error!"),
    };

    let json_response: Value = match serde_json::from_str(&response) {
        Ok(s) => s,
        Err(_) => panic!("JSON error!"),
    };

    let mut mailbox = Mailbox::new();
    let item_array = json_response.as_array().unwrap();

    for i in 0..item_array.len() {
        let item = item_array[i].as_object().unwrap();

        mailbox.items.push(MailboxItem {
            id: item["id"].as_i64().unwrap(),
            from: String::from(item["from"].as_str().unwrap()),
            subject: String::from(item["subject"].as_str().unwrap()),
            date: String::from(item["date"].as_str().unwrap()),
        });
    }

    mailbox
}

fn check_message(login: &str, domain: &str, id: i64) -> String {
    let response = match reqwest::blocking::get(format!(
        "{}readMessage&login={}&domain={}&id={}",
        MAIL_API, login, domain, id
    )) {
        Ok(r) => match r.text() {
            Ok(s) => s,
            Err(_) => panic!("Response text error!"),
        },
        Err(_) => panic!("GET request error!"),
    };

    let message: Message = match serde_json::from_str(&response) {
        Ok(s) => s,
        Err(_) => panic!("JSON error!"),
    };

    format!(
        "From:{}\nSubject:{}\nDate{}\n\n{}",
        message.from, message.subject, message.date, message.textBody
    )
}
