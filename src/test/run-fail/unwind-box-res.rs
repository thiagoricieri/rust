// error-pattern:fail

fn failfn() {
    fail;
}

struct r {
  v: *int,
  drop unsafe {
    let _v2: ~int = cast::reinterpret_cast(&self.v);
  }
}

fn r(v: *int) -> r {
    r {
        v: v
    }
}

fn main() unsafe {
    let i1 = ~0;
    let i1p = cast::reinterpret_cast(&i1);
    cast::forget(i1);
    let x = @r(i1p);
    failfn();
    log(error, x);
}