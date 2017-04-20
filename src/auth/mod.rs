use jwt::{Registered, Token};
use jwt::header::{Algorithm, Header, HeaderType};
use models::User;
use time::{Duration, now_utc};

impl User {
    pub fn auth_token(&self) -> Token<Header, Registered> {
        let header = Header {
            typ: Some(HeaderType::JWT),
            kid: None,
            alg: Algorithm::HS256,
        };
        let now = now_utc().to_timespec();
        let exp = now + Duration::weeks(1);
        let claims = Registered {
            aud: Some("snacks.acm.umn.edu".to_string()),
            exp: Some(exp.sec as u64),
            iat: Some(now.sec as u64),
            iss: Some("auth.acm.umn.edu".to_string()),
            jti: None,
            nbf: Some(now.sec as u64),
            sub: Some(self.id.to_string()),
        };
        Token::new(header, claims)
    }
}
