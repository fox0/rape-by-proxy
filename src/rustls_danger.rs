use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::{DigitallySignedStruct, Error, SignatureScheme};

/// ```
/// use rustls::ClientConfig;
///
/// let mut config = ClientConfig::builder()
///     .with_root_certificates(root_store)
///     .with_no_client_auth();
///
/// let mut dangerous_config = ClientConfig::dangerous(&mut config);
/// dangerous_config.set_certificate_verifier(Arc::new(NoCertificateVerification {}));
/// ```
#[derive(Debug)]
pub struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls_pki_types::CertificateDer<'_>,
        _intermediates: &[rustls_pki_types::CertificateDer<'_>],
        _server_name: &rustls_pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls_pki_types::UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls_pki_types::CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls_pki_types::CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        unimplemented!()
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        // todo!()
        vec![SignatureScheme::ECDSA_NISTP256_SHA256]
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::net::TcpStream;
    use std::sync::Arc;

    use rustls::{ClientConfig, RootCertStore};
    use rustls::craft::CHROME_108;
    use webpki_roots::TLS_SERVER_ROOTS;

    use super::*;

    #[test]
    fn fakesni() {
        let root_store = RootCertStore {
            roots: TLS_SERVER_ROOTS.into(),
        };

        let mut config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
            .with_fingerprint(CHROME_108.main.builder());

        let mut dangerous_config = ClientConfig::dangerous(&mut config);
        dangerous_config.set_certificate_verifier(Arc::new(NoCertificateVerification {}));

        let server_name = "google.com".try_into().unwrap();
        let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
        let mut sock = TcpStream::connect("www.xnxx.com:443").unwrap();
        let mut tls = rustls::Stream::new(&mut conn, &mut sock);
        tls.write_all(
            concat!(
            "GET / HTTP/1.1\r\n",
            "Host: www.xnxx.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
            ).as_bytes()
        ).unwrap();
    }

    #[test]
    fn padding() {
        todo!();
    }
}
