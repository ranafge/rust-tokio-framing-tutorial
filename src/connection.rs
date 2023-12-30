use bytes::BytesMut;
use tokio::net::TcpStream;

// pub struct Connection {
//     stream : TcpStream,
//     buffer: BytesMut
// }
pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize
}
impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // buffer: BytesMut::with_capacity(4096)
            buffer: vec![0; 4096],
            cursor: 0
        }
    }
}