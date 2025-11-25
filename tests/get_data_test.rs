mod mocks;
use mocks::mock_provider::MockProvider;
use wapp::providers::ApiProvider;

#[tokio::test]
async fn test_get_data_with_mock() {
    let mock = MockProvider {
        response: "DATA_OK".into(),
    };

    let out = mock.get_data("Kyiv".into(), "now".into()).await.unwrap();

    assert_eq!(out, "DATA_OK");
}
