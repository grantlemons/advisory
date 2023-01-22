use jsonwebtokens_cognito as jwt_c;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    aud: String,
    auth_time: u32,
    email: String,
    email_verified: bool,
    event_id: String,
    exp: u32,
    iat: u32,
    iss: String,
    jti: String,
    name: String,
    origin_jti: String,
    sub: String,
    token_use: String,
}

async fn verify_jwt(token: &str) -> Result<Data, jwt_c::Error> {
    let keyset = jwt_c::KeySet::new("us-east-1", "us-east-1_Ye96rGbqV")?;
    let verifier = keyset
        .new_id_token_verifier(&["5c6eva8nctpb3aug8l0teak36v"])
        .build()?;
    match keyset.verify(&token, &verifier).await {
        Ok(res) => Ok(serde_json::from_value(res).unwrap()),
        Err(err) => Err(err),
    }
}

#[tokio::test]
async fn sample_jwt_verify() -> Result<(), jwt_c::Error> {
    let token: &str = "eyJraWQiOiIxQnpZSDNQaGNrMzZXQnFQZmhTU1V2WFpjREJZS05DZ0wzV09mS1JEeU5FPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiI2YmYwMzdiNS1mODVhLTQ3Y2MtODE5NC0xNmRiMzdhMzcyYTciLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMS5hbWF6b25hd3MuY29tXC91cy1lYXN0LTFfWWU5NnJHYnFWIiwiY29nbml0bzp1c2VybmFtZSI6IjZiZjAzN2I1LWY4NWEtNDdjYy04MTk0LTE2ZGIzN2EzNzJhNyIsIm9yaWdpbl9qdGkiOiI2ZWYxNTAxMi03ZDUxLTRmYmEtYjY0Ni0yOTJkM2U5MDY1YTciLCJhdWQiOiI1YzZldmE4bmN0cGIzYXVnOGwwdGVhazM2diIsImV2ZW50X2lkIjoiMzFhNjhmMjktMTVkZC00MjBkLWIxMzctNGI5NTdmN2YxNmU5IiwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE2NzQ0MjQ3NTMsIm5hbWUiOiJHcmFudCBMZW1vbnMiLCJleHAiOjE2NzQ0MjgzNTMsImlhdCI6MTY3NDQyNDc1MywianRpIjoiYmE2YzRhYWUtMTRkYS00OWFhLWEwYmYtYzI4MDFiMWJhN2M5IiwiZW1haWwiOiJncmFudGxlbW9uc0Bhb2wuY29tIn0.Ev3KCjbUJlaFGOrmZwlhkiRlNHBQl708IAcEsZsTmb3_CJU1ZEJPuwUb_3629_HaYR4Q8g4but-0Pq9EA29RzpB7oSafjaCNjtJ9ofSxS2fgvuvYvh9VGdNh_T3RQ7urNbf7MyZs75-OyDKU2shJeZ6B9P-YJbKPcdf9s7EeNLp1Ylo6gTuKNYvFuwOj7eduMIEmt7y44ls6eo9EAsV62u1VwVr65vj1XaIhbsSYjJ41MaH-WUQ00SeN8rqQ2lg4o9AfcnWEKGdQ0EdQ3e_cSv11R2n0qqU4lM9o_WkK_8yzZE4f80znY_rBrQIi1lR3QLCPVW_Pftv426euv6oUXQ";
    match verify_jwt(token).await {
        Ok(res) => {
            println!("{:?}", res);
            Ok(())
        }
        Err(err) => Err(err),
    }
}
