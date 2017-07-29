//!Library providing one single function: convert a standard-libary SystemTime
//!into the much more user-friendly DateTime format used by the Chrono crate.
//!
//!For some reason chrono doesn't provide this conversion, 
//!even though it's handy for dealing with things like 
//!file metadata, etc. that are typed as SystemTime
//!
//!Though the implementation is trivially simple, this is provided
//!so that its users can perform the conversion
//!without first having to figure out how to work with SystemTime.
#![allow(unused_imports)]


extern crate chrono;
use std::time::{SystemTime,UNIX_EPOCH};
use chrono::{DateTime,TimeZone};

///converts a SystemTime to a DateTime, given a timezone
pub fn convert<T:TimeZone>(st:SystemTime,tz:T) -> DateTime<T>{
    let duration = st.duration_since(UNIX_EPOCH).unwrap().as_secs();
    return tz.timestamp(duration as i64,0);
}

#[cfg(test)]
mod tests {
    use super::convert;
    use std::time::{Duration,SystemTime,UNIX_EPOCH};
    use chrono::Utc;
    #[test]
    fn test() {
        let d = Duration::from_secs(1501301355);
        let t = UNIX_EPOCH + d;
        let dt = convert(t,Utc);
        assert_eq!("Sat, 29 Jul 2017 04:09:15 +0000",dt.to_rfc2822());
    }
}
