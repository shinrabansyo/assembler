mod imem;
mod dmem;

pub fn assemble(program: &str) -> anyhow::Result<String> {
    imem::assemble(program) 
}
