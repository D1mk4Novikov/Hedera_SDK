extern crate libc;


use std::ffi::CString;
use std::str;
use libc::c_char;


pub(crate) fn create_json_representation(match_key: i32, input_vector: Vec<&'static str>) -> *const c_char {
	let mut json_representation: String = "".to_string();
	match match_key {
		1 => {
			json_representation = format!(r#"
			{{
				"accountID": {
					"shard": "{}",
					"realm": "{}",
					"account": "{}" 
				},
				"contractAccountID": {},
				"deleted": "{}",
				"proxyAccountID": {
					"shard": "{}",
					"realm": "{}",
					"account": "{}" 
				},
				"proxyFraction": "{}",
				"proxyReceived": "{}",
				"key": "{}",
				"balance": "{}",
				"generateSendRecordThreshold": "{}",
				"generateReceiveRecordThreshold": "{}",
				"receiverSignatureRequired": "{}",
				"expirationTime": "{}",
				"autoRenewPeriod": "{}",
				"claims": "{}"
			}}
			"#, input_vector[0].to_string(), input_vector[1].to_string(), input_vector[2].to_string(), input_vector[3].to_string(), input_vector[4].to_string(), input_vector[5].to_string(), input_vector[6].to_string(), input_vector[7].to_string(),
				input_vector[8].to_string(), input_vector[9].to_string(), input_vector[10].to_string(), input_vector[11].to_string(),
				input_vector[12].to_string(), input_vector[13].to_string(), input_vector[14].to_string(), input_vector[15].to_string(),
				input_vector[16].to_string(), input_vector[17].to_string());
		},
		5 => {
			json_representation = format!(r#"
			{{
				"privateKey": "{}",
				"publicKey": "{}",
				"keywords": "{}"
			}}
			"#, input_vector[0].to_string(), input_vector[1].to_string(), input_vector[2].to_string());
		},
		_ => {},
	}
	CString::new(json_representation.as_bytes()).unwrap().into_raw()
} 