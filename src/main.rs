use neddle::Injectable;

fn main() {
    let ctx = Context::new();
    infer.ctx_call(&ctx);
    infer2.ctx_call(&ctx);
}

struct Num(u32);
struct Str(&'static str);
#[derive(Debug)]
struct Test(u32, f32);

fn infer(num: Num, str: Str, test: Test) {
    println!("{}: {}", num.0, str.0);
    println!("{:?}", test);
}

fn infer2(test: Test) {
    println!("{}, {}", test.0, test.1);
}

impl FromCtx for Test {
    fn from(_ctx: &Context) -> Self {
        Test(123, 321.31)
    }
}

impl FromCtx for Num {
    fn from(_ctx: &Context) -> Self {
        Num(0)
    }
}

impl FromCtx for Str {
    fn from(_ctx: &Context) -> Self {
        Str("Hello world")
    }
}
