use uuid::Uuid;
use std::io::*;
use model::*;
use xml::write;

pub struct FinancialInstitution<'a> {
    pub org : &'a str,
    pub fid : &'a str,
    pub url : &'a str,
}

pub struct Credentials<'a> {
    pub username : &'a str,
    pub password : &'a str,
}

pub struct VersionId<'a> {
    id : &'a str,
    version : &'a str,
}

pub struct OfxClient<'a> {
    versionId : VersionId<'a>,
    institution : FinancialInstitution<'a>,
    credentials : Credentials<'a>,
}

impl<'a> OfxClient<'a> {
    pub fn new(credentials : Credentials<'a>, financialInstitution : FinancialInstitution<'a>) -> OfxClient<'a> {
        OfxClient {
            versionId : VersionId{
                id : "QWIN",
                version : "1500"
            },
            institution : financialInstitution,
            credentials : credentials,
        }
    }

    pub fn list_profiles(&mut self) {
        let ofx = OFX {
            SIGNONMSGSRQV1 : SIGNONMSGSRQV1_T {
                SONRQ : SONRQ_T {
                    DTCLIENT : "20150509035913.964".to_string(),
                    USERID : self.credentials.username.to_string(),
                    USERPASS : self.credentials.password.to_string(),
                    LANGUAGE : "ENG".to_string(),
                    FI : FI_T {
                        ORG : self.institution.org.to_string(),
                        FID : self.institution.fid.to_string(),
                    },
                    APPID : self.versionId.id.to_string(),
                    APPVER : self.versionId.version.to_string(),
                }
            },
            ACCTINFOTRNRQ : ACCTINFOTRNRQ_T {
                TRNUID : Uuid::new_v4().to_string(),
                ACCTINFORQ : ACCTINFORQ_T {
                    DTACCTUP: "19700101000000.000".to_string()
                }
            }
        };

        write(&ofx);
    }
}
