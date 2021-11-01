use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Deserializer};

use crate::error::ConfigError;
use crate::filesystem::locate_files;

/// A implementation of the configuration detailed on the [Lift configuration reference](https://help.sonatype.com/lift/configuration-reference) page.
#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Config {
    pub setup: Option<String>,

    pub build: Option<String>,

    #[serde(rename = "importantRules")]
    pub important_rules: Option<Vec<String>>,

    #[serde(rename = "ignoreRules")]
    pub ignore_rules: Option<Vec<String>>,

    #[serde(rename = "ignoreFiles", deserialize_with = "trim_whitespace", default)]
    pub ignore_files: Option<String>,

    pub tools: Option<Vec<String>>,

    #[serde(rename = "disableTools")]
    pub disable_tools: Option<Vec<String>>,

    #[serde(rename = "customTools")]
    pub custom_tools: Option<Vec<String>>,

    pub allow: Option<Vec<String>>,

    #[serde(rename = "jdk11")]
    pub jdk_11: Option<bool>,

    #[serde(rename = "androidVersion")]
    pub android_version: Option<u32>,

    #[serde(rename = "errorproneBugPatterns")]
    pub errorprone_bug_patterns: Option<Vec<String>>,

    #[serde(rename = "summaryComments")]
    pub summary_comments: Option<bool>,
}

fn trim_whitespace<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let with_whitespace: Option<String> = Option::deserialize(deserializer)?;

    Ok(with_whitespace.map(|with_whitespace| {
        itertools::join(
            with_whitespace
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty()),
            "\n",
        )
    }))
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if path.exists() && path.is_file() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            toml::from_str(&contents).map_err(Into::into)
        } else {
            Err(ConfigError::FileNotFound(path.to_path_buf()))
        }
    }

    pub fn from_folder<P: AsRef<Path>>(path: P) -> Result<Option<Self>, ConfigError> {
        let config_files = locate_files(&path)?;
        if let Some(config_file) = config_files.into_iter().next() {
            Ok(Some(Config::from_file(config_file)?))
        } else {
            Ok(None)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            setup: None,
            build: None,
            important_rules: None,
            ignore_rules: None,
            ignore_files: None,
            tools: None,
            disable_tools: None,
            custom_tools: None,
            allow: None,
            jdk_11: None,
            android_version: None,
            errorprone_bug_patterns: None,
            summary_comments: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_the_documentation_example() {
        let example = r#"
setup     = ".lift/script_that_downloads_deps.sh"
build     = "gradlew assemble"

# We only care about NULL_DEREFERENCE (from Infer)
# and no-extra-boolean-cast (from ESLint)
importantRules = ["NULL_DEREFERENCE", "no-extra-boolean-cast"]

# Ignore results from test and build directories
ignoreFiles = """
            build/
            src/test/
            """

# Only run infer and eslint (do not run errorprone, hlint, findsecbugs)
tools = [ "infer", "eslint" ]

# Only analyze and post responses to PRs from developers with these usernames
allow = [ "jill", "dave", "shawn" ]

jdk11 = false
"#;
        let expected = Config {
            setup: Some(String::from(".lift/script_that_downloads_deps.sh")),
            build: Some(String::from("gradlew assemble")),
            important_rules: Some(vec![
                String::from("NULL_DEREFERENCE"),
                String::from("no-extra-boolean-cast"),
            ]),
            ignore_files: Some(String::from(
                r#"build/
src/test/"#,
            )),
            tools: Some(vec![String::from("infer"), String::from("eslint")]),
            allow: Some(vec![
                String::from("jill"),
                String::from("dave"),
                String::from("shawn"),
            ]),
            jdk_11: Some(false),
            ..Default::default()
        };
        assert_eq!(toml::from_str(example), Ok(expected));
    }

    #[test]
    fn it_should_parse_a_file() {
        let expected = Config {
            setup: Some(String::from("echo 'Hello, Lift'")),
            allow: Some(vec![String::from("amy")]),
            summary_comments: Some(true),
            ..Default::default()
        };
        let actual =
            Config::from_file("examples/.lift.toml").expect("Failed to parse example toml");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_should_fail_to_parse_a_file_when_the_file_does_not_exist() {
        let error =
            Config::from_file("examples/does_not_exist.toml").expect_err("This should have failed");
        match error {
            ConfigError::FileNotFound(path) => assert_eq!(
                path.to_string_lossy().to_string(),
                String::from("examples/does_not_exist.toml")
            ),
            e => panic!("Expected a FileNotFound, but got {:?}", e),
        }
    }

    #[test]
    fn it_should_fail_to_parse_a_file_that_is_not_toml() {
        let error = Config::from_file("examples/.lift.json").expect_err("This should have failed");
        if let ConfigError::FileTomlParseFailed(_e) = error {
        } else {
            panic!("Expected a FileTomlParseFailed, but got {:?}", error);
        }
    }

    #[test]
    fn it_should_parse_a_folder() {
        let expected = Config {
            setup: Some(String::from("echo 'Hello, Lift'")),
            allow: Some(vec![String::from("amy")]),
            summary_comments: Some(true),
            ..Default::default()
        };
        let actual = Config::from_folder("examples/").expect("Failed to parse example toml");
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn it_should_not_fail_if_there_are_no_configs() {
        let actual =
            Config::from_folder("examples/no_configs/").expect("Failed to parse example toml");
        assert_eq!(actual, None);
    }
}
