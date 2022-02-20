
use math::round;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use plotters::prelude::*;

#[derive(Debug)]
struct Buffer {
    queued: i32,
    wip: i32,
    max_wip: i32,
    max_flow: i32,
    rng: ThreadRng
}

impl Buffer {
    fn new(max_wip:i32, max_flow: i32) -> Self {
        Buffer{queued: 0, wip: 0, max_wip, max_flow, rng: thread_rng()}
    }

    fn work(&mut self, u: f64) -> i32 {
        let mut u2 = round::half_up(u, 1).max(0.0);
        u2 = u2.min(self.max_wip as f64);
        self.wip += u2 as i32;

        let mut r = 0;
        if u > 0.0 {
            let dist = Uniform::new(0.0, f64::from(self.wip));
            r = round::half_up(self.rng.sample(dist), 1) as i32;
        }

        self.wip -= r;
        self.queued += r;

        let dist = Uniform::new(0.0, f64::from(self.max_flow));
        r = round::half_up(self.rng.sample(dist), 1) as i32;
        r = r.min(self.queued);
        self.queued -= r;

        self.queued
    }
}

struct Controller {
    kp: f64,
    ki: f64,
    i: i32
}

impl Controller {
    fn new(kp: f64, ki: f64) -> Self {
        Controller{kp, ki, i: 0}
    }

    fn work(&mut self, e: i32) -> f64 {
        self.i += e;
        self.kp * f64::from(e) + self.ki * f64::from(self.i)
    }
}

#[derive(Debug, Copy, Clone)]
struct Data {
    t: i32,
    r: i32,
    e: i32,
    u: f64,
    y: i32
}

fn close_loop(c: &mut Controller, p: &mut Buffer, tm: Option<i32>, set_point: &dyn Fn(i32) -> i32) -> Vec<Data> {
    let mut y = 0;

    let mut data: Vec<Data> = Vec::new();

    for t in 0..tm.unwrap_or(5000) {
        let r = set_point(t);
        let e = r - y;
        let u = c.work(e);
        y = p.work(u);

        data.push(Data{t:t, r:r, e:e, u:u, y:y});
        println!("{} {} {} {} {}", t, r, e, u, y);
    }

    data
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut c = Controller::new(1.25, 0.01);
    let mut p = Buffer::new(50, 10);

    fn set_point(t: i32) -> i32 {
        match t {
            0..=99 => 0,
            100..=300 => 50,
            _ => 10
        }
    }

    close_loop(&mut c, &mut p, Some(310), &set_point);
    let mut canvas = SVGBackend::new("output.svg", (800, 600)).into_drawing_area();
    canvas.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&canvas);

    Ok(())
}
