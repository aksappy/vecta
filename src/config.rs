/// Configuration for Vecta runtime.
/// Checks the user's home directory for the vecta configuration file.
/// If the file is not found, it creates a default configuration file.
/// If the file is found, it reads the configuration file and returns a Config struct.
/// If the file is not found, it creates a default configuration file and returns a Config struct.
///
/// The configuration file is a TOML file, without a TOML extension.
///
///
use serde::{Deserialize, Serialize};
use toml::value::Array;

#[derive(Deserialize, Serialize)]
struct VectaConfiguration {
    main: IndexingConfiguration,
}

#[derive(Deserialize, Serialize)]
struct IndexingConfiguration {
    directory: String,
    exclusions: ExclusionConfiguration,
    inclusions: InclusionConfiguration,
}

#[derive(Deserialize, Serialize)]
struct ExclusionConfiguration {
    excluded_files: Array,
    excluded_directories: Array,
    excluded_extensions: Array,
}

#[derive(Deserialize, Serialize)]
struct InclusionConfiguration {
    included_files: Array,
    included_directories: Array,
    included_extensions: Array,
}

#[cfg(test)]
mod config_tests {

    use toml::Value;

    use crate::config::{
        ExclusionConfiguration, InclusionConfiguration, IndexingConfiguration, VectaConfiguration,
    };

    #[test]
    fn config_should_parse_correctly() {
        let config = VectaConfiguration {
            main: IndexingConfiguration {
                directory: "/path/to/directory".to_string(),
                exclusions: ExclusionConfiguration {
                    excluded_files: vec![],
                    excluded_directories: vec![Value::String(String::from("/path/to/exclusion"))],
                    excluded_extensions: vec![],
                },
                inclusions: InclusionConfiguration {
                    included_files: vec![],
                    included_directories: vec![],
                    included_extensions: vec![],
                },
            },
        };
        let config_string = toml::to_string(&config).unwrap();
        let result: VectaConfiguration = toml::from_str(config_string.as_str()).unwrap();
        assert_eq!(result.main.directory, "/path/to/directory");
        assert!(
            result
                .main
                .exclusions
                .excluded_directories
                .contains(&toml::Value::String("/path/to/exclusion".to_string()))
                == true,
        )
    }
}
