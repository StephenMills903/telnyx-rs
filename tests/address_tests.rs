mod common;

use telnyx_rs::models::CreateAddressRequest;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{bearer_token, body_json, method, path},
};

mod responses {
    use chrono::Utc;

    pub fn address_response(id: i64) -> serde_json::Value {
        serde_json::json!({
            "data": {
                "id": id,
                "record_type": "address",
                "customer_reference": null,
                "first_name": "John",
                "last_name": "Doe",
                "business_name": null,
                "phone_number": null,
                "street_address": "311 W Superior St",
                "extended_address": null,
                "locality": "Chicago",
                "administrative_area": "IL",
                "neighborhood": null,
                "borough": null,
                "postal_code": "60654",
                "country_code": "US",
                "address_book": false,
                "validate_address": false,
                "created_at": Utc::now().to_rfc3339(),
                "updated_at": Utc::now().to_rfc3339()
            }
        })
    }
}

#[tokio::test]
async fn create_address_sucess() {
    // Arrange
    let context = common::setup().await;

    let request = CreateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .locality("Chicago".to_string())
        .country_code("US".to_string())
        .administrative_area("IL".to_string())
        .postal_code("60654".to_string())
        .first_name("John".to_string())
        .last_name("Doe".to_string())
        .build();

    let expected_response = responses::address_response(123456);

    Mock::given(method("POST"))
        .and(path("/address"))
        .and(bearer_token("test-api-key"))
        .and(body_json(&request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&context.server)
        .await;

    // Act
    let result = context.client.addresses().create(request).await;

    // Assert
    assert!(result.is_ok());
    let address = result.unwrap().data;
    assert_eq!(address.id, 123456);
    assert_eq!(address.street_address, "311 W Superior St");
    assert_eq!(address.locality, "Chicago");
    assert_eq!(address.administrative_area, Some("IL".to_string()));
    assert_eq!(address.postal_code, Some("60654".to_string()));
    assert_eq!(address.country_code, "US");
    assert_eq!(address.first_name, Some("John".to_string()));
    assert_eq!(address.last_name, Some("Doe".to_string()));
}
