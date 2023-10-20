# gRPC Demo

This is a sample gRPC API demo. backend is made in rust and frontend is a dart cli app.

## Requirements

- Rust
- Dart
- Protoc

### 

1. Generate protos for dart,
   
   In root directory execute following command.
   
   ```protoc --dart_out=grpc:frontend/lib/generated -Iprotos protos/* ```
   
2. Run dart cli by executing following command in frontend directory.
   
   ```dart run```

## Run Server

1. Execute following command in `backend` folder.
   
   ```cargo run ```
