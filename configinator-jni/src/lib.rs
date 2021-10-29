use configinator::{error::ConfigError, Config};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_sonatype_configinator_Config_loadConfig(
    env: JNIEnv,
    _class: JClass,
    config_path: JString,
) -> jlong {
    let config_path: String = env
        .get_string(config_path)
        .expect("Could not process the config path as a string")
        .into();
    let config = Config::from_file(config_path);
    match config {
        Ok(config) => Box::into_raw(Box::new(config)) as jlong,
        Err(ConfigError::FileNotFound(_e)) => JObject::null().into_inner() as jlong,
        Err(e) => panic!("Got unexpected error: {:?}", e),
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
        let output = env
            .new_string(setup.clone())
            .expect("Failed to create a string for the setup");
        output.into_inner()
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
