use std::fmt::Display;

use configinator::{error::ConfigError, Config};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

/// Class defined in configinator-java/lib/src/main/java/com/sonatype/configinator/exceptions/JNIException.java
const JNI_EXCEPTION_CLASS: &'static str = "com/sonatype/configinator/exceptions/JNIException";
/// Fallback Java exception class in case we can't instantiate our specific exception
const RUNTIME_EXCEPTION_CLASS: &'static str = "java/lang/RuntimeException";

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
                &format!("Could not process the config path as a string: {}", e),
            );
            JObject::null().into_inner() as jlong
        }
    }
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
                    &format!("Failed to create a string for the setup: {}", e),
                );
                JObject::null().into_inner()
            }
        }
    } else {
        JObject::null().into_inner()
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

fn throw_exception<D: Display + ?Sized>(env: &JNIEnv, message: &D) {
    let message = message.to_string();
    if let Err(_) = env.throw_new(JNI_EXCEPTION_CLASS, &message) {
        env.throw_new(RUNTIME_EXCEPTION_CLASS, &message)
            .expect(&format!(
                "Could not throw exception for message: {}",
                message
            ));
    }
}
