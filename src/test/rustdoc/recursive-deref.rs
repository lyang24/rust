use std::ops::Deref;

// Cyclic deref with the parent (which is not the top parent).
pub struct A;
pub struct B;
pub struct C;

// @has recursive_deref/struct.A.html '//h3[@class="code-header in-band"]' 'impl Deref for A'
impl Deref for A {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.B.html '//h3[@class="code-header in-band"]' 'impl Deref for B'
impl Deref for B {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.C.html '//h3[@class="code-header in-band"]' 'impl Deref for C'
impl Deref for C {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// Cyclic deref with the grand-parent (which is not the top parent).
pub struct D;
pub struct E;
pub struct F;
pub struct G;

// @has recursive_deref/struct.D.html '//h3[@class="code-header in-band"]' 'impl Deref for D'
impl Deref for D {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.E.html '//h3[@class="code-header in-band"]' 'impl Deref for E'
impl Deref for E {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.F.html '//h3[@class="code-header in-band"]' 'impl Deref for F'
impl Deref for F {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.G.html '//h3[@class="code-header in-band"]' 'impl Deref for G'
impl Deref for G {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// Cyclic deref with top parent.
pub struct H;
pub struct I;

// @has recursive_deref/struct.H.html '//h3[@class="code-header in-band"]' 'impl Deref for H'
impl Deref for H {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}

// @has recursive_deref/struct.I.html '//h3[@class="code-header in-band"]' 'impl Deref for I'
impl Deref for I {
    type Target = H;

    fn deref(&self) -> &Self::Target {
        panic!()
    }
}
