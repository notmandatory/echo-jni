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

    impl ToString for JNIError {
        fn to_string(&self) -> String {
            serde_json::to_string(self)
                .unwrap_or("{\"error\": \"Can't serialize error\", \"code\": -1000}".to_string())
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_org_notmandatory_echojni_Lib_echo(
        env: JNIEnv,
        _: JClass,
        incoming_jstring: JString,
    ) -> jstring {
        android_logger::init_once(
            android_logger::Config::default().with_min_level(log::Level::Debug),
        );

        let incoming_string: String = match env.get_string(incoming_jstring) {
            Ok(string) => string.into(),
            Err(e) => {
                JNIError {
                    error: format!("Invalid input string: {:?}", e),
                    code: -1001,
                }.to_string()
            }
        };

        let echo_string = incoming_string.clone();
        debug!("echo \"{}\"", &echo_string);
        thread::sleep(time::Duration::from_millis(1));

        string_to_jstring(&env, &echo_string).unwrap_or(JObject::null().into_inner())
    }
}
