use std::thread;
use std::time::SystemTime;
fn main(){
    let start_t=SystemTime::now();

    //println!("{}",fac(40));
    let mut vec:Vec<thread::JoinHandle<()>>=Vec::new();
    //线程数量
    for i in 1..20{
        let th=thread::spawn(move||{run(i)});
        vec.push(th);
    }
    for i in vec{
        i.join().unwrap();
    }
    println!("{} ms",SystemTime::now().duration_since(start_t).unwrap().as_millis());
}

fn run(n:u64){
    println!("{} {} {}",n,40,fib(40));
}

fn fib(n:u64)->u64{
    if n<2{
        1
    }else{
        fib(n-1)+fib(n-2)
    }
}