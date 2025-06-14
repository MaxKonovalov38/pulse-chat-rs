use std::net::TcpStream;
use std::io::{Read};

fn main() -> std::io::Result<()> {
    // Устанавливаем соединение с сервером на адресе "127.0.0.1:12345"
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;

    // Буфер для чтения данных
    let mut buffer = [0; 1024];

    // Читаем данные от сервера
    let bytes_read = stream.read(&mut buffer)?;

    // Преобразуем байты в строку и печатаем полученные данные
    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Получено: {}", received_data);

    Ok(())
}
