use p12_keystore::{Certificate, KeyStore, KeyStoreEntry, PrivateKeyChain};
use rcgen::DnType::{CommonName, CountryName, LocalityName, OrganizationName, OrganizationalUnitName};
use rcgen::DnValue::{PrintableString, Utf8String};
use rcgen::{date_time_ymd, CertificateParams, DistinguishedName, KeyPair, RsaKeySize, PKCS_RSA_SHA256};
use sha1::{Digest, Sha1};
use std::{env, fs, io};

struct KeyReq {
    key: KeyPair,
    csr: rcgen::CertificateSigningRequest,
    crt: Option<Certificate>,
}

impl KeyReq {
    fn key_and_req(dn: DistinguishedName) -> KeyReq {
        let key = KeyPair::generate_rsa_for(&PKCS_RSA_SHA256, RsaKeySize::_2048).unwrap();

        let mut param = CertificateParams::new(vec!["P5T".to_string()]).unwrap();

        param.distinguished_name = dn;
        param.not_before = date_time_ymd(2024, 1, 1);
        param.not_after = date_time_ymd(4096, 1, 1);
        let csr = param.serialize_request(&key).unwrap();

        KeyReq { key, csr, crt: None }
    }

    fn p12(&self) -> Option<KeyStore> {
        self.crt.as_ref()?;

        let mut sha1 = Sha1::new();
        sha1.update(self.csr.der());

        let private_key_chain = PrivateKeyChain::new(
            self.key.serialize_pem(),
            sha1.finalize(),
            vec![Certificate::from_der(self.csr.der()).unwrap()],
        );

        let mut key_store = KeyStore::new();
        key_store.add_entry("", KeyStoreEntry::PrivateKeyChain(private_key_chain));
        Some(key_store)
    }

    fn write_csr(&self, cn: &str) {
        fs::write(
            env::var("HOME").unwrap() + "/certs/" + cn + ".csr",
            self.csr.pem().unwrap(),
        ).unwrap();
    }

    fn write_key(&self, cn: &str) {
        fs::write(
            env::var("HOME").unwrap() + "/certs/" + cn + ".key",
            self.key.serialize_pem(),
        ).unwrap()
    }

    fn write_p12(&self, cn: &str) {
        if let Some(p12) = self.p12() {
            fs::write(
                env::var("HOME").unwrap() + "/certs/" + cn + ".p12",
                p12.writer("secPas$4").write().unwrap(),
            ).unwrap();
        } else {
            println!("No pkcs12 data found.")
        }
    }
}

fn get_distinguished_names(file: &str) -> io::Result<Vec<DistinguishedName>> {
    Ok(fs::read_to_string(file)?.lines()
        .map(|line| line.trim())
        .filter(|&line| {
            if !line.starts_with("CN=") {
                eprintln!("Warning: Skipping invalid input: '{line}'");
                false
            } else { true }
        })
        .map(|line| {
            let mut dn = DistinguishedName::new();
            line.split(',')
                .map(|field| field.split_once('=').unwrap())
                .for_each(|key_val| {
                    match key_val.0.trim() {
                        "OU" => dn.push(OrganizationalUnitName, PrintableString(key_val.1.try_into().unwrap())),
                        "CN" => dn.push(CommonName, Utf8String(String::from(key_val.1))),
                        "C" => dn.push(CountryName, PrintableString(key_val.1.try_into().unwrap())),
                        "O" => dn.push(OrganizationName, PrintableString(key_val.1.try_into().unwrap())),
                        "L" => dn.push(LocalityName, PrintableString(key_val.1.try_into().unwrap())),
                        _ => println!("{:?}", key_val),
                    }
                });
            dn
        })
        .collect::<Vec<_>>())
}

fn main() {
    if let Ok(dn) = get_distinguished_names(&format!("{}/certs/dn_input.txt", env::var("HOME").unwrap())) {
        for dn in dn {
            let cn = if let Utf8String(cn) = dn.get(&CommonName).unwrap() {
                cn.clone()
            } else {
                println!("Failed to determine Common Name from DN");
                continue;
            };
            let key_req = KeyReq::key_and_req(dn);
            key_req.write_csr(&cn);
            key_req.write_key(&cn);
            key_req.write_p12(&cn);
        }
    }
}
