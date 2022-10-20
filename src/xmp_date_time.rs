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

use crate::{ffi, XmpError, XmpResult};

/// The `XmpDateTime` struct allows easy conversion between ISO8601 format
/// (the "native" representation of dates and times in XMP) and other formats.
///
/// All of the fields are signed 32 bit integers, even though most could be 8
/// bit. (The same is true in the underlying C++ XMP Toolkit.) This avoids
/// overflow when doing carries for arithmetic or normalization.
///
/// `XmpDateTime` values are occasionally used with only a date or only a time
/// component. These possibilities are expressed using `Option`s.
///
/// Note that the [`DateTime` struct in the `chrono` crate](https://docs.rs/chrono/0.4.22/chrono/struct.DateTime.html)
/// is _similar_ to this struct. We chose not to use that in the
/// Rust XMP Toolkit in order to provide a more precise mapping
/// to the API provided by the underlying C++ XMP Toolkit.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct XmpDateTime {
    /// The date, if known.
    pub date: Option<XmpDate>,

    /// The time, if known.
    pub time: Option<XmpTime>,
}

/// The date portion of [`XmpDateTime`].
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct XmpDate {
    /// The year, can be negative.
    pub year: i32,

    /// The month in the range 1..12.
    pub month: i32,

    /// The day of the month in the range 1..31.
    pub day: i32,
}

/// The time portion of [`XmpDateTime`].
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct XmpTime {
    /// The hour in the range 0..23.
    pub hour: i32,

    /// The minute in the range 0..59.
    pub minute: i32,

    /// The second in the range 0..59.
    pub second: i32,

    /// Nanoseconds within a second, often left as zero.
    pub nanosecond: i32,

    /// The time zone, if known.
    pub time_zone: Option<XmpTimeZone>,
}

/// The time zone portion of [`XmpTime`].
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct XmpTimeZone {
    /// The time zone hour in the range -23..+23.
    /// Negative numbers are west of UTC; positive numbers are east.
    pub hour: i32,

    /// The time zone minute in the range 0..59.
    pub minute: i32,
}

impl XmpDateTime {
    /// Creates a new date-time struct reflecting the current time.
    pub fn current() -> XmpResult<Self> {
        let mut dt = ffi::CXmpDateTime::default();
        let mut err = ffi::CXmpError::default();

        unsafe { ffi::CXmpDateTimeCurrent(&mut dt, &mut err) };

        XmpError::raise_from_c(&err)?;

        Ok(Self::from_ffi(&dt))
    }

    pub(crate) fn from_ffi(dt: &ffi::CXmpDateTime) -> Self {
        Self {
            date: if dt.has_date {
                Some(XmpDate {
                    year: dt.year,
                    month: dt.month,
                    day: dt.day,
                })
            } else {
                None
            },
            time: if dt.has_time {
                Some(XmpTime {
                    hour: dt.hour,
                    minute: dt.minute,
                    second: dt.second,
                    nanosecond: dt.nanosecond,
                    time_zone: if dt.has_time_zone {
                        Some(XmpTimeZone {
                            hour: if dt.tz_sign < 0 {
                                -dt.tz_hour
                            } else {
                                dt.tz_hour
                            },
                            minute: dt.tz_minute,
                        })
                    } else {
                        None
                    },
                })
            } else {
                None
            },
        }
    }

    pub(crate) fn as_ffi(&self) -> ffi::CXmpDateTime {
        let mut result = ffi::CXmpDateTime::default();

        if let Some(date) = &self.date {
            result.has_date = true;
            result.year = date.year;
            result.month = date.month;
            result.day = date.day;
        }

        if let Some(time) = &self.time {
            result.has_time = true;
            result.hour = time.hour;
            result.minute = time.minute;
            result.second = time.second;
            result.nanosecond = time.nanosecond;

            if let Some(tz) = &time.time_zone {
                result.has_time_zone = true;
                match tz.hour {
                    h if h < 0 => {
                        result.tz_sign = -1;
                        result.tz_hour = -h;
                    }
                    0 if tz.minute == 0 => {
                        result.tz_sign = 0;
                        result.tz_hour = 0;
                    }
                    h => {
                        result.tz_sign = 1;
                        result.tz_hour = h;
                    }
                };
                result.tz_minute = tz.minute;
            }
        }

        result
    }
}
