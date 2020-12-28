/*******************************************************************************
*   (c) 2018 ZondaX GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
//! Provider for Ledger cosmos validator app
#[macro_use]
extern crate quick_error;

extern crate byteorder;
extern crate ledger;

const CLA: u8 = 0x56;
const INS_GET_VERSION: u8 = 0x00;
const INS_PUBLIC_KEY_ED25519: u8 = 0x01;
const INS_SIGN_ED25519: u8 = 0x02;

const USER_MESSAGE_CHUNK_SIZE: usize = 250;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidVersion{
            description("This version is not supported")
        }
        InvalidEmptyMessage{
            description("message cannot be empty")
        }
        InvalidMessageSize{
            description("message size is invalid (too big)")
        }
        InvalidPK{
            description("received an invalid PK")
        }
        NoSignature {
            description("received no signature back")
        }
        InvalidSignature {
            description("received an invalid signature")
        }
        InvalidDerivationPath {
            description("invalid derivation path")
        }
        Ledger ( err: ledger::Error ) {
            from()
            description("ledger error")
            display("Ledger error: {}", err)
            cause(err)
        }
    }
}

#[allow(dead_code)]
pub struct CosmosValidatorApp
{
    app: ledger::LedgerApp
}

unsafe impl Send for CosmosValidatorApp {}

#[allow(dead_code)]
pub struct Version {
    mode: u8,
    major: u8,
    minor: u8,
    patch: u8,
}

fn to_bip32array(path: &[u32]) -> Result<Vec<u8>, Error> {
    use byteorder::{LittleEndian, WriteBytesExt};

    if path.len() > 10 {
        return Err(Error::InvalidDerivationPath);
    }

    let mut answer = Vec::new();
    answer.write_u8(path.len() as u8).unwrap();

    for v in path { answer.write_u32::<LittleEndian>(*v).unwrap(); }

    Ok(answer)
}

impl CosmosValidatorApp {
    pub fn connect() -> Result<Self, Error> {
        let app = ledger::LedgerApp::new()?;
        Ok(CosmosValidatorApp { app })
    }

    pub fn version(&self) -> Result<Version, Error> {
        use ledger::ApduCommand;

        let command = ApduCommand {
            cla: CLA,
            ins: INS_GET_VERSION,
            p1: 0x00,
            p2: 0x00,
            length: 0,
            data: Vec::new(),
        };

        let response = self.app.exchange(command)?;

        // TODO: this is just temporary, ledger errors should check for 0x9000
        if response.retcode != 0x9000 {
            return Err(Error::InvalidVersion);
        }

        let version = Version {
            mode: response.data[0],
            major: response.data[1],
            minor: response.data[2],
            patch: response.data[3],
        };

        Result::Ok(version)
    }

    pub fn public_key(&self) -> Result<[u8; 32], Error> {
        use ledger::ApduCommand;

        // TODO: Define what to do with the derivation path
        let mut bip32 = vec![44, 118, 0, 0, 0];
        for i in &mut bip32 {
            *i |= 0x8000_0000;
        }

        let bip32path = to_bip32array(&bip32)?;

        let command = ApduCommand {
            cla: CLA,
            ins: INS_PUBLIC_KEY_ED25519,
            p1: 0x00,
            p2: 0x00,
            length: bip32path.len() as u8,
            data: bip32path,
        };

        let response = self.app.exchange(command)?;

        if response.retcode != 0x9000 {
            println!("WARNING: retcode={:X?}", response.retcode);
        }

        if response.data.len() != 32 {
            return Err(Error::InvalidPK);
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&response.data[..32]);
        Ok(array)
    }

    // Sign message
    pub fn sign(&self, message: &[u8]) -> Result<[u8; 64], Error> {
        use ledger::ApduCommand;
        use ledger::ApduAnswer;

        let chunks = message.chunks(USER_MESSAGE_CHUNK_SIZE);

        if chunks.len() > 255 {
            return Err(Error::InvalidMessageSize);
        }

        if chunks.len() == 0 {
            return Err(Error::InvalidEmptyMessage);
        }

        let packet_count = chunks.len() as u8;
        let mut response: ApduAnswer = ApduAnswer { data: vec![], retcode: 0 };

        // Send message chunks
        for (packet_idx, chunk) in chunks.enumerate() {
            let _command = ApduCommand {
                cla: CLA,
                ins: INS_SIGN_ED25519,
                p1: (packet_idx + 1) as u8,
                p2: packet_count,
                length: chunk.len() as u8,
                data: chunk.to_vec(),
            };

            response = self.app.exchange(_command)?;
        }

        if response.data.len() == 0 && response.retcode == 0x9000 {
            return Err(Error::NoSignature);
        }

        // Last response should contain the answer
        if response.data.len() != 64 {
            return Err(Error::InvalidSignature);
        }

        let mut array = [0u8; 64];
        array.copy_from_slice(&response.data[..64]);
        Ok(array)
    }
}

#[cfg(test)]
#[macro_use]
extern crate matches;

#[cfg(test)]
extern crate sha2;

#[cfg(test)]
extern crate ed25519_dalek;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use CosmosValidatorApp;
    use std::sync::Mutex;

    lazy_static! {
        static ref APP: Mutex<CosmosValidatorApp> = Mutex::new(CosmosValidatorApp::connect().unwrap());
    }

    #[test]
    fn derivation_path() {
        use to_bip32array;

        let mut answer = to_bip32array(&vec![1]).unwrap();
        assert_eq!(answer, b"\x01\
                             \x01\x00\x00\x00");

        answer = to_bip32array(&vec![1, 2]).unwrap();
        assert_eq!(answer, b"\x02\
                             \x01\x00\x00\x00\
                             \x02\x00\x00\x00");

        answer = to_bip32array(&vec![1, 2, 12345]).unwrap();
        assert_eq!(answer, b"\x03\
                             \x01\x00\x00\x00\
                             \x02\x00\x00\x00\
                             \x39\x30\x00\x00");

        answer = to_bip32array(&vec![44, 118, 0, 0, 0]).unwrap();
        assert_eq!(answer, b"\x05\
                             \x2c\x00\x00\x00\
                             \x76\x00\x00\x00\
                             \x00\x00\x00\x00\
                             \x00\x00\x00\x00\
                             \x00\x00\x00\x00");

        answer = to_bip32array(&vec![
            44 | 0x80000000,
            118 | 0x80000000,
            0 | 0x80000000,
            0 | 0x80000000,
            0 | 0x80000000]).unwrap();

        assert_eq!(answer, b"\x05\
                             \x2c\x00\x00\x80\
                             \x76\x00\x00\x80\
                             \x00\x00\x00\x80\
                             \x00\x00\x00\x80\
                             \x00\x00\x00\x80");
    }

    #[test]
    fn version() {
        let app = APP.lock().unwrap();

        let resp = app.version();

        match resp {
            Ok(version) => {
                println!("mode  {}", version.mode);
                println!("major {}", version.major);
                println!("minor {}", version.minor);
                println!("patch {}", version.patch);

                assert_eq!(version.mode, 0xFF);
                assert_eq!(version.major, 0x00);
                assert!(version.minor >= 0x04);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn public_key() {
        let app = APP.lock().unwrap();
        let resp = app.public_key();

        match resp {
            Ok(pk) => {
                assert_eq!(pk.len(), 32);
                println!("PK {:0X?}", pk);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn sign_empty() {
        use Error;

        let app = APP.lock().unwrap();

        let some_message0 = b"";

        let signature = app.sign(some_message0);
        assert!(signature.is_err());
        assert!(matches!(signature.err().unwrap(), Error::InvalidEmptyMessage));
    }

    #[test]
    fn sign_verify() {
        let app = APP.lock().unwrap();

        let some_message1 = [
            0x8,                                    // (field_number << 3) | wire_type
            0x1,                                    // PrevoteType
            0x11,                                   // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // height
            0x19,                                   // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // round
            0x22, // (field_number << 3) | wire_type
            // remaining fields (timestamp):
            0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1];

        match app.sign(&some_message1) {
            Ok(sig) => { println!("{:#?}", sig.to_vec()); }
            Err(e) => { println!("Err {:#?}", e); }
        }

        let some_message2 = [
            0x8,                                    // (field_number << 3) | wire_type
            0x1,                                    // PrevoteType
            0x11,                                   // (field_number << 3) | wire_type
            0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // height
            0x19,                                   // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // round
            0x22, // (field_number << 3) | wire_type
            // remaining fields (timestamp):
            0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1];

        match app.sign(&some_message2) {
            Ok(sig) => {
                use sha2::Sha512;
                use ed25519_dalek::PublicKey;
                use ed25519_dalek::Signature;

                println!("{:#?}", sig.to_vec());

                // First, get public key
                let public_key_bytes = app.public_key().unwrap();
                let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
                let signature = Signature::from_bytes(&sig).unwrap();

                // Verify signature
                assert!(public_key.verify::<Sha512>(&some_message2, &signature).is_ok());
            }
            Err(e) => { println!("Err {:#?}", e); }
        }
    }

    #[test]
    fn sign_many() {
        let app = APP.lock().unwrap();

        // First, get public key
        let resp = app.public_key();
        match resp {
            Ok(pk) => {
                assert_eq!(pk.len(), 32);
                println!("{:?}", pk);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }


        // Now send several votes
        for index in 50u8..254u8 {
            let some_message1 = [
                0x8,                                    // (field_number << 3) | wire_type
                0x1,                                    // PrevoteType
                0x11,                                   // (field_number << 3) | wire_type
                index, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // height
                0x19,                                   // (field_number << 3) | wire_type
                0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, // round
                0x22, // (field_number << 3) | wire_type
                // remaining fields (timestamp):
                0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1];

            let signature = app.sign(&some_message1);
            match signature {
                Ok(sig) => { println!("{:#?}", sig.to_vec()); }
                Err(e) => { println!("Err {:#?}", e); }
            }
        }
    }
}