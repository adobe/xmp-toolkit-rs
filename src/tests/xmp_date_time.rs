// Copyright 2020 Adobe. All rights reserved.
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

use crate::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

#[test]
fn default() {
    let dt = XmpDateTime::default();
    assert!(dt.date.is_none());
    assert!(dt.time.is_none());

    let date = XmpDate::default();
    assert_eq!(date.year, 0);
    assert_eq!(date.month, 0);
    assert_eq!(date.day, 0);

    let time = XmpTime::default();
    assert_eq!(time.hour, 0);
    assert_eq!(time.minute, 0);
    assert_eq!(time.second, 0);
    assert_eq!(time.nanosecond, 0);
    assert!(time.time_zone.is_none());

    let tz = XmpTimeZone::default();
    assert_eq!(tz.hour, 0);
    assert_eq!(tz.minute, 0);
}

#[test]
fn current() {
    let dt = XmpDateTime::current().unwrap();

    let date = dt.date.as_ref().unwrap();
    assert!(date.year >= 2022);
    assert!(date.month >= 1);
    assert!(date.month <= 12);
    assert!(date.day >= 1);
    assert!(date.day <= 31);
}

mod set_local_time_zone {
    use crate::{XmpDate, XmpDateTime, XmpError, XmpErrorType, XmpTime, XmpTimeZone};

    #[test]
    fn no_existing_tz() {
        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        dt.set_local_time_zone().unwrap();

        // We don't know when writing this test what time zone will be used
        // when running this test. All we can do is verify that *some* time zone
        // was added and that other fields weren't altered. Print the result so
        // it can be manually inspected.

        println!("Manually verify correct local time zone: {:#?}", dt);

        assert_eq!(
            dt.date.unwrap(),
            XmpDate {
                year: 2022,
                month: 11,
                day: 5
            }
        );

        let time = dt.time.unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 40);
        assert_eq!(time.second, 35);
        assert_eq!(time.nanosecond, 42);
        assert!(time.time_zone.is_some());
    }

    #[test]
    fn error_existing_tz() {
        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 1, minute: 2 }),
            }),
        };

        assert_eq!(
            dt.set_local_time_zone().unwrap_err(),
            XmpError {
                error_type: XmpErrorType::BadParam,
                debug_message: "SetTimeZone can only be used on zone-less times".to_owned()
            }
        );
    }
}

mod convert_to_local_time {
    use crate::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn no_existing_tz() {
        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        dt.convert_to_local_time().unwrap();

        assert_eq!(
            dt.date.unwrap(),
            XmpDate {
                year: 2022,
                month: 11,
                day: 5
            }
        );

        let time = dt.time.unwrap();
        assert_eq!(time.hour, 14);
        assert_eq!(time.minute, 40);
        assert_eq!(time.second, 35);
        assert_eq!(time.nanosecond, 42);
        assert!(time.time_zone.is_none());
    }

    #[test]
    fn existing_tz() {
        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 1, minute: 2 }),
                // Use an unusual time zone so we can determine if
                // *something* changed.
            }),
        };

        dt.convert_to_local_time().unwrap();

        // Since we don't know when writing this test what time
        // zone will be in effect when running this test, we do some
        // basic sanity checks to ensure that *something* changed.

        println!("Updated date time = {:#?}", dt);

        assert_eq!(dt.date.unwrap().year, 2022);

        let time = dt.time.unwrap();

        assert_ne!(
            time,
            XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 1, minute: 2 }),
            }
        );

        assert_ne!(time.minute, 40);
    }
}

mod convert_to_utc {
    use crate::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn no_existing_tz() {
        let original_dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 14,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        let mut dt = original_dt.clone();
        dt.convert_to_utc().unwrap();

        assert_eq!(original_dt, dt);
    }

    #[test]
    fn existing_tz() {
        let mut dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 19,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        dt.convert_to_utc().unwrap();

        println!("Updated date time = {:#?}", dt);

        assert_eq!(
            dt,
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 11,
                    day: 6,
                }),
                time: Some(XmpTime {
                    hour: 2,
                    minute: 40,
                    second: 35,
                    nanosecond: 42,
                    time_zone: Some(XmpTimeZone { hour: 0, minute: 0 }),
                }),
            }
        );
    }

    #[test]
    fn already_utc() {
        let original_dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 11,
                day: 5,
            }),
            time: Some(XmpTime {
                hour: 19,
                minute: 40,
                second: 35,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 0, minute: 0 }),
            }),
        };

        let mut dt = original_dt.clone();

        dt.convert_to_utc().unwrap();

        println!("Updated date time = {:#?}", dt);

        assert_eq!(original_dt, dt);
    }
}

mod from_ffi {
    use crate::{ffi, XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn fully_populated_west_of_utc() {
        let dt = ffi::CXmpDateTime {
            year: 2022,
            month: 10,
            day: 19,
            hour: 18,
            minute: 9,
            second: 20,
            has_date: true,
            has_time: true,
            has_time_zone: true,
            tz_sign: -1,
            tz_hour: 7,
            tz_minute: 0,
            nanosecond: 42,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 10,
                    day: 19,
                }),
                time: Some(XmpTime {
                    hour: 18,
                    minute: 9,
                    second: 20,
                    nanosecond: 42,
                    time_zone: Some(XmpTimeZone {
                        hour: -7,
                        minute: 0,
                    }),
                })
            }
        );
    }

    #[test]
    fn fully_populated_east_of_utc() {
        let dt = ffi::CXmpDateTime {
            year: 2022,
            month: 10,
            day: 19,
            hour: 18,
            minute: 9,
            second: 20,
            has_date: true,
            has_time: true,
            has_time_zone: true,
            tz_sign: 1,
            tz_hour: 5,
            tz_minute: 30,
            nanosecond: 42,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 10,
                    day: 19,
                }),
                time: Some(XmpTime {
                    hour: 18,
                    minute: 9,
                    second: 20,
                    nanosecond: 42,
                    time_zone: Some(XmpTimeZone {
                        hour: 5,
                        minute: 30,
                    }),
                })
            }
        );
    }

    #[test]
    fn fully_populated_utc() {
        let dt = ffi::CXmpDateTime {
            year: 2022,
            month: 10,
            day: 19,
            hour: 18,
            minute: 9,
            second: 20,
            has_date: true,
            has_time: true,
            has_time_zone: true,
            tz_sign: 0,
            tz_hour: 0,
            tz_minute: 0,
            nanosecond: 42,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 10,
                    day: 19,
                }),
                time: Some(XmpTime {
                    hour: 18,
                    minute: 9,
                    second: 20,
                    nanosecond: 42,
                    time_zone: Some(XmpTimeZone { hour: 0, minute: 0 }),
                })
            }
        );
    }

    #[test]
    fn no_time_zone() {
        let dt = ffi::CXmpDateTime {
            year: 2022,
            month: 10,
            day: 19,
            hour: 18,
            minute: 9,
            second: 20,
            has_date: true,
            has_time: true,
            has_time_zone: false,
            tz_sign: 0,
            tz_hour: 0,
            tz_minute: 0,
            nanosecond: 42,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 10,
                    day: 19,
                }),
                time: Some(XmpTime {
                    hour: 18,
                    minute: 9,
                    second: 20,
                    nanosecond: 42,
                    time_zone: None,
                })
            }
        );
    }

    #[test]
    fn no_time() {
        let dt = ffi::CXmpDateTime {
            year: 2022,
            month: 10,
            day: 19,
            hour: 0,
            minute: 0,
            second: 0,
            has_date: true,
            has_time: false,
            has_time_zone: false,
            tz_sign: 0,
            tz_hour: 0,
            tz_minute: 0,
            nanosecond: 0,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: Some(XmpDate {
                    year: 2022,
                    month: 10,
                    day: 19,
                }),
                time: None
            }
        );
    }

    #[test]
    fn no_date() {
        let dt = ffi::CXmpDateTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 18,
            minute: 9,
            second: 20,
            has_date: false,
            has_time: true,
            has_time_zone: true,
            tz_sign: -1,
            tz_hour: 7,
            tz_minute: 0,
            nanosecond: 42,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: None,
                time: Some(XmpTime {
                    hour: 18,
                    minute: 9,
                    second: 20,
                    nanosecond: 42,
                    time_zone: Some(XmpTimeZone {
                        hour: -7,
                        minute: 0,
                    }),
                })
            }
        );
    }

    #[test]
    fn no_date_or_time() {
        let dt = ffi::CXmpDateTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            has_date: false,
            has_time: false,
            has_time_zone: false,
            tz_sign: 0,
            tz_hour: 0,
            tz_minute: 0,
            nanosecond: 0,
        };

        assert_eq!(
            XmpDateTime::from_ffi(&dt),
            XmpDateTime {
                date: None,
                time: None
            }
        );
    }
}

mod as_ffi {
    use crate::{ffi, XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn fully_populated_west_of_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 2022,
                month: 10,
                day: 19,
                hour: 18,
                minute: 9,
                second: 20,
                has_date: true,
                has_time: true,
                has_time_zone: true,
                tz_sign: -1,
                tz_hour: 7,
                tz_minute: 0,
                nanosecond: 42,
            }
        );
    }

    #[test]
    fn fully_populated_east_of_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: 5,
                    minute: 30,
                }),
            }),
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 2022,
                month: 10,
                day: 19,
                hour: 18,
                minute: 9,
                second: 20,
                has_date: true,
                has_time: true,
                has_time_zone: true,
                tz_sign: 1,
                tz_hour: 5,
                tz_minute: 30,
                nanosecond: 42,
            }
        );
    }

    #[test]
    fn fully_populated_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 0, minute: 0 }),
            }),
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 2022,
                month: 10,
                day: 19,
                hour: 18,
                minute: 9,
                second: 20,
                has_date: true,
                has_time: true,
                has_time_zone: true,
                tz_sign: 0,
                tz_hour: 0,
                tz_minute: 0,
                nanosecond: 42,
            }
        );
    }

    #[test]
    fn no_time_zone() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 2022,
                month: 10,
                day: 19,
                hour: 18,
                minute: 9,
                second: 20,
                has_date: true,
                has_time: true,
                has_time_zone: false,
                tz_sign: 0,
                tz_hour: 0,
                tz_minute: 0,
                nanosecond: 42,
            }
        );
    }

    #[test]
    fn no_time() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: None,
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 2022,
                month: 10,
                day: 19,
                hour: 0,
                minute: 0,
                second: 0,
                has_date: true,
                has_time: false,
                has_time_zone: false,
                tz_sign: 0,
                tz_hour: 0,
                tz_minute: 0,
                nanosecond: 0,
            }
        );
    }

    #[test]
    fn no_date() {
        let dt = XmpDateTime {
            date: None,
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 18,
                minute: 9,
                second: 20,
                has_date: false,
                has_time: true,
                has_time_zone: true,
                tz_sign: -1,
                tz_hour: 7,
                tz_minute: 0,
                nanosecond: 42,
            }
        );
    }

    #[test]
    fn no_date_or_time() {
        let dt = XmpDateTime {
            date: None,
            time: None,
        };

        assert_eq!(
            dt.as_ffi(),
            ffi::CXmpDateTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
                has_date: false,
                has_time: false,
                has_time_zone: false,
                tz_sign: 0,
                tz_hour: 0,
                tz_minute: 0,
                nanosecond: 0,
            }
        );
    }
}

mod fmt {
    use crate::{XmpDate, XmpDateTime, XmpTime, XmpTimeZone};

    #[test]
    fn fully_populated_west_of_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(format!("{}", dt), "2022-10-19T18:09:20.000000042-07:00");
    }

    #[test]
    fn fully_populated_east_of_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: 5,
                    minute: 30,
                }),
            }),
        };

        assert_eq!(format!("{}", dt), "2022-10-19T18:09:20.000000042+05:30");
    }

    #[test]
    fn fully_populated_utc() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone { hour: 0, minute: 0 }),
            }),
        };

        assert_eq!(format!("{}", dt), "2022-10-19T18:09:20.000000042Z");
    }

    #[test]
    fn no_time_zone() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: None,
            }),
        };

        assert_eq!(format!("{}", dt), "2022-10-19T18:09:20.000000042");
    }

    #[test]
    fn no_time() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 2022,
                month: 10,
                day: 19,
            }),
            time: None,
        };

        assert_eq!(format!("{}", dt), "2022-10-19");
    }

    #[test]
    fn no_time_year_after_10000() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 10203,
                month: 10,
                day: 19,
            }),
            time: None,
        };

        assert_eq!(format!("{}", dt), "10203-10-19");
    }

    #[test]
    fn no_time_year_before_1000() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: 981,
                month: 10,
                day: 19,
            }),
            time: None,
        };

        assert_eq!(format!("{}", dt), "0981-10-19");
    }

    #[test]
    fn no_time_year_before_0() {
        let dt = XmpDateTime {
            date: Some(XmpDate {
                year: -542,
                month: 10,
                day: 19,
            }),
            time: None,
        };

        assert_eq!(format!("{}", dt), "-0542-10-19");
    }

    #[test]
    fn no_date() {
        let dt = XmpDateTime {
            date: None,
            time: Some(XmpTime {
                hour: 18,
                minute: 9,
                second: 20,
                nanosecond: 42,
                time_zone: Some(XmpTimeZone {
                    hour: -7,
                    minute: 0,
                }),
            }),
        };

        assert_eq!(format!("{}", dt), "0000-01-01T18:09:20.000000042-07:00");
    }

    #[test]
    fn no_date_or_time() {
        let dt = XmpDateTime {
            date: None,
            time: None,
        };

        assert_eq!(format!("{}", dt), "0000");
    }
}
