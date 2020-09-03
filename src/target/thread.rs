use super::registers::Registers;

pub trait Thread<Reg>
where
    Reg: Registers,
{
    type ThreadId;

    /// Return a thread name.
    fn name(&self) -> Result<Option<String>, Box<dyn std::error::Error>>;

    /// Return a thread ID.
    fn thread_id(&self) -> Self::ThreadId;

    /// Return CPU registers structure for this thread.
    fn read_regs(&self) -> Result<Reg, Box<dyn std::error::Error>>;

    /// Write CPU registers for this thread.
    fn write_regs(&self, registers: Reg) -> Result<(), Box<dyn std::error::Error>>;
}
