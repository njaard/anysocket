use std::io::Result;
use std::net::SocketAddr as IpSocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::SocketAddr as UnixSocketAddr;
#[cfg(unix)]
use std::os::unix::net::UnixListener;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

/// Like ToSocketAddrs
pub trait AbstractToSocketAddrs {
    /// Like TcpListener::bind
    fn bind_any(&self) -> Result<AbstractListener>;
    /// Like TcpStream::connect
    fn connect_any(&self) -> Result<AbstractStream>;
}

impl AbstractToSocketAddrs for IpSocketAddr {
    fn bind_any(&self) -> Result<AbstractListener> {
        TcpListener::bind(self).map(Into::into)
    }

    fn connect_any(&self) -> Result<AbstractStream> {
        TcpStream::connect(self).map(Into::into)
    }
}

impl AbstractToSocketAddrs for (&str, u16) {
    fn bind_any(&self) -> Result<AbstractListener> {
        TcpListener::bind(self).map(Into::into)
    }

    fn connect_any(&self) -> Result<AbstractStream> {
        TcpStream::connect(self).map(Into::into)
    }
}

#[cfg(unix)]
impl AbstractToSocketAddrs for UnixSocketAddr {
    fn bind_any(&self) -> Result<AbstractListener> {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "cannot bind to an existing address",
        ))
    }

    fn connect_any(&self) -> Result<AbstractStream> {
        if let Some(p) = self.as_pathname() {
            UnixStream::connect(p).map(Into::into)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "cannot connect to unnamed address",
            ))
        }
    }
}

impl AbstractToSocketAddrs for str {
    fn bind_any(&self) -> Result<AbstractListener> {
        #[cfg(unix)]
        if self.starts_with("unix:") {
            return UnixListener::bind(&self["unix:".len()..]).map(Into::into);
        }
        TcpListener::bind(self).map(Into::into)
    }
    fn connect_any(&self) -> Result<AbstractStream> {
        #[cfg(unix)]
        if self.starts_with("unix:") {
            return UnixStream::connect(&self["unix:".len()..]).map(Into::into);
        }
        TcpStream::connect(self).map(Into::into)
    }
}

impl AbstractToSocketAddrs for &str {
    fn bind_any(&self) -> Result<AbstractListener> {
        #[cfg(unix)]
        if self.starts_with("unix:") {
            return UnixListener::bind(&self["unix:".len()..]).map(Into::into);
        }
        TcpListener::bind(self).map(Into::into)
    }
    fn connect_any(&self) -> Result<AbstractStream> {
        #[cfg(unix)]
        if self.starts_with("unix:") {
            return UnixStream::connect(&self["unix:".len()..]).map(Into::into);
        }
        TcpStream::connect(self).map(Into::into)
    }
}

#[cfg(unix)]
impl AbstractToSocketAddrs for dyn AsRef<std::path::Path> {
    fn bind_any(&self) -> Result<AbstractListener> {
        UnixListener::bind(self).map(Into::into)
    }
    fn connect_any(&self) -> Result<AbstractStream> {
        UnixStream::connect(self).map(Into::into)
    }
}

impl AbstractToSocketAddrs for AbstractAddr {
    fn bind_any(&self) -> Result<AbstractListener> {
        match self {
            AbstractAddr::Ip(a) => (*a).bind_any().map(Into::into),
            #[cfg(unix)]
            AbstractAddr::Unix(a) => (*a).bind_any().map(Into::into),
        }
    }
    fn connect_any(&self) -> Result<AbstractStream> {
        match self {
            AbstractAddr::Ip(a) => a.connect_any().map(Into::into),
            #[cfg(unix)]
            AbstractAddr::Unix(a) => a.connect_any().map(Into::into),
        }
    }
}

/// Like TcpListener
///
/// Either a [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
/// or [`UnixListener`](https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html)
///
/// Instead of calling `TcpListener::bind(address)`, you would call `address.bind_any`.
pub enum AbstractListener {
    Tcp(TcpListener),
    #[cfg(unix)]
    Unix(UnixListener),
}

impl Into<AbstractListener> for TcpListener {
    fn into(self) -> AbstractListener {
        AbstractListener::Tcp(self)
    }
}

#[cfg(unix)]
impl Into<AbstractListener> for UnixListener {
    fn into(self) -> AbstractListener {
        AbstractListener::Unix(self)
    }
}

/// Like SocketAddr
///
/// Either a [`SocketAddr`](https://doc.rust-lang.org/std/net/struct.SocketAddr.html)
/// or [`std::os::unix::net::SocketAddr`](https://doc.rust-lang.org/std/os/unix/net/struct.SocketAddr.html)
#[derive(Debug, Clone)]
pub enum AbstractAddr {
    Ip(IpSocketAddr),
    #[cfg(unix)]
    Unix(UnixSocketAddr),
}

impl AbstractAddr {
    pub fn port(&self) -> Option<u16> {
        match self {
            AbstractAddr::Ip(a) => Some(a.port()),
            #[cfg(unix)]
            AbstractAddr::Unix(_) => None,
        }
    }
}

impl std::fmt::Display for AbstractAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbstractAddr::Ip(a) => write!(f, "{}", a),
            #[cfg(unix)]
            AbstractAddr::Unix(a) => write!(f, "{:?}", a),
        }
    }
}

impl Into<AbstractAddr> for IpSocketAddr {
    fn into(self) -> AbstractAddr {
        AbstractAddr::Ip(self)
    }
}
#[cfg(unix)]
impl Into<AbstractAddr> for UnixSocketAddr {
    fn into(self) -> AbstractAddr {
        AbstractAddr::Unix(self)
    }
}

/// Like TcpStream
///
/// Either a [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
/// or an [`UnixStream`](https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html)
pub enum AbstractStream {
    Tcp(TcpStream),
    #[cfg(unix)]
    Unix(UnixStream),
}

impl Into<AbstractStream> for TcpStream {
    fn into(self) -> AbstractStream {
        AbstractStream::Tcp(self)
    }
}
#[cfg(unix)]
impl Into<AbstractStream> for UnixStream {
    fn into(self) -> AbstractStream {
        AbstractStream::Unix(self)
    }
}

impl AbstractStream {
    pub fn shutdown(&self, how: std::net::Shutdown) -> Result<()> {
        match self {
            Self::Tcp(l) => l.shutdown(how),
            #[cfg(unix)]
            Self::Unix(l) => l.shutdown(how),
        }
    }
    pub fn try_clone(&self) -> Result<AbstractStream> {
        match self {
            Self::Tcp(l) => l.try_clone().map(Into::into),
            #[cfg(unix)]
            Self::Unix(l) => l.try_clone().map(Into::into),
        }
    }
    pub fn peer_addr(&self) -> Result<AbstractAddr> {
        match self {
            Self::Tcp(l) => l.peer_addr().map(Into::into),
            #[cfg(unix)]
            Self::Unix(l) => l.peer_addr().map(Into::into),
        }
    }
}

impl std::convert::AsRef<dyn std::io::Read> for AbstractStream {
    fn as_ref(&self) -> &(dyn std::io::Read + 'static) {
        match self {
            Self::Tcp(l) => l,
            #[cfg(unix)]
            Self::Unix(l) => l,
        }
    }
}

impl std::convert::AsRef<dyn std::io::Write> for AbstractStream {
    fn as_ref(&self) -> &(dyn std::io::Write + 'static) {
        match self {
            Self::Tcp(l) => l,
            #[cfg(unix)]
            Self::Unix(l) => l,
        }
    }
}

impl std::io::Read for AbstractStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.read(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.read(buf),
        }
    }
    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut]) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.read_vectored(bufs),
            #[cfg(unix)]
            Self::Unix(l) => l.read_vectored(bufs),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.read_to_end(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.read_to_end(buf),
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.read_to_string(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.read_to_string(buf),
        }
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        match self {
            Self::Tcp(l) => l.read_exact(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.read_exact(buf),
        }
    }
}

impl std::io::Write for AbstractStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.write(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.write(buf),
        }
    }
    fn flush(&mut self) -> Result<()> {
        match self {
            Self::Tcp(l) => l.flush(),
            #[cfg(unix)]
            Self::Unix(l) => l.flush(),
        }
    }
    fn write_vectored(&mut self, bufs: &[std::io::IoSlice]) -> Result<usize> {
        match self {
            Self::Tcp(l) => l.write_vectored(bufs),
            #[cfg(unix)]
            Self::Unix(l) => l.write_vectored(bufs),
        }
    }
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        match self {
            Self::Tcp(l) => l.write_all(buf),
            #[cfg(unix)]
            Self::Unix(l) => l.write_all(buf),
        }
    }
    fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()> {
        match self {
            Self::Tcp(l) => l.write_fmt(fmt),
            #[cfg(unix)]
            Self::Unix(l) => l.write_fmt(fmt),
        }
    }
}

/// Like TcpListener
///
/// Either a [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
/// or an [`UnixListener`](https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html)
impl AbstractListener {
    pub fn local_addr(&self) -> Result<AbstractAddr> {
        match self {
            Self::Tcp(l) => l.local_addr().map(|m| m.into()),
            #[cfg(unix)]
            Self::Unix(l) => l.local_addr().map(|m| m.into()),
        }
    }

    pub fn accept(&self) -> Result<(AbstractStream, AbstractAddr)> {
        match self {
            Self::Tcp(l) => l
                .accept()
                .map(|(s, a)| (AbstractStream::Tcp(s), AbstractAddr::Ip(a))),
            #[cfg(unix)]
            Self::Unix(l) => l
                .accept()
                .map(|(s, a)| (AbstractStream::Unix(s), AbstractAddr::Unix(a))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn parse1() {
        let _b = "unix:abc".bind_any();
    }
}
