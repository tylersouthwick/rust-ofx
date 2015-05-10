use rustc_serialize::*;

#[derive(Debug, RustcEncodable)]
pub struct OfxHolder {
  pub OFX: OFX,
}

#[derive(Debug, RustcEncodable)]
pub struct OFX {
  pub SIGNONMSGSRQV1 : SIGNONMSGSRQV1_T,
  pub ACCTINFOTRNRQ : ACCTINFOTRNRQ_T
}

#[derive(Debug, RustcEncodable)]
pub struct ACCTINFOTRNRQ_T {
    pub TRNUID : String,
    pub ACCTINFORQ : ACCTINFORQ_T
}

#[derive(Debug, RustcEncodable)]
pub struct ACCTINFORQ_T {
    pub DTACCTUP : String
}

#[derive(Debug, RustcEncodable)]
pub struct SIGNONMSGSRQV1_T {
    pub SONRQ: SONRQ_T
}

#[derive(Debug, RustcEncodable)]
pub struct SONRQ_T{
    pub DTCLIENT : String,
    pub USERID : String,
    pub USERPASS : String,
    pub LANGUAGE : String,
    pub FI : FI_T,
    pub APPID : String,
    pub APPVER : String
}

#[derive(Debug, RustcEncodable)]
pub struct FI_T {
    pub ORG : String,
    pub FID : String
}
