use std::sync::{Condvar, Mutex};
pub struct FooBar {
    n: usize,
    mu: Mutex<bool>,
    cond: Condvar,
}

impl FooBar {
    pub fn new(n: usize) -> Self {
        FooBar {
            n,
            mu: Mutex::new(true),
            cond: Condvar::new(),
        }
    }

    pub fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut turn = self.mu.lock().unwrap();
            while !*turn {
                turn = self.cond.wait(turn).unwrap()
            }

            print_foo();
            *turn = false;
            self.cond.notify_one();
        }
    }

    pub fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut turn = self.mu.lock().unwrap();
            while *turn {
                turn = self.cond.wait(turn).unwrap();
            }

            print_bar();
            *turn = true;
            self.cond.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_multithreaded_foobar() {
        let n = 5; // Количество итераций
        let foobar = Arc::new(FooBar::new(n));

        let output = Arc::new(Mutex::new(String::new()));
        let output_clone_foo = Arc::clone(&output);
        let output_clone_bar = Arc::clone(&output);

        // Поток для foo
        let foo_thread = {
            let foobar = Arc::clone(&foobar);
            thread::spawn(move || {
                println!("старт foo_thread");
                foobar.foo(|| {
                    let mut out = output_clone_foo.lock().unwrap();
                    out.push_str("foo");
                });
            })
        };

        // Поток для bar
        let bar_thread = {
            let foobar = Arc::clone(&foobar);
            thread::spawn(move || {
                println!("старт bar_thread");
                foobar.bar(|| {
                    let mut out = output_clone_bar.lock().unwrap();
                    out.push_str("bar");
                });
            })
        };

        // Ждем завершения потоков
        foo_thread.join().unwrap();
        bar_thread.join().unwrap();

        // Проверяем результат
        let result = output.lock().unwrap();
        assert_eq!(result.as_str(), "foobar".repeat(n));
    }
}
