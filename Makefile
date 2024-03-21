install:
	cargo add actix
	cargo add actix-web
	cargo add dotenv
	cargo add serde --features=derive
	cargo add serde_json
	cargo add sqlx --features="runtime-async-std-native-tls postgres uuid"
	cargo add uuid --features ="v4 serde"