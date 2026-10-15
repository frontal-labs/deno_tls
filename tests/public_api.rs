use std::io::Cursor;

use deno_tls::SocketUse;
use deno_tls::TlsClientConfigOptions;
use deno_tls::TlsError;
use deno_tls::create_client_config;
use deno_tls::create_default_root_cert_store;
use deno_tls::load_certs;
use deno_tls::load_private_keys;

#[test]
fn default_root_store_is_populated() {
  assert!(!create_default_root_cert_store().is_empty());
}

#[test]
fn client_config_selects_alpn_for_http_modes() {
  let config = create_client_config(TlsClientConfigOptions {
    socket_use: SocketUse::Http,
    ..Default::default()
  })
  .unwrap();
  assert_eq!(
    config.alpn_protocols,
    vec![b"h2".to_vec(), b"http/1.1".to_vec()]
  );

  let config = create_client_config(TlsClientConfigOptions {
    socket_use: SocketUse::Http1Only,
    ..Default::default()
  })
  .unwrap();
  assert_eq!(config.alpn_protocols, vec![b"http/1.1".to_vec()]);
}

#[test]
fn empty_certificate_and_key_inputs_report_specific_errors() {
  assert!(matches!(
    load_certs(&mut Cursor::new(Vec::<u8>::new())),
    Err(TlsError::CertsNotFound)
  ));
  assert!(matches!(
    load_private_keys(&[]),
    Err(TlsError::KeysNotFound)
  ));
}
