// SPDX-License-Identifier: MIT

/// Provides some extra convenience for Qt
pub fn suggest_needed_env_vars(template_contents: &str) {
    // create a string to string map, where key is env name and value is the message
    let env_vars = vec![
        (
            "QT_SDK_INSTALL",
            "the root of the Qt SDK, for example /home/user/Qt/",
        ),
        (
            "QT_INSTALL",
            "the specific Qt installed version, for example /opt/Qt/6.2.0/gcc_64",
        ),
    ];

    // iterate over the map and check if the env var exists
    for (varname, message) in env_vars {
        if template_contents.contains(varname) && std::env::var(varname).is_err() {
            println!(
                "Env variable {} isn't set! Should be set to {}",
                varname, message
            );
        }
    }
}
