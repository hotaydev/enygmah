use std::process;

use bollard::Docker;
use log::debug;

use super::logger;

pub fn get_docker() -> Docker {
    return match Docker::connect_with_local_defaults() {
        Ok(res) => {
            debug!("Docker instance: {:#?}", res);
            res
        }
        Err(err) => {
            match err {
                bollard::errors::Error::SocketNotFoundError(_) => {
                    logger::create_log("It wasn't possible to connect to Docker Socket. Ensure that Docker is installed and running.", logger::EnygmahLogType::Error);
                }
                _ => {
                    logger::create_log(
                        &format!(
                            "An error occurred trying to connect to docker socket: {}",
                            err,
                        ),
                        logger::EnygmahLogType::Error,
                    );
                }
            }
            process::exit(1);
        }
    };
}
