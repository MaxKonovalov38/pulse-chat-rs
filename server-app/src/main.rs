use std::net::TcpListener;
use std::io::{Write};

fn main() -> std::io::Result<()> {
    // Создание TCP сокета, который будет слушать на адресе 127.0.0.1:12345
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    println!("Сервер запущен. Ожидание подключения...");

    for stream in listener.incoming() {
        // Получение потока данных для обмена с клиентом
        let mut stream = stream?;

        println!("Подключение от {:?}", stream.peer_addr());

        // Отправка данных клиенту
        stream.write_all(b"Hello")?;
    }

    Ok(())
}
