use bollard::Docker;

use crate::helpers::enygmah_docker::{self, get_docker};

pub async fn run(docker_from_func: Option<&Docker>) {
    let docker: &Docker = match docker_from_func {
        Some(d) => d,
        None => &get_docker(),
    };

    // Ensure that the _outputs folder exists and is empty
    enygmah_docker::execute_command(&docker, String::from("rm -rf /home/enygmah/_outputs")).await;
    enygmah_docker::execute_command(&docker, String::from("mkdir /home/enygmah/_outputs")).await;
}
