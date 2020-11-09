#[macro_use]
extern crate serde_json;

use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use log::{debug, error, info, trace};

#[allow(non_snake_case)]
pub mod android {
    use std::{thread, time};
    use std::ffi::{CString, CStr};

    use jni::JNIEnv;
    use jni::objects::{JClass, JString, JObject};
    use jni::sys::jstring;

    use crate::*;

    #[derive(Debug, Serialize)]
    struct JNIError {
        error: String,
        code: i32,
    }

    fn string_to_jstring(env: &JNIEnv, input: &str) -> Result<jstring, String> {
        let cstring = CString::new(input).map_err(|e| format!("{:?}", e))?;
        let cstr = cstring.to_str().map_err(|e| format!("{:?}", e))?;

        let output = env.new_string(cstr).map_err(|e| format!("{:?}", e))?;
        Ok(output.into_inner())
    }

    impl JNIError {
        fn into_string(self, env: &JNIEnv) -> jstring {
            let serialized = serde_json::to_string(&self)
                .unwrap_or("{\"error\": \"Can't serialize error\", \"code\": -1000}".to_string());
            string_to_jstring(env, &serialized).unwrap_or(JObject::null().into_inner())
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_org_notmandatory_echojni_Lib_echo(
        env: JNIEnv,
        _: JClass,
        incoming_string: JString,
    ) -> jstring {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Debug),
        );

        let incoming_cstr = match env.get_string(incoming_string) {
            Ok(string) => CStr::from_ptr(string.as_ptr()),
            Err(e) => {
                return JNIError {
                    error: format!("Invalid input string: {:?}", e),
                    code: -1001,
                }
                    .into_string(&env)
            }
        };

        let incoming_str = match incoming_cstr.to_str() {
            Ok(string) => string,
            Err(e) => {
                return JNIError {
                    error: format!("Invalid input string encoding: {:?}", e),
                    code: -1002,
                }
                    .into_string(&env)
            }
        };

        let echo_string = String::from(incoming_str);
        debug!("echo \"{}\"", &echo_string);
        thread::sleep(time::Duration::from_millis(10));

        string_to_jstring(&env, &echo_string).unwrap_or(JObject::null().into_inner())
    }
}
