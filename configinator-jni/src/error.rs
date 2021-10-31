use jni::JNIEnv;
use std::fmt::Display;
use thiserror::Error;

/// Class defined in configinator-java/lib/src/main/java/com/sonatype/configinator/exceptions/JNIException.java
const JNI_EXCEPTION_CLASS: &'static str = "com/sonatype/configinator/exceptions/JNIException";
/// Fallback Java exception class in case we can't instantiate our specific exception
const RUNTIME_EXCEPTION_CLASS: &'static str = "java/lang/RuntimeException";

pub(crate) fn throw_exception<D: Display + ?Sized>(env: &JNIEnv, message: &D) {
    let message = message.to_string();
    if let Err(e) = env.throw_new(JNI_EXCEPTION_CLASS, &message) {
        let message = format!("{}\n\nCreating custom exception failed:\n{:?}", message, e);
        env.throw_new(RUNTIME_EXCEPTION_CLASS, &message)
            .expect(&format!(
                "Could not throw exception for message:\n{}",
                message
            ));
    }
}

#[derive(Error, Debug)]
pub(crate) enum JniError {
    #[error("Failed to perform a JNI call into the JVM")]
    JniCallError(#[from] jni::errors::Error),
}
