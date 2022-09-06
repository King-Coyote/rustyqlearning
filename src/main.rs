use std::env;
use clap::Parser;
use learner::*;
use environment::*;

mod learner;
mod environment;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long)]
    episodes: u32,
    #[clap(short, long)]
    limit_per_episode: Option<u32>,
}

fn main() {
    let args = Arguments::parse();
    let limit = args.limit_per_episode.unwrap_or(20);
    let mut env = Environment::new(3, 2, 0, 0);
    let mut learner = TableLearner::using_env(&env);

    for _ in 0..args.episodes {
        let mut n = 0;
        let cond = move |v: f32| {n += 1; n >= limit || v < -2.0 || v > 9.0};
        learn_until(cond, &mut learner, &env);
    }
    learner.print();

    println!("Beginning simulation...");
    let cond = move |v: f32| {v < -2.0 || v > 9.0};
    let effect = |a, v| ();
    simulate_until(cond, effect, &mut learner, &env);
}

fn learn_until<C, L>(mut cond: C, learner: &mut L, env: &Environment) 
where
    C: FnMut(f32) -> bool,
    L: Learner,
{
    let mut local_env = env.clone();
    loop {
        let q = learner.learn(&mut local_env);
        match q {
            Some(v) => {
                if cond(v) {
                    break;
                }
            },
            None => break
        }
    }
}

fn simulate_until<C, E, L>(mut cond: C, effect: E, learner: &mut L, env: &Environment)
where
    C: FnMut(f32) -> bool,
    E: Fn(Action, f32),
    L: Learner,
{
    let mut local_env = env.clone();
    loop {
        let r = learner.simulate(&mut local_env);
        match r {
            Some((a, v)) => {
                effect(a, v);
                if cond(v) {
                    break;
                }
            },
            None => break
        }
    }
}