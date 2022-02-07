use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SynoContainerConfig {
    pub name: String,
    pub image: String,
    pub links: Vec<SynoContainerLink>,
    pub port_bindings: Vec<SynoContainerPortBinding>,
    pub volume_bindings: Vec<SynoContainerVolumeBinding>,
    pub env_variables: Vec<SynoContainerEnvVariable>
}

#[derive(Deserialize, Debug)]
pub struct SynoContainerLink {
    pub //alias: String,
    link_container: String,
}

#[derive(Deserialize, Debug)]
pub struct SynoContainerPortBinding {
    pub container_port: i32,
    pub host_port: i32,
    #[serde(rename = "type")]
    pub port_type: String,
}

#[derive(Deserialize, Debug)]
pub struct SynoContainerVolumeBinding {
    pub host_volume_file: String,
    pub mount_point: String,
    #[serde(rename = "type")]
    pub volume_type: String,
}

#[derive(Deserialize, Debug)]
pub struct SynoContainerEnvVariable {
    pub key: String,
    pub value: String,
}