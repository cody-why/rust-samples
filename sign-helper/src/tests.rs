/*
 * @Author: plucky
 * @Date: 2022-10-27 08:43:53
 * @LastEditTime: 2022-10-27 17:00:16
 * @Description:
 */

#[cfg(test)]
mod tests {
    


    #[test]
    fn it_works_by_rsa() {
        use rsa::{PublicKey, RsaPublicKey, PaddingScheme, pkcs8:: DecodePublicKey};
        let mut data = vec![
            ("appId".to_string(),"100".to_string()),
            ("gameId".to_string(),"100".to_string()),
            ("at".to_string(),"100".to_string()),
        ];
        data.sort_by(|a,b|{
            a.cmp(b)
        });
        let str = data.iter().map(|(k,v)| format!("{}={}",k.trim(),v.trim())).collect::<Vec<String>>().join("&");
        // 1. md5
        let md5 = md5::compute(str.as_bytes());
        let md5 = format!("{:X}", md5);
        // let public_key = include_str!("../public_key.pem");
        let public_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCj3V68uVdsB5GhhrMQRAanapY9PtGwKsdFPXrNjG/27zp7obYbe0paN4Urs83w5HM4mo3uPLRCJpiTwxkqdwmQ4Mp5DNdb9k2gmnHJG2EcM4/annEdVyg4b3MeKxzCNRUGNWH4Ybic5AA5rjYC/BI0GHUWjbq9+odMKRUyddsvFwIDAQAB";
        // 格式pem,间隔64个字符换行
        let public_key = public_key.as_bytes().chunks(64).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<&str>>().join("\n");
        let public_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key);
        // rsa pkcs8
        let public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();
        
        let mut rng = rand::rngs::OsRng;
        let rsau8 = public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), md5.as_bytes()).unwrap();
        let rsa_str = base64::encode(rsau8);

        println!("{}",format!("params: {}\nmd5: {}\nrsa: {}",str, md5, rsa_str));
    }

    #[test]
    fn it_works_by_openssl() {
        use openssl::rsa::{Rsa, Padding};
        
        let mut data = vec![
            ("appId".to_string(),"100".to_string()),
            ("gameId".to_string(),"100".to_string()),
            ("at".to_string(),"100".to_string()),
        ];
        data.sort_by(|a,b|{
            a.cmp(b)
        });
        let str = data.iter().map(|(k,v)| format!("{}={}",k.trim(),v.trim())).collect::<Vec<String>>().join("&");
        // 1. md5
        let md5 = md5::compute(str.as_bytes());
        let md5 = format!("{:X}", md5);
        // let public_key = include_str!("../public_key.pem");
        let public_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCj3V68uVdsB5GhhrMQRAanapY9PtGwKsdFPXrNjG/27zp7obYbe0paN4Urs83w5HM4mo3uPLRCJpiTwxkqdwmQ4Mp5DNdb9k2gmnHJG2EcM4/annEdVyg4b3MeKxzCNRUGNWH4Ybic5AA5rjYC/BI0GHUWjbq9+odMKRUyddsvFwIDAQAB";
        // 格式pem,间隔64个字符换行
        let public_key = public_key.as_bytes().chunks(64).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<&str>>().join("\n");
        let public_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key);
        
        let rsa = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
        let mut rsa_u8 = vec![0; rsa.size() as usize];
        let len = rsa.public_encrypt(md5.as_bytes(), &mut rsa_u8, Padding::PKCS1).unwrap();
        rsa_u8.truncate(len);
        let rsa_str = base64::encode(rsa_u8);

        println!("{}",format!("params: {}\nmd5: {}\nrsa: {}",str, md5, rsa_str));
    }


    #[test]
    fn test_generate_key_by_openssl() {
        use openssl::rsa::Rsa;
        
        let rsa = Rsa::generate(1024).unwrap();
        let public_key = rsa.public_key_to_pem().unwrap();
        let private_key = rsa.private_key_to_pem().unwrap();
        println!("public_key: {}", std::str::from_utf8(&public_key).unwrap());
        println!("private_key: {}", std::str::from_utf8(&private_key).unwrap());
        
    }

    #[test]
    fn test_generate_key_by_rsa() {
        use rsa::{RsaPrivateKey, pkcs1::EncodeRsaPrivateKey,pkcs8::EncodePublicKey,pkcs8::LineEnding};
        
        let private_key = RsaPrivateKey::new(&mut rand::rngs::OsRng, 1024).unwrap();
        // let public_key = private_key.to_public_key();
        let public_key = private_key.to_public_key_pem(LineEnding::LF).unwrap_or_default();
        let private_key:String = private_key.to_pkcs1_pem(LineEnding::LF).unwrap_or_default().as_str().to_string();
        println!("public_key: {}", public_key);
        println!("private_key: {}", private_key);
        
    }
    
}
