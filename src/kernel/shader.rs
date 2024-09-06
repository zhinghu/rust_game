pub struct Shader<F>
where
    F: Fn() + 'static,
{
    shader: FShader<F>,
}

impl<F> Shader<F>
where
    F: Fn() + 'static,
{
    pub fn new(shader: FShader<F>) -> Shader<F> {
        Shader { shader }
    }
}

pub struct FShader<F>
where
    F: Fn() + 'static,
{
    program: F,
}

impl<F> FShader<F>
where
    F: Fn() + 'static,
{
    pub fn new(program: F) -> FShader<F> {
        FShader { program }
    }
}
