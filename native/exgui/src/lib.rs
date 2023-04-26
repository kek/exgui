use rustler::{Encoder, Env, Term};
use std::sync::RwLock;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
#[allow(unused_variables)]
fn hello(pid: rustler::types::LocalPid) -> () {
    std::thread::spawn(|| {
        println!("Hello from thread!");
    });
    <rustler::thread::ThreadSpawner as rustler::thread::JobSpawner>::spawn(move || {
        println!("Hello from job!");
        let mut msg_env = rustler::OwnedEnv::new();
        let data = "Hello world";
        msg_env.send_and_clear(&pid, |env| data.encode(env));
    });
}

fn load(env: Env, _term: Term) -> bool {
    rustler::resource!(TestResource, env);

    true
}

#[allow(dead_code)]
pub struct TestResource {
    test_field: RwLock<i32>,
}

#[rustler::nif]
fn make_resource() -> rustler::resource::ResourceArc<TestResource> {
    rustler::resource::ResourceArc::new(TestResource {
        test_field: RwLock::new(42),
    })
}

#[rustler::nif]
fn read_resource(resource: rustler::resource::ResourceArc<TestResource>) -> i32 {
    *resource.test_field.read().unwrap()
}

rustler::init!(
    "Elixir.Exgui",
    [add, hello, make_resource, read_resource],
    load = load
);
