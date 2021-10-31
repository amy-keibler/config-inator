use configinator::{error::ConfigError, Config};
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jlong, jobject, jstring};
use jni::JNIEnv;

mod error;
use error::{throw_exception, JniError};

const BOOLEAN_CLASS: &'static str = "java/lang/Boolean";
const BOOLEAN_CONSTRUCTOR: &'static str = "(Z)V";
const INTEGER_CLASS: &'static str = "java/lang/Integer";
const INTEGER_CONSTRUCTOR: &'static str = "(I)V";
const ARRAYLIST_CLASS: &'static str = "java/util/ArrayList";
const ARRAYLIST_CONSTRUCTOR: &'static str = "()V";

#[no_mangle]
pub extern "system" fn Java_com_sonatype_configinator_Config_loadConfig(
    env: JNIEnv,
    _class: JClass,
    config_path: JString,
) -> jlong {
    let config_path = env.get_string(config_path);
    match config_path {
        Ok(config_path) => {
            let config = Config::from_file::<String>(config_path.into());
            match config {
                Ok(config) => Box::into_raw(Box::new(config)) as jlong,
                Err(ConfigError::FileNotFound(_e)) => JObject::null().into_inner() as jlong,
                Err(e) => {
                    throw_exception(&env, &e);
                    JObject::null().into_inner() as jlong
                }
            }
        }
        Err(e) => {
            throw_exception(
                &env,
                &format!("Could not process the config path as a string:\n{}", e),
            );
            JObject::null().into_inner() as jlong
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_unloadConfig(
    _env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) {
    let _boxed_config = Box::from_raw(config_pointer as *mut Config);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetSetup(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jstring {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(setup) = &config.setup {
        let output = env.new_string(setup.clone());
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a string for the setup:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetBuild(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jstring {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(build) = &config.build {
        let output = env.new_string(build.clone());
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a string for the build:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetImportantRules(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(important_rules) = &config.important_rules {
        let output = create_list_of_strings(&env, important_rules);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a list of strings for the important_rules:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetIgnoreRules(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(ignore_rules) = &config.ignore_rules {
        let output = create_list_of_strings(&env, ignore_rules);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a list of strings for the ignore_rules:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetIgnoreFiles(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jstring {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(ignore_files) = &config.ignore_files {
        let output = env.new_string(ignore_files.clone());
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a string for the ignore_files:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetTools(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(tools) = &config.tools {
        let output = create_list_of_strings(&env, tools);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a list of strings for the tools:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetDisableTools(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(disable_tools) = &config.disable_tools {
        let output = create_list_of_strings(&env, disable_tools);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a list of strings for the disable_tools:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetCustomTools(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(custom_tools) = &config.custom_tools {
        let output = create_list_of_strings(&env, custom_tools);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a list of strings for the custom_tools:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetAllow(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(allow) = &config.allow {
        let output = create_list_of_strings(&env, allow);
        match output {
            Ok(output) => output,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a list of strings for the allow:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetJdk11(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(jdk_11) = &config.jdk_11 {
        let output = env.new_object(BOOLEAN_CLASS, BOOLEAN_CONSTRUCTOR, &[JValue::from(*jdk_11)]);
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!("Failed to create a boolean for the jdk_11:\n{}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetAndroidVersion(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(android_version) = &config.android_version {
        let output = env.new_object(
            INTEGER_CLASS,
            INTEGER_CONSTRUCTOR,
            &[JValue::from(*android_version as i32)],
        );
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create an integer for the android_version:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetErrorproneBugPatterns(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(errorprone_bug_patterns) = &config.errorprone_bug_patterns {
        let list = create_list_of_strings(&env, errorprone_bug_patterns);
        match list {
            Ok(list) => list,
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a list of strings for the errorprone_bug_patterns:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sonatype_configinator_Config_configGetSummaryComments(
    env: JNIEnv,
    _class: JClass,
    config_pointer: jlong,
) -> jobject {
    let config = &mut *(config_pointer as *mut Config);

    if let Some(summary_comments) = &config.summary_comments {
        let output = env.new_object(
            BOOLEAN_CLASS,
            BOOLEAN_CONSTRUCTOR,
            &[JValue::from(*summary_comments)],
        );
        match output {
            Ok(output) => output.into_inner(),
            Err(e) => {
                throw_exception(
                    &env,
                    &format!(
                        "Failed to create a boolean for the summary_comments:\n{}",
                        e
                    ),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
    }
}

fn create_list_of_strings(env: &JNIEnv, values: &Vec<String>) -> Result<jobject, JniError> {
    let output = env.new_object(ARRAYLIST_CLASS, ARRAYLIST_CONSTRUCTOR, &[])?;
    let list = env.get_list(output)?;
    for value in values {
        let value = env.new_string(value.clone())?;
        list.add(*value)?;
    }
    Ok(output.into_inner())
}
