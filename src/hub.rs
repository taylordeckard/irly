mod file_reader;
mod proto {
    tonic::include_proto!("irly.v1");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("irly_descriptor");
}

use proto::irly_server::Irly;

#[derive(Debug, Default)]
struct IrlyService {}

#[tonic::async_trait]
impl Irly for IrlyService {
    async fn get_file(
        &self,
        request: tonic::Request<proto::GetFileRequest>,
    ) -> Result<tonic::Response<proto::GetFileResponse>, tonic::Status> {
        let input = request.get_ref();

        if input.file_path.is_empty() {
            return Err(tonic::Status::invalid_argument("file_path is empty"));
        }

        let mut file_path = input.file_path.clone();
        if input.file_path.starts_with('/') {
            file_path.remove(0);
        }
        if &input.file_path == "/" {
            file_path = "index.html".to_string();
        }

        println!("Request for file: {:?}", &file_path);

        let file = file_reader::read(&file_path).await;

        if file.is_err() {
            return Err(tonic::Status::not_found("file not found"));
        }

        let response = proto::GetFileResponse {
            file_path: file_path.clone(),
            file_content: file.unwrap(),
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let irly = IrlyService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tonic::transport::Server::builder()
        .add_service(service)
        .add_service(proto::irly_server::IrlyServer::new(irly))
        .serve(addr)
        .await?;

    Ok(())
}
