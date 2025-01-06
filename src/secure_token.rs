use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, Local, version4::V4};

pub fn generate_token(email: &str, path: &str) -> String {
    let settings = crate::settings::get_settings();
    let mut claims = Claims::new().unwrap();
    claims
        .add_additional("email", email)
        .unwrap();
    let sk;

    if path == r"/login/admin" {
        sk = SymmetricKey::<V4>::try_from(settings.admin_cookie_secret.as_str()).unwrap();
    } else {
        sk = SymmetricKey::<V4>::try_from(settings.club_cookie_secret.as_str()).unwrap();
    }

    local::encrypt(&sk, &claims, None, Some(settings.implicit_assertion.as_bytes())).unwrap()
}

pub fn verify_token(token: &str, path: &str) -> Result<String, ()> {
    let settings = crate::settings::get_settings();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(token).unwrap();
    let sk;

    if path == r"/login/admin" {
        sk = SymmetricKey::<V4>::try_from(settings.admin_cookie_secret.as_str()).unwrap();
    } else {
        sk = SymmetricKey::<V4>::try_from(settings.club_cookie_secret.as_str()).unwrap();
    }

    let trusted_token = local::decrypt(
        &sk, 
        &untrusted_token, 
        &validation_rules, 
        None, 
        Some(settings.implicit_assertion.as_bytes())
    );

    match trusted_token {
        Ok(token) => {
            let email = token.payload_claims().unwrap().get_claim("email").unwrap();
            Ok(email.to_string())
        }
        Err(_) => {
            Err(())
        }
    }
}