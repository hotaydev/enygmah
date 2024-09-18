use bollard::Docker;

use crate::helpers::enygmah_docker::{self, get_docker};

// TODO: get results before cleaning up the _outputs folder
pub async fn clean_up() {
    let docker: Docker = get_docker();
    enygmah_docker::execute_command(&docker, String::from("rm -rf /home/enygmah/_outputs/")).await;
}

pub async fn delete_created_folder(docker: &Docker, folder: &str) {
    enygmah_docker::execute_command(docker, format!("rm -rf {}", folder)).await;
}
