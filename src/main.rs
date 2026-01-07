mod proxy;
mod rustls_danger;

// use rustls::{ClientConfig, RootCertStore};
// use rustls::craft::{CHROME_108, CHROME_108_EXT, CHROME_CIPHER, CraftPadding, FingerprintBuilder};
// use rustls::craft::{Fingerprint, CraftExtension};
// use webpki_roots::TLS_SERVER_ROOTS;
// use rustls::craft::CraftExtension::FakeApplicationSettings;
// use rustls::internal::msgs::enums::ExtensionType;
// use rustls::internal::msgs::handshake::ClientExtension;

// use rustls_danger::NoCertificateVerification;

fn main() {
    // let root_store = RootCertStore {
    // roots: TLS_SERVER_ROOTS.into(),
    // };

    // let fingerprint: &Fingerprint = &CHROME_108.main;
    // let fingerprint = &CHROME_108.test_alpn_http1;

    // let aaa = CHROME_108_EXT.clone();
    // let bbb = CHROME_CIPHER.clone();
    //
    // let fingerprint = Fingerprint {
    //     extensions: &[],
    //     shuffle_extensions: false,
    //     cipher: &[],
    // };

    // let aaa = CraftExtension::Padding;
    // let aa = ClientExtension::CraftPadding(CraftPadding {
    //     psk_len: 16000
    // });
}
