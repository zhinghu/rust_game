pub struct Shader<T>
where
    T: Fn(),
{
    shader: FShader<T>,
}

impl<T> Shader<T>
where
    T: Fn(),
{
    pub fn new(shader: FShader<T>) -> Shader<T> {
        Shader { shader }
    }
}

pub struct FShader<T>
where
    T: Fn(),
{
    program: T,
}

impl<T> FShader<T>
where
    T: Fn(),
{
    pub fn new(program: T) -> FShader<T> {
        FShader { program }
    }
}
