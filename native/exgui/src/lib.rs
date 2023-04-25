use rustler::Encoder;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
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

rustler::init!("Elixir.Exgui", [add, hello]);
