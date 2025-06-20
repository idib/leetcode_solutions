use std::sync::{Condvar, Mutex};

pub struct ZeroEvenOdd {
    n: usize,

    mu: Mutex<usize>,
    cond: Condvar,
}

impl ZeroEvenOdd {
    pub fn new(n: usize) -> Self {
        ZeroEvenOdd {
            n,
            mu: Mutex::new(0),
            cond: Condvar::new(),
        }
    }

    pub fn zero<F>(&self, printNumber: F)
    where
        F: Fn(usize),
    {
        for _ in 1..=self.n {
            let mut current_state = self
                .cond
                .wait_while(self.mu.lock().unwrap(), |x| *x != 0_usize)
                .unwrap();

            printNumber(0);
            *current_state = 1;
            self.cond.notify_all()
        }
    }

    pub fn even<F>(&self, printNumber: F)
    where
        F: Fn(usize),
    {
        for i in 1..=self.n {
            let mut current_state = self
                .cond
                .wait_while(self.mu.lock().unwrap(), |x| *x != 1_usize)
                .unwrap();
            if i % 2 == 0 {
                printNumber(i);
            }
            *current_state = 2;
            self.cond.notify_all()
        }
    }

    pub fn odd<F>(&self, printNumber: F)
    where
        F: Fn(usize),
    {
        for i in 1..=self.n {
            let mut current_state = self
                .cond
                .wait_while(self.mu.lock().unwrap(), |x| *x != 2_usize)
                .unwrap();

            if i % 2 == 1 {
                printNumber(i);
            }
            *current_state = 0;
            self.cond.notify_all()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn setup_multithreaded_test_ZeroEvenOdd(n: usize) -> String {
        let zero_even_odd = Arc::new(ZeroEvenOdd::new(n));

        let output = Arc::new(Mutex::new(String::new()));

        let zero_thread = {
            let zero_even_odd = Arc::clone(&zero_even_odd);
            let zero_output = Arc::clone(&output);
            thread::spawn(move || {
                println!("start zero_thread");
                zero_even_odd.zero(|x| {
                    let mut out = zero_output.lock().unwrap();
                    out.push_str(x.to_string().as_ref());
                });
            })
        };

        let even_thread = {
            let zero_even_odd = Arc::clone(&zero_even_odd);
            let even_output = Arc::clone(&output);
            thread::spawn(move || {
                println!("start even_thread");
                zero_even_odd.even(|x| {
                    let mut out = even_output.lock().unwrap();
                    out.push_str(x.to_string().as_ref());
                });
            })
        };

        let odd_thread = {
            let zero_even_odd = Arc::clone(&zero_even_odd);
            let odd_output = Arc::clone(&output);
            thread::spawn(move || {
                println!("start odd_thread");
                zero_even_odd.odd(|x| {
                    let mut out = odd_output.lock().unwrap();
                    out.push_str(x.to_string().as_ref());
                });
            })
        };

        zero_thread.join().unwrap();
        even_thread.join().unwrap();
        odd_thread.join().unwrap();

        let result = output.lock().unwrap();
        return result.clone();
    }

    #[test]
    fn test_multithreaded_ZeroEvenOdd_2() {
        let res = setup_multithreaded_test_ZeroEvenOdd(2);
        assert_eq!(res, "0102")
    }

    #[test]
    fn test_multithreaded_ZeroEvenOdd_5() {
        let res = setup_multithreaded_test_ZeroEvenOdd(5);
        assert_eq!(res, "0102030405")
    }

    #[test]
    fn test_multithreaded_ZeroEvenOdd_11() {
        let res = setup_multithreaded_test_ZeroEvenOdd(11);
        assert_eq!(res, "010203040506070809010011")
    }
}
