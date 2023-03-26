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

use std::fmt;

use crate::{
    ffi::{self, CXmpString},
    XmpError, XmpResult,
};

/// Represents the concept of date and time as expressed in XMP.
///
/// XMP understands a close variant of the ISO8601 format, where date, time,
/// and time zone are all optional fields. These possibilities are expressed
/// using [`Option`]s.
///
/// All of the fields are signed 32 bit integers, even though most could be 8
/// bit. (The same is true in the underlying C++ XMP Toolkit.) This avoids
/// overflow when doing carries for arithmetic or normalization.
///
/// The [`DateTime` struct in the `chrono` crate](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)
/// is _similar_ to this struct, but does not provide a way to express
/// the optionality of date, time, and time zone in a single struct. For that
/// reason, we did not use it in this crate.
///
/// Enable crate feature `chrono` to enable conversions between this struct
/// and [`chrono::DateTime<FixedOffset>`](https://docs.rs/chrono/latest/chrono/struct.DateTime.html).
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
    /// The year, which can be negative.
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

    /// Sets the time zone to the local time zone.
    ///
    /// Can only be used when there is a time with no existing time zone
    /// (i.e. `self.time.time_zone.is_none()`). It is an error to call this
    /// function with an existing time zone.
    ///
    /// In that case, the time zone value is replaced with the local time zone.
    /// The other date/time fields are
    /// not adjusted in any way.
    pub fn set_local_time_zone(&mut self) -> XmpResult<()> {
        let mut dt = self.as_ffi();
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpDateTimeSetTimeZone(&mut dt, &mut err);
        }

        XmpError::raise_from_c(&err)?;

        self.update_from_ffi(&dt);
        Ok(())
    }

    /// Translate the value to the local time zone.
    ///
    /// If the time zone is not the local zone, the time is adjusted and the
    /// time zone set to be local. The value is not modified if the time zone is
    /// already the local zone or if the value has no time zone.
    pub fn convert_to_local_time(&mut self) -> XmpResult<()> {
        let mut dt = self.as_ffi();
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpDateTimeConvertToLocalTime(&mut dt, &mut err);
        }

        XmpError::raise_from_c(&err)?;

        self.update_from_ffi(&dt);
        Ok(())
    }

    /// Translates the value to UTC (Coordinated Universal Time).
    ///
    /// If the time zone is not UTC, the time is adjusted and the time zone set
    /// to be UTC. The value is not modified if the time zone is already UTC or
    /// if the value has no time zone.
    pub fn convert_to_utc(&mut self) -> XmpResult<()> {
        let mut dt = self.as_ffi();
        let mut err = ffi::CXmpError::default();

        unsafe {
            ffi::CXmpDateTimeConvertToUTCTime(&mut dt, &mut err);
        }

        XmpError::raise_from_c(&err)?;

        self.update_from_ffi(&dt);
        Ok(())
    }

    pub(crate) fn from_ffi(dt: &ffi::CXmpDateTime) -> Self {
        let mut result = Self::default();
        result.update_from_ffi(dt);
        result
    }

    pub(crate) fn update_from_ffi(&mut self, dt: &ffi::CXmpDateTime) {
        self.date = if dt.has_date {
            Some(XmpDate {
                year: dt.year,
                month: dt.month,
                day: dt.day,
            })
        } else {
            None
        };

        self.time = if dt.has_time {
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
        };
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

impl fmt::Display for XmpDateTime {
    /// Formats a date according to the ISO 8601 profile in <https://www.w3.org/TR/NOTE-datetime>.
    ///
    /// The format will be one of the following:
    ///
    /// * `YYYY`
    /// * `YYYY-MM-DD`
    /// * `YYYY-MM-DDThh:mmTZD`
    /// * `YYYY-MM-DDThh:mm:ssTZD`
    /// * `YYYY-MM-DDThh:mm:ss.sTZD`
    ///
    /// Where:
    ///
    /// * `YYYY` = four-digit year
    /// * `MM` = two-digit month (01=January)
    /// * `DD` = two-digit day of month (01 through 31)
    /// * `hh` = two digits of hour (00 through 23)
    /// * `mm` = two digits of minute (00 through 59)
    /// * `ss` = two digits of second (00 through 59)
    /// * `s` = one or more digits representing a decimal fraction of a second
    /// * `TZD` = time zone designator (`Z` or `+hh:mm` or `-hh:mm`)
    ///
    /// XMP allows time-only values (`date` = `None`). In this case, the date
    /// portion of the output will be `0000-01-01`.
    ///
    /// **NOTE:** ISO 8601 does not allow years less than 1000 or greater than
    /// 9999. `XmpDateTime` allows any year, even negative ones. The W3C
    /// profile also requires a time zone designator if a time is present;
    /// since `XmpDateTime` has an explicit notion of zone-less time, the
    /// `TZD` will not appear in that case.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err = ffi::CXmpError::default();

        unsafe {
            match CXmpString::from_ptr(ffi::CXmpDateTimeToString(&self.as_ffi(), &mut err))
                .map(|s| s)
            {
                Some(s) => {
                    write!(f, "{}", s)
                }
                None => {
                    let err = XmpError::raise_from_c(&err);
                    write!(f, "(unable to format date: {:#?})", err)
                }
            }
        }
    }
}

#[cfg(feature = "chrono")]
use std::convert::TryFrom;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, FixedOffset, LocalResult, NaiveDate, Timelike};
#[cfg(feature = "chrono")]
use thiserror::Error;

#[cfg(feature = "chrono")]
impl TryFrom<XmpDateTime> for DateTime<FixedOffset> {
    type Error = DateTimeConvertError;

    fn try_from(dt: XmpDateTime) -> Result<Self, Self::Error> {
        Self::try_from(&dt)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<&XmpDateTime> for DateTime<FixedOffset> {
    type Error = DateTimeConvertError;

    fn try_from(dt: &XmpDateTime) -> Result<Self, Self::Error> {
        let date = dt.date.as_ref().ok_or(DateTimeConvertError::NoDate)?;
        let time = dt.time.as_ref().ok_or(DateTimeConvertError::NoTime)?;
        let tz = time
            .time_zone
            .as_ref()
            .ok_or(DateTimeConvertError::NoTimeZone)?;

        let offset = FixedOffset::east_opt(tz.hour * 3600 + tz.minute * 60)
            .ok_or(DateTimeConvertError::InvalidTimeZone)?;

        match NaiveDate::from_ymd_opt(date.year, date.month as u32, date.day as u32)
            .ok_or(DateTimeConvertError::InvalidDate)?
            .and_hms_nano_opt(
                time.hour as u32,
                time.minute as u32,
                time.second as u32,
                time.nanosecond as u32,
            )
            .ok_or(DateTimeConvertError::InvalidTime)?
            .and_local_timezone(offset)
        {
            LocalResult::Single(t) => Ok(t),
            _ => Err(DateTimeConvertError::InvalidTimeZone),
        }
    }
}

/// Represents various reasons why an [`XmpDateTime`]
/// can not be converted to a [`DateTime`].
///
/// [`DateTime`]: chrono::DateTime
#[cfg(feature = "chrono")]
#[derive(Debug, Eq, Error, PartialEq)]
pub enum DateTimeConvertError {
    /// The [`XmpDateTime`] struct's `date` value is `None`.
    #[error("the date value is None")]
    NoDate,

    /// The [`XmpDateTime`] struct's `time` value is `None`.
    #[error("the time value is None")]
    NoTime,

    /// The [`XmpDateTime`] struct's `time.time_zone` value is `None`.
    #[error("the time.time_zone value is None")]
    NoTimeZone,

    /// The [`XmpDateTime`] struct's `date` value is out of bounds.
    #[error("the date value is out of bounds")]
    InvalidDate,

    /// The [`XmpDateTime`] struct's `time` value is out of bounds.
    #[error("the time value is out of bounds")]
    InvalidTime,

    /// The [`XmpDateTime`] struct's `time.time_zone` value is out of bounds.
    #[error("the time.time_zone value is out of bounds")]
    InvalidTimeZone,
}

#[cfg(feature = "chrono")]
impl From<DateTime<FixedOffset>> for XmpDateTime {
    fn from(dt: DateTime<FixedOffset>) -> Self {
        Self::from(&dt)
    }
}

#[cfg(feature = "chrono")]
impl From<&DateTime<FixedOffset>> for XmpDateTime {
    fn from(dt: &DateTime<FixedOffset>) -> Self {
        let nd = dt.date_naive();
        let date = XmpDate {
            year: nd.year(),
            month: nd.month() as i32,
            day: nd.day() as i32,
        };

        let tz = dt.timezone().local_minus_utc();
        let tz = XmpTimeZone {
            hour: tz / 3600,
            minute: (tz.abs() % 3600) / 60,
        };

        let nt = dt.time();
        let time = XmpTime {
            hour: nt.hour() as i32,
            minute: nt.minute() as i32,
            second: nt.second() as i32,
            nanosecond: nt.nanosecond() as i32,
            time_zone: Some(tz),
        };

        Self {
            date: Some(date),
            time: Some(time),
        }
    }
}
