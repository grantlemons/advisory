use crate::auth::decrypt_jwt;

#[tokio::test]
async fn sample_jwt_verify() -> Result<(), jsonwebtokens_cognito::Error> {
    let token: &str = "eyJraWQiOiIxQnpZSDNQaGNrMzZXQnFQZmhTU1V2WFpjREJZS05DZ0wzV09mS1JEeU5FPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiI2YmYwMzdiNS1mODVhLTQ3Y2MtODE5NC0xNmRiMzdhMzcyYTciLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMS5hbWF6b25hd3MuY29tXC91cy1lYXN0LTFfWWU5NnJHYnFWIiwiY29nbml0bzp1c2VybmFtZSI6IjZiZjAzN2I1LWY4NWEtNDdjYy04MTk0LTE2ZGIzN2EzNzJhNyIsIm9yaWdpbl9qdGkiOiJhYzg0NjMzNy0zNmQxLTQ0NmYtYmM2Mi0wZjUxYzg5ZTMyYmUiLCJhdWQiOiI1YzZldmE4bmN0cGIzYXVnOGwwdGVhazM2diIsImV2ZW50X2lkIjoiZmQ5YWQ0NTAtMGYyMS00NDQ2LWJhYmItZjA2OTFmYTM3MWFmIiwidG9rZW5fdXNlIjoiaWQiLCJhdXRoX3RpbWUiOjE2NzQ0NDUyNDksIm5hbWUiOiJHcmFudCBMZW1vbnMiLCJleHAiOjE2NzQ0NDg4NDksImlhdCI6MTY3NDQ0NTI0OSwianRpIjoiZDUxMDU4MDItODg4MS00Y2MwLWE1NzUtOTg2ZjM3NjgwOTRhIiwiZW1haWwiOiJncmFudGxlbW9uc0Bhb2wuY29tIn0.KJrw5GrxlPxrEd2raoKsJz0vyak0JrKzoqTcR19G-b6pkGPwRaHbOrqJZVH2DV01a-R8N0fMTLrthT7JgSJdxNNKNDHCgL-mTNjXE3NzQV3P0Kb-qx8kXcOBUtwSRV7rmmP6rMKiDFDtQiNDNevksctrfWf5M8EphepLhMouSJ_GeCQtAIFXgwMIlw1ggJzm4iUMD28vZ1_MjxhbXwe3Yqh3qXukp_n0cu78_CqnPbMULUotdHUnzKBbnCKabKaQydrTWIgEz0TtN7GLNZwMkHBeggVX2ZO8lP3-XW5t8w4EKKUPUUCerMh-xtyqpZW4lCkvwWFpYAJckHCbpAkzaA";

    let keyset = jsonwebtokens_cognito::KeySet::new("us-east-1", "us-east-1_Ye96rGbqV").unwrap();
    let state = crate::SharedState {
        graph: None,
        verifier: keyset
            .new_id_token_verifier(&["5c6eva8nctpb3aug8l0teak36v"])
            .build()
            .unwrap(),
        keyset,
    };

    match decrypt_jwt(token, &state.keyset, &state.verifier).await {
        Ok(user) => {
            println!("Successful JWT Verification for user {}", user.name);
            Ok(())
        }
        Err(err) => {
            println!("Failed JWT Verification");
            Err(err)
        }
    }
}
