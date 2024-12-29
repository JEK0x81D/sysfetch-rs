use std::fmt::{Display, Formatter};
use std::time::Duration;

static MINUTE_SECONDS: f64 = 60.;
static HOUR_SECONDS: f64 = 60. * 60.;
static DAY_SECONDS: f64 = 24. * 60. * 60.;

pub(crate) struct DisplayableDuration(pub Duration);

impl Display for DisplayableDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let seconds = (self.0.as_secs_f64() % MINUTE_SECONDS).floor();
        let minutes = ((self.0.as_secs_f64() / MINUTE_SECONDS) % (HOUR_SECONDS / MINUTE_SECONDS)).floor();
        let hours = ((self.0.as_secs_f64() / HOUR_SECONDS) % (DAY_SECONDS / HOUR_SECONDS)).floor();
        let days = (self.0.as_secs_f64() / DAY_SECONDS).floor();
        
        if days > 0. {
            write!(f, "{} days ", days.floor())?;
        }
        if hours > 0. {
            write!(f, "{} hours ", hours.floor())?;
        }
        if minutes > 0. {
            write!(f, "{} minutes ", minutes.floor())?;
        }
        if seconds > 0. {
            write!(f, "{} seconds", seconds.floor())?;
        }

        Ok(())
    }
}
