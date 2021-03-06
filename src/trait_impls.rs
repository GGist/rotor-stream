use std::io;
use std::any::Any;
use std::net::SocketAddr;
#[cfg(unix)]
use std::path::PathBuf;

use rotor::mio::Evented;
use rotor::mio::tcp;
#[cfg(unix)]
use rotor::mio::unix;

use {StreamSocket, ActiveStream, SocketError};

impl<T> StreamSocket for T
    where T: io::Read, T: io::Write, T: Evented, T:SocketError, T:Any
{}

impl ActiveStream for tcp::TcpStream {
    type Address = SocketAddr;
    fn connect(addr: &SocketAddr) -> io::Result<Self> {
        tcp::TcpStream::connect(addr)
    }
}

#[cfg(unix)]
impl ActiveStream for unix::UnixStream {
    type Address = PathBuf;
    fn connect(addr: &PathBuf) -> io::Result<Self> {
        unix::UnixStream::connect(addr)
    }
}

impl SocketError for tcp::TcpStream {
    fn take_socket_error(&self) -> io::Result<()> {
        tcp::TcpStream::take_socket_error(self)
    }
}

#[cfg(unix)]
impl SocketError for unix::UnixStream {
    fn take_socket_error(&self) -> io::Result<()> {
        Ok(())
    }
}
