use bytes::Bytes;
use tokio::{net::TcpStream, io::AsyncReadExt};
use mini_redis::{Frame, Result, frame};
// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>)
// }
// enum HttpFrame {
//     RequestHead {
//         method: Method,
//         uri: Uri,
//         version: Version,
//         headers: HeaderMap,
//     },
//     ResponseHead {
//         status: StatusCode,
//         version: Version,
//         headers: HeaderMap,
//     },
//     BodyChunk {
//         chunk:Bytes,
//     },

// }

struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
       loop {
           // Attempt to parse a frame from the buffered data.if enough data has ben buffered the fram is return
           if let Some(frame) = self.parse_frame()?   {
            return Ok(Some(frame));
               
           }

           // Ensure the buffer has capacity
           if self.buffer.len() = self.cusror {
            self.buffer.resize(self.cusor*2,0);
           }

           // Read into the buffer, tracking the number of bytes read
           let n = self.stream.read(
            &mut self.buffer[self.cusror..]
           ).await?;

           if 0 == n {
            if self.cursor == 0{
                return Ok(None);
            }else {
                return Err("connection reset by peer".into());
            }
           }else {
               self.cursor += n;
           }
           // There is not enough buffered data to read a frame
           // attempt to read more data from the socket
           // On success, the number of bytes is returned. `0`
           // indicates "end of stream".
           if 0 == self.stream.read_buf(&mut self.buffer).await? {
            // the remote cosed the connection. for this to be
            // a claean shutdown. there should be no data in the read buffer.
            // if there is, this means that the 
            // peer closed the socket while sending a frame.
            if self.buffer.is_empty() {
                return Ok(None);
            }else {
                return Err("connection reset by peer".into());
            }
           }
       }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()>{
        Ok(())
    }
}
fn main() {
    
}