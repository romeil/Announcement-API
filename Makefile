install:
	cargo add actix
	cargo add actix-session --features="redis-rs-session"
	cargo add actix-web --features=openssl
	cargo add actix-web-httpauth
	cargo add bcrypt
	cargo add chrono
	cargo add dotenv
	cargo add env_logger
	cargo add futures_util
	cargo add hex
	cargo add lazy_static
	cargo add openssl
	cargo add pasetors
	cargo add serde --features=derive
	cargo add ring
	cargo add serde_json
	cargo add sqlx --features="runtime-async-std-native-tls postgres uuid"
	cargo add uuid --features ="v4 serde"