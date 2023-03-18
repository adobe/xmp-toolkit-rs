// Copyright 2023 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

mod from_date_time {
    use std::convert::TryInto;

    use chrono::{DateTime, Datelike, FixedOffset, Timelike};

    use crate::{DateTimeConvertError, XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn happy_path() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2023,
                month: 3,
                day: 18,
            }),
            time: Some(XmpTime {
                hour: 11,
                minute: 20,
                second: 41,
                nanosecond: 123_456_789,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        let cdt: DateTime<FixedOffset> = dt.try_into().unwrap();

        let nd = cdt.date_naive();
        assert_eq!(nd.year(), 2023);
        assert_eq!(nd.month(), 3);
        assert_eq!(nd.day(), 18);

        let time = cdt.time();
        assert_eq!(time.hour(), 11);
        assert_eq!(time.minute(), 20);
        assert_eq!(time.second(), 41);
        assert_eq!(time.nanosecond(), 123_456_789);

        let tz = cdt.timezone();
        assert_eq!(tz.local_minus_utc(), -7 * 3600);
    }

    #[test]
    fn error_no_date() {
        let dt = XmpDateTime {
            date: None,
            time: Some(XmpTime {
                hour: 11,
                minute: 20,
                second: 41,
                nanosecond: 123_456_789,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(
            <XmpDateTime as TryInto<DateTime<FixedOffset>>>::try_into(dt).unwrap_err(),
            DateTimeConvertError::NoDate
        );
    }

    #[test]
    fn error_no_time() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2023,
                month: 3,
                day: 18,
            }),
            time: None,
        };

        assert_eq!(
            <XmpDateTime as TryInto<DateTime<FixedOffset>>>::try_into(dt).unwrap_err(),
            DateTimeConvertError::NoTime
        );
    }

    #[test]
    fn error_no_time_zone() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2023,
                month: 3,
                day: 18,
            }),
            time: Some(XmpTime {
                hour: 11,
                minute: 20,
                second: 41,
                nanosecond: 123_456_789,
                time_zone: None,
            }),
        };

        assert_eq!(
            <XmpDateTime as TryInto<DateTime<FixedOffset>>>::try_into(dt).unwrap_err(),
            DateTimeConvertError::NoTimeZone
        );
    }

    #[test]
    fn error_invalid_time() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2023,
                month: 3,
                day: 18,
            }),
            time: Some(XmpTime {
                hour: 27,
                minute: 20,
                second: 41,
                nanosecond: 123_456_789,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(
            <XmpDateTime as TryInto<DateTime<FixedOffset>>>::try_into(dt).unwrap_err(),
            DateTimeConvertError::InvalidTime
        );
    }

    #[test]
    fn error_invalid_time_zone() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2023,
                month: 3,
                day: 18,
            }),
            time: Some(XmpTime {
                hour: 1,
                minute: 20,
                second: 41,
                nanosecond: 123_456_789,
                time_zone: Some(XmpTimeZone {
                    hour: -25,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(
            <XmpDateTime as TryInto<DateTime<FixedOffset>>>::try_into(dt).unwrap_err(),
            DateTimeConvertError::InvalidTimeZone
        );
    }
}

mod to_date_time {
    use std::convert::Into;

    use chrono::{FixedOffset, NaiveDate};

    use crate::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn happy_path() {
        let dt = NaiveDate::from_ymd_opt(2023, 3, 18)
            .unwrap()
            .and_hms_nano_opt(13, 42, 21, 987_654_321)
            .unwrap()
            .and_local_timezone(FixedOffset::east_opt(-6 * 3600).unwrap())
            .unwrap();

        let dt: XmpDateTime = dt.into();

        assert_eq!(
            dt,
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2023,
                    month: 3,
                    day: 18,
                }),
                time: Some(XmpTime {
                    hour: 13,
                    minute: 42,
                    second: 21,
                    nanosecond: 987_654_321,
                    time_zone: Some(XmpTimeZone {
                        hour: -6,
                        minute: 0,
                    })
                }),
            }
        );
    }
}
