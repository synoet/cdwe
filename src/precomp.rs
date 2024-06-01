use crate::config::EnvVariable;

struct PreComp {
    path_to_var: std::collections::HashMap<String, Vec<EnvVariable>>,
}
