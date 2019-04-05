use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

pub fn run() {
    let desired_tps = 13.0;
    let rate_limiter = Rc::new(RateLimiter::new(desired_tps));
    let api1 = Api {
        rate_limiter: Rc::clone(&rate_limiter),
        id: 0,
    };

    let api2 = Api {
        rate_limiter: Rc::clone(&rate_limiter),
        id: 1,
    };

    println!("Calling APIs rate limited to {} calls/sec", desired_tps);

    let start = Instant::now();
    let mut results = Vec::new();
    for _ in 0..5 {
        results.push(api1.do_something_expensive());
        results.push(api2.do_something_expensive());
    }
    println!("Results: {:?}", results);
    let elapsed = start.elapsed().as_millis() as f64 / 1000.0;
    let items = results.len();
    let actual_tps = (items as f64) / elapsed;
    println!(
        "It took {} secs to complete {} calls, which is {} calls/sec",
        elapsed, items, actual_tps
    );
}

struct Api {
    rate_limiter: Rc<RateLimiter>,
    id: i32,
}

impl Api {
    fn do_something_expensive(&self) -> String {
        self.rate_limiter.get_permit();
        format!("API {} result", self.id)
    }
}

struct RateLimiter {
    tokens_per_second: f64,
    tokens: RefCell<f64>,
    max_tokens: f64,
    last_check: RefCell<Instant>,
}

impl RateLimiter {
    fn new(tps: f64) -> RateLimiter {
        RateLimiter {
            tokens_per_second: tps,
            tokens: RefCell::new(0.0),
            max_tokens: tps,
            last_check: RefCell::new(Instant::now()),
        }
    }

    /// Blocks until permit is available
    fn get_permit(&self) {
        self.block_for_token();
        self.use_token();
        self.incr_tokens();
    }

    fn use_token(&self) {
        *self.tokens.borrow_mut() -= 1.0;
    }

    fn block_for_token(&self) {
        let tokens = *self.tokens.borrow();
        if tokens >= 1.0 {
            return;
        } else {
            let block_time_secs = (1.0 - tokens) / self.tokens_per_second;
            let sleep_time = Duration::from_millis((block_time_secs * 1000.0) as u64);
            thread::sleep(sleep_time);
        }
    }

    fn incr_tokens(&self) {
        let tokens_to_add: f64 = (self.last_check.borrow().elapsed().as_millis() as f64 / 1000.0)
            * self.tokens_per_second;
        if tokens_to_add > 0.0 {
            let tokens = *self.tokens.borrow();
            *self.tokens.borrow_mut() = f64::min(tokens + tokens_to_add, self.max_tokens);
            *self.last_check.borrow_mut() = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::RateLimiter;
    use std::time::Instant;
    #[test]
    fn test_rate_limiter() {
        let rate_limiter = RateLimiter::new(1.0);
        let mut actions_completed = 0;
        let start = Instant::now();
        while start.elapsed().as_millis() <= 25 {
            rate_limiter.get_permit();
            actions_completed += 1
        }
        assert_eq!(1, actions_completed);
    }

    #[test]
    fn test_rate_limiter2() {
        let rate_limiter = RateLimiter::new(10.0);
        let mut actions_completed = 0;
        let start = Instant::now();
        while start.elapsed().as_millis() <= 1000 {
            rate_limiter.get_permit();
            actions_completed += 1
        }
        assert_eq!(10, actions_completed);
    }
}
