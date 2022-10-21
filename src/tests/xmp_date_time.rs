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
