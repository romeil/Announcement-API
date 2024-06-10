install:
	cargo add actix
	cargo add actix-web --features=openssl
	cargo add actix-web-httpauth
	cargo add bcrypt
	cargo add dotenv
	cargo add env_logger
	cargo add openssl
	cargo add serde --features=derive
	cargo add serde_json
	cargo add sqlx --features="runtime-async-std-native-tls postgres uuid"
	cargo add uuid --features ="v4 serde"