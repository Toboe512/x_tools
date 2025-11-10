use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help(args[0].as_str());
        exit(0);
    }

    let addr = if args.len() > 3 && args[2] == "-addr" {
        args[3].as_str()
    } else {
        "127.0.0.1:8080"
    };

    match args[1].as_str() {
        "-server" => {
            server(addr)?;
        }
        "-client" => {
            client(addr)?;
        }
        _ => {
            help(args[0].as_str());
        }
    }

    Ok(())
}

fn help(app_name: &str) {
    println!("Использование:");
    println!("  Сервер: {app_name} -server [-addr IP:PORT]");
    println!("  Клиент: {app_name} -client [-addr IP:PORT]");
    println!("По умолчанию: 127.0.0.1:8080");
}

fn server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Сервер запущен на {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Новое подключение: {:?}", stream.peer_addr());

                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| {
                        eprintln!("Ошибка обработки клиента: {e}");
                    });
                });
            }
            Err(e) => {
                eprintln!("Ошибка подключения: {e}");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        println!("От {peer_addr}: {line}");

        // Эхо-ответ
        let response = format!("Эхо: {line} \n");
        // TODO  добавить возможность отправки своего ответа

        stream.write_all(response.as_bytes())?;
    }

    println!("Клиент {peer_addr} отключился");
    Ok(())
}

fn client(server_addr: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(server_addr)?;
    println!("Подключились к серверу {server_addr}");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Поток для чтения ответов от сервера
    let stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        let reader = BufReader::new(stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("Сервер: {msg} \n"),
                Err(..) => break,
            }
        }
    });

    println!("Для отключения от серверва введите /quit");

    // Основной цикл для отправки сообщений
    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        if input.trim().is_empty() {
            continue;
        }

        if input.trim() == "/quit" {
            break;
        }

        stream.write_all(input.as_bytes())?;
        thread::sleep(Duration::from_millis(100));
    }

    println!("Отключение...");
    Ok(())
}
