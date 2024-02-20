use std::io::{self, Cursor};

use tokio::{
    io::AsyncWriteExt,
    net::{self, TcpStream},
};

use crate::minecraft::{
    packet::{handshake::Handshake, slp::SlpRequest},
    varint::VarInt,
};

use super::packet::slp::SlpResponse;

pub struct Client {
    host: String,
    port: u16,
}

impl Client {
    pub async fn new(host: String, port: u16) -> io::Result<Self> {
        _ = net::lookup_host(format!("{}:{}", host, port)).await?;

        Ok(Self { host, port })
    }

    async fn connection(&self) -> io::Result<TcpStream> {
        let addr = match net::lookup_host(format!("{}:{}", self.host, self.port))
            .await?
            .next()
        {
            Some(addr) => addr,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Hostname doesn't resolve to address",
                ))
            }
        };

        let stream =
            std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(5))?;
        stream.set_nonblocking(true)?;

        Ok(TcpStream::from_std(stream)?)
    }

    pub async fn status(&self) -> io::Result<SlpResponse> {
        let mut stream = self.connection().await?;

        {
            let packet = Handshake::new(
                Handshake::VERSION_UNSPECIFIED,
                self.host.clone(),
                self.port,
                Handshake::NEXT_STATE_STATUS,
            );
            let bytes: Vec<u8> = packet.into();

            stream.write_all(bytes.as_slice()).await?;
        }

        {
            let packet = SlpRequest;
            let bytes: Vec<u8> = packet.into();

            stream.write_all(bytes.as_slice()).await?;
        }

        stream.readable().await?;

        let mut data = Vec::new();

        let packet_size: usize;
        {
            let mut buf = [0; 5]; // the varint packet len has a maximum of 5 bytes
            stream.try_read(&mut buf)?;

            let packet_len = VarInt::from_bytes(Cursor::new(&buf))?;
            data.extend_from_slice(&buf);

            packet_size = packet_len.0 as usize + packet_len.to_bytes().len();
            log::debug!("packet size: {}", packet_size);
        }

        let mut buf = [0; 1024];
        while data.len() < packet_size - 1 {
            stream.readable().await?;

            match stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => data.extend_from_slice(&buf[0..n]),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e.into()),
            }

            log::debug!("read: {}/{}", data.len(), packet_size);
        }

        SlpResponse::try_from(data)
    }
}
